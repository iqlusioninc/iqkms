//! iqkms Ethereum RPC service.

use crate::Error;
use proto::ethereum::{signer_server::Signer, SignDigestRequest, SignEip155Request, Signature};
use signing::{signature::ecdsa::secp256k1, VerifyingKey};
use tonic::{Request, Response, Status};
use tower::{Service, ServiceExt};
use tracing::trace;
use types::{ethereum::Address, BoxError, Bytes};

/// Signer gRPC service.
pub struct SignerService<S> {
    /// Reference to the signer service.
    signing_service: S,
}

impl<S> SignerService<S>
where
    S: Service<signing::Request, Response = signing::Response, Error = BoxError>
        + Clone
        + Send
        + Sync
        + 'static,
    S::Future: Send,
{
    /// Create a new RPC service with the given keyring.
    pub fn new(signing_service: S) -> Self {
        Self { signing_service }
    }

    /// Parse the given string as an Ethereum address and look up the
    /// corresponding key in the keyring.
    async fn sign_digest(&self, address: Address, digest: Bytes) -> Result<Signature, Error> {
        let request = signing::Request::SignPrehash {
            key_handle: address.into(),
            prehash: digest.clone(),
        };

        let (signature, verifying_key) = match self.call_service(request).await {
            Ok(signing::Response::SignPrehash {
                signature,
                verifying_key: VerifyingKey::EcdsaSecp256k1(verifying_key),
            }) => (signature, verifying_key),
            other => todo!("handle bad response: {:?}", other),
        };

        // TODO(tarcieri): less janky signature recovery API
        let digest = <[u8; 32]>::try_from(digest.as_ref()).map_err(|_| Error::DigestMalformed)?;
        let signature = secp256k1::RecoverableSignature::from_digest_bytes_trial_recovery(
            &verifying_key,
            &digest.into(),
            &secp256k1::Signature::try_from(signature.as_ref())?,
        )?;

        let r = signature.r().to_bytes().to_vec();
        let s = signature.s().to_bytes().to_vec();
        let v = u8::from(signature.recovery_id()) + 27;

        Ok(Signature { r, s, v: v.into() })
    }

    /// Make a request to the signing service.
    async fn call_service(&self, req: signing::Request) -> signing::Result<signing::Response> {
        self.signing_service
            .clone()
            .ready()
            .await
            .expect("signing service not ready") // TODO(tarcieri): handle this?
            .call(req)
            .await
            .map_err(Into::into)
    }
}

#[tonic::async_trait]
impl<S> Signer for SignerService<S>
where
    S: Service<signing::Request, Response = signing::Response, Error = BoxError>
        + Clone
        + Send
        + Sync
        + 'static,
    S::Future: Send,
{
    async fn sign_digest(
        &self,
        request: Request<SignDigestRequest>,
    ) -> Result<Response<Signature>, Status> {
        trace!("sign_digest[{:?}]: {:?}", request.remote_addr(), request);

        let request = request.into_inner();
        let address = request.address.parse::<Address>().map_err(Error::from)?;

        Ok(self
            .sign_digest(address, request.digest.into())
            .await
            .map(Response::new)
            .map_err(Error::from)?)
    }

    async fn sign_eip155(
        &self,
        request: Request<SignEip155Request>,
    ) -> Result<Response<Signature>, Status> {
        trace!("sign_eip155[{:?}]: {:?}", request.remote_addr(), request);

        let request = request.into_inner();
        let address = request.address.parse::<Address>().map_err(Error::from)?;

        let mut signature = self
            .sign_digest(address, request.digest.into())
            .await
            .map_err(Error::from)?;

        // Apply EIP-155
        signature.v = (request.chain_id * 2 + 35) + ((signature.v - 1) % 2);
        Ok(Response::new(signature))
    }
}
