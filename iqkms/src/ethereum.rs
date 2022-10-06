//! iqkms Ethereum support

pub use types::{
    crypto::digest::{sha3::Keccak256, Digest},
    ethereum::{Address, ChainId, H256},
};

use crate::{Error, StdError};
use proto::ethereum::{SignDigestRequest, SignEip155Request, Signature};
use tonic::{transport, Request};

/// Tonic-generated inner gRPC client.
type SignerClientInner = proto::ethereum::signer_client::SignerClient<transport::Channel>;

/// Ethereum transaction signer.
pub struct SignerClient {
    inner: SignerClientInner,
}

impl SignerClient {
    /// Attempt to create a new client by connecting to a given endpoint.
    pub async fn connect<D>(dst: D) -> Result<Self, transport::Error>
    where
        D: TryInto<transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        SignerClientInner::connect(dst).await.map(Into::into)
    }

    /// Sign the given digest using the private key with the given address.
    pub async fn sign_digest(
        &mut self,
        address: Address,
        digest: H256,
    ) -> Result<Signature, Error> {
        let request = SignDigestRequest {
            address: address.to_string(),
            digest: digest.as_ref().to_vec(),
        };

        let response = self.inner.sign_digest(Request::new(request)).await?;
        Ok(response.into_inner())
    }

    /// Sign the given digest using the private key with the given address,
    /// applying EIP-155 to the resulting signature to encode the provided
    /// chain ID.
    pub async fn sign_digest_with_eip155(
        &mut self,
        address: Address,
        digest: H256,
        chain_id: ChainId,
    ) -> Result<Signature, Error> {
        let request = SignEip155Request {
            address: address.to_string(),
            digest: digest.as_ref().to_vec(),
            chain_id,
        };

        let response = self.inner.sign_eip155(Request::new(request)).await?;
        Ok(response.into_inner())
    }

    /// Hash the given message with [`Keccak256`] and sign the resulting digest
    /// with EIP-155.
    ///
    /// See [`SignerClient::sign_digest_with_eip155`] for more information.
    pub async fn sign_message_with_eip155(
        &mut self,
        address: Address,
        msg: &[u8],
        chain_id: ChainId,
    ) -> Result<Signature, Error> {
        let digest = H256::from(Keccak256::digest(msg).as_ref());
        self.sign_digest_with_eip155(address, digest, chain_id)
            .await
    }
}

impl From<SignerClientInner> for SignerClient {
    fn from(inner: SignerClientInner) -> SignerClient {
        SignerClient { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, SignerClient, StdError, H256};

    #[tokio::test]
    async fn it_works() -> Result<(), StdError> {
        let mut client = SignerClient::connect("http://[::1]:27100").await?;

        // TODO(tarcieri): real signing key address
        let address = "0x27b1fdb04752bbc536007a920d24acb045561c26"
            .parse::<Address>()
            .unwrap();

        let chain_id = 2018;
        let digest = H256::from([
            0x6f, 0xd4, 0x3e, 0x7c, 0xff, 0xc3, 0x1b, 0xb5, 0x81, 0xd7, 0x42, 0x1c, 0x86, 0x98,
            0xe2, 0x9a, 0xa2, 0xbd, 0x8e, 0x71, 0x86, 0xa3, 0x94, 0xb8, 0x52, 0x99, 0x90, 0x8b,
            0x4e, 0xb9, 0xb1, 0x75,
        ]);

        let response = client
            .sign_digest_with_eip155(address, digest, chain_id)
            .await
            .unwrap();
        println!("RESPONSE={:?}", response);
        Ok(())
    }
}
