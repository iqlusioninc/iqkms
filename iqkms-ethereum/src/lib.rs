//! iqkms Ethereum services.
//!
//! Implements an RPC service with the following features: transaction signing.

mod error;
mod signer;

pub use crate::{
    error::{Error, Result},
    signer::SignerService,
};
pub use proto::ethereum::signer_server::SignerServer;
