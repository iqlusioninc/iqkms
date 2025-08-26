//! iqkms client library

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::arithmetic_side_effects,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

mod error;

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub mod ethereum;

pub use crate::error::{Error, ErrorCode, Result};
pub use proto;
pub use tokio;
pub use tonic;

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub use types;

/// Boxed `Error` trait object.
pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
