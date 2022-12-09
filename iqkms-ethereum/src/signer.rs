//! iqkms Ethereum RPC service.

use crate::Error;
use proto::ethereum::{signer_server::Signer, SignDigestRequest, SignEip155Request, Signature};
use signing::{signature::ecdsa::secp256k1, Keyring, VerifyingKey};
use tonic::{Request, Response, Status};
use tracing::trace;
use types::ethereum::Address;

/// Keccak256 digest.
type H256 = [u8; 32];

/// Signer gRPC service.
pub struct SignerService {
    /// Signing keyring.
    keyring: Keyring,
}

impl SignerService {
    /// Create a new RPC service with the given keyring.
    pub fn new(keyring: Keyring) -> Self {
        Self { keyring }
    }

    /// Parse the given string as an Ethereum address and look up the
    /// corresponding key in the keyring.
    fn sign_digest(&self, addr: &str, digest: &[u8]) -> Result<Signature, Error> {
        let addr = addr.parse::<Address>().map_err(Error::from)?;

        let signing_key = self
            .keyring
            .find_by_eth_address(&addr)
            .map_err(Error::from)?;

        let verifying_key = match signing_key.verifying_key() {
            VerifyingKey::EcdsaSecp256k1(vk) => vk,
            #[allow(unreachable_patterns)]
            _ => {
                return Err(Error::SigningKeyNotFound {
                    addr: addr.to_string(),
                })
            }
        };

        let digest = H256::try_from(digest).map_err(|_| Error::DigestMalformed)?;
        let raw_signature =
            secp256k1::Signature::try_from(signing_key.sign_prehash(&digest)?.as_ref())?;

        let signature = secp256k1::RecoverableSignature::from_digest_bytes_trial_recovery(
            &verifying_key,
            &digest.into(),
            &raw_signature,
        )?;

        let r = signature.r().to_bytes().to_vec();
        let s = signature.s().to_bytes().to_vec();
        let v = u8::from(signature.recovery_id()) + 27;

        Ok(Signature { r, s, v: v.into() })
    }
}

#[tonic::async_trait]
impl Signer for SignerService {
    async fn sign_digest(
        &self,
        request: Request<SignDigestRequest>,
    ) -> Result<Response<Signature>, Status> {
        trace!("sign_digest[{:?}]: {:?}", request.remote_addr(), request);

        let request = request.into_inner();
        Ok(self
            .sign_digest(&request.address, &request.digest)
            .map(Response::new)
            .map_err(Error::from)?)
    }

    async fn sign_eip155(
        &self,
        request: Request<SignEip155Request>,
    ) -> Result<Response<Signature>, Status> {
        trace!("sign_eip155[{:?}]: {:?}", request.remote_addr(), request);

        let request = request.into_inner();
        let mut signature = self
            .sign_digest(&request.address, &request.digest)
            .map_err(Error::from)?;

        // Apply EIP-155
        signature.v = (request.chain_id * 2 + 35) + ((signature.v - 1) % 2);
        Ok(Response::new(signature))
    }
}
