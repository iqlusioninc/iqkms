//! iqkms types: core types shared between clients and services.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::integer_arithmetic,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub mod ethereum;

mod error;

pub use crate::error::{Error, Result};

#[cfg(feature = "crypto")]
#[cfg_attr(docsrs, doc(cfg(feature = "crypto")))]
pub use crypto;

#[cfg(feature = "hex")]
#[cfg_attr(docsrs, doc(cfg(feature = "hex")))]
pub use hex;

/// Immutable bytestring representation.
// TODO(tarcieri): use `bytes::Bytes` instead?
pub type Bytes = Vec<u8>;
