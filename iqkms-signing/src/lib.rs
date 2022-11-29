//! iqkms signing: service and keyring manager for producing digital signatures
//! using keys stored in iqkms.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(
    clippy::integer_arithmetic,
    clippy::mod_module_files,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    future_incompatible,
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_qualifications
)]

mod error;
mod keyring;
mod signing_key;
mod verifying_key;

pub use crate::{
    error::{Error, Result},
    keyring::Keyring,
    signing_key::SigningKey,
    verifying_key::VerifyingKey,
};
pub use crypto::signature;
