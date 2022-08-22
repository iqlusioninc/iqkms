//! iqkms Ethereum RPC service.

use crate::Error;
use proto::ethereum::{tx_signer_server::TxSigner, SignTxRequest, SignTxResponse};
use signing::{ethereum::Address, KeyRing};
use tonic::{Request, Response, Status};

/// RPC service.
pub struct RpcService {
    /// Signing keyring.
    keyring: KeyRing,
}

impl RpcService {
    /// Create a new RPC service with the given keyring.
    pub fn new(keyring: KeyRing) -> Self {
        Self { keyring }
    }
}

#[tonic::async_trait]
impl TxSigner for RpcService {
    async fn sign_tx(
        &self,
        request: Request<SignTxRequest>,
    ) -> Result<Response<SignTxResponse>, Status> {
        // TODO(tarcieri): log with tracing
        println!("sign_tx[{:?}]: {:?}", request.remote_addr(), request);

        let request = request.into_inner();
        let address = request.address.parse::<Address>().map_err(Error::from)?;
        let signing_key = self
            .keyring
            .find_by_eth_address(&address)
            .map_err(Error::from)?;

        let signature = signing_key.sign(&request.tx_body).map_err(Error::from)?;
        Ok(Response::new(SignTxResponse { signature }))
    }
}
