//! iqkms ethers-rs signing provider.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::integer_arithmetic,
    clippy::panic,
    // clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

pub use ethers_core::types;

use ethers_core::types::{
    transaction::{eip2718::TypedTransaction, eip712::Eip712},
    Signature,
};
use ethers_signers::Signer;
use iqkms::{
    proto,
    tokio::sync::Mutex,
    tonic,
    types::{
        crypto::digest::{sha3::Keccak256, Digest},
        ethereum::{Address, ChainId, H160, H256, U256},
    },
    Error, StdError,
};
use std::{fmt, sync::Arc};
use tracing::instrument;

/// iqkms ethers-rs signing provider.
pub struct IqkmsSigner {
    /// gRPC client
    client: Arc<Mutex<iqkms::ethereum::SignerClient>>,

    /// Ethereum address of the signing key.
    address: Address,

    /// EIP-55 chain ID.
    chain_id: ChainId,
}

impl IqkmsSigner {
    /// Create a new signer, connecting to the given endpoint.
    pub async fn connect<D>(dst: D, address: Address, chain_id: ChainId) -> iqkms::Result<Self>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let client = iqkms::ethereum::SignerClient::connect(dst).await?;
        Self::new(client, address, chain_id).await
    }

    /// Create a new signer from the given `iqkms` gRPC client.
    #[instrument(err, skip(client))]
    pub async fn new(
        client: iqkms::ethereum::SignerClient,
        address: Address,
        chain_id: ChainId,
    ) -> iqkms::Result<Self> {
        if let Some(id) = address.chain_id {
            // TODO(tarcieri): real error handling
            assert_eq!(id, chain_id);
        }

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            address,
            chain_id,
        })
    }

    /// Sign a digest with this signer's key and add the eip155 `v` value
    /// corresponding to the `chain_id` this signer was initialized with.
    #[instrument(err)]
    async fn sign_digest_with_eip155(&self, digest: H256) -> iqkms::Result<Signature> {
        let mut client = self.client.lock().await;
        let signature = client
            .sign_digest_with_eip155(self.address, digest, self.chain_id)
            .await?;

        parse_signature_proto(signature)
    }
}

fn parse_signature_proto(signature: proto::ethereum::Signature) -> iqkms::Result<Signature> {
    if signature.r.len() == 32 && signature.s.len() == 32 {
        Ok(Signature {
            r: U256::from_big_endian(&signature.r),
            s: U256::from_big_endian(&signature.s),
            v: signature.v,
        })
    } else {
        todo!()
    }
}

#[async_trait::async_trait]
impl Signer for IqkmsSigner {
    type Error = Error;

    #[instrument(err, skip(msg))]
    async fn sign_message<S>(&self, msg: S) -> iqkms::Result<Signature>
    where
        S: AsRef<[u8]> + Send + Sync,
    {
        let digest = H256::from(Keccak256::digest(msg).as_ref());
        self.sign_digest_with_eip155(digest).await
    }

    #[instrument(err)]
    async fn sign_transaction(&self, _tx: &TypedTransaction) -> iqkms::Result<Signature> {
        todo!()
    }

    async fn sign_typed_data<T>(&self, payload: &T) -> iqkms::Result<Signature>
    where
        T: Eip712 + Send + Sync,
    {
        // TODO(tarcieri): don't panic
        let digest = payload.encode_eip712().expect("EIP-712 encoding error");
        let mut client = self.client.lock().await;
        let signature = client.sign_digest(self.address, H256::from(digest)).await?;
        parse_signature_proto(signature)
    }

    fn address(&self) -> H160 {
        self.address.into()
    }

    /// Returns the signer's chain id
    fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    /// Sets the signer's chain id
    fn with_chain_id<T: Into<ChainId>>(mut self, chain_id: T) -> Self {
        self.chain_id = chain_id.into();
        self
    }
}

impl fmt::Debug for IqkmsSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IqkmsSigner")
            .field("address", &self.address)
            .field("chain_id", &self.chain_id)
            .finish()
    }
}
