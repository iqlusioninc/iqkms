//! iqkms Ethereum services.
//!
//! Implements an RPC service with the following features: transaction signing.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

mod error;
mod signer;

pub use crate::{
    error::{Error, Result},
    signer::SignerService,
};
pub use proto::ethereum::signer_server::SignerServer;
