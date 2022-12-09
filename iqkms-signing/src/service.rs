use crate::{Error, Keyring, Result};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;
use types::Bytes;

#[cfg(feature = "ethereum")]
use types::ethereum;

/// Tower service which controls access to the signing [`Keyring`].
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
    fn sign_prehash(&self, req: SignPrehashRequest) -> Result<Response> {
        let signing_key = match req.key_handle {
            #[cfg(feature = "ethereum")]
            KeyHandle::Ethereum(eth_addr) => self.keyring.find_by_eth_address(&eth_addr)?,
            #[allow(unreachable_patterns)]
            _ => return Err(Error),
        };

        let signature = signing_key.sign_prehash(&req.prehash)?;
        Ok(Response::SignPrehash(SignPrehashResponse { signature }))
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
            Request::SignPrehash(req) => self.sign_prehash(req),
        };

        Box::pin(async { result })
    }
}

/// Requests to the signing service.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    SignPrehash(SignPrehashRequest),
}

/// Responses from the signing service.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Response {
    SignPrehash(SignPrehashResponse),
}

/// Request to sign the given prehash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignPrehashRequest {
    /// Handle to the given signing key.
    key_handle: KeyHandle,

    /// Message prehash to be signed.
    prehash: Bytes,
}

/// Response from a [`SignPrehashRequest`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignPrehashResponse {
    /// Resulting algorithm-specific signature, serialized as bytes.
    signature: Bytes,
}

/// Handle to a key in the signing keyring.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum KeyHandle {
    /// Key identified by its Ethereum address, e.g.
    /// `0x27b1fdb04752bbc536007a920d24acb045561c26`
    #[cfg(feature = "ethereum")]
    Ethereum(ethereum::Address),
}
