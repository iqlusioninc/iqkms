use crate::{Error, Result, VerifyingKey, keyring::Keyring};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;
use types::Bytes;

#[cfg(feature = "ethereum")]
use types::ethereum;

/// Tower service which controls access to the signing keyring.
#[derive(Debug, Default)]
pub struct SigningService {
    /// Signing keyring.
    keyring: Keyring,
}

impl SigningService {
    /// Create new [`SigningService`] with an empty keyring.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sign the given prehash using the key with the given handle.
    fn sign_prehash(&self, key_handle: KeyHandle, prehash: &[u8]) -> Result<Response> {
        let signing_key = match key_handle {
            #[cfg(feature = "ethereum")]
            KeyHandle::Ethereum(eth_addr) => self.keyring.find_by_eth_address(&eth_addr)?,
            #[allow(unreachable_patterns)]
            _ => return Err(Error),
        };

        let verifying_key = signing_key.verifying_key();
        let signature = signing_key.sign_prehash(prehash)?;

        Ok(Response::SignPrehash {
            signature,
            verifying_key,
        })
    }
}

impl Service<Request> for SigningService {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response>> + Send + 'static>>;

    fn poll_ready(&mut self, _ctx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let result = match request {
            Request::SignPrehash {
                key_handle,
                prehash,
            } => self.sign_prehash(key_handle, &prehash),
        };

        Box::pin(async { result })
    }
}

/// Requests to the signing service.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    /// Sign the provided prehash.
    SignPrehash {
        /// Handle to the given signing key.
        key_handle: KeyHandle,

        /// Message prehash to be signed.
        prehash: Bytes,
    },
}

/// Responses from the signing service.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Response {
    SignPrehash {
        /// Verifying key which corresponds to this signer.
        verifying_key: VerifyingKey,

        /// Resulting algorithm-specific signature, serialized as bytes.
        signature: Bytes,
    },
}

/// Handle to a key in the signing keyring.
// TODO(tarcieri): OCap-like access control for key handles?
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum KeyHandle {
    /// Key identified by its Ethereum address, e.g.
    /// `0x27b1fdb04752bbc536007a920d24acb045561c26`
    #[cfg(feature = "ethereum")]
    Ethereum(ethereum::Address),
}

#[cfg(feature = "ethereum")]
impl From<ethereum::Address> for KeyHandle {
    fn from(eth_addr: ethereum::Address) -> KeyHandle {
        KeyHandle::Ethereum(eth_addr)
    }
}
