//! iqkms Ethereum services.
//!
//! Implements an RPC service with the following features: transaction signing.

mod error;
mod rpc_service;

pub use crate::{error::Error, rpc_service::RpcService};
pub use proto::ethereum::tx_signer_server::TxSignerServer;
