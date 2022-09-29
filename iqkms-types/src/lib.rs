//! iqkms types: core types shared between clients and services.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub mod ethereum;

mod error;

pub use crate::error::{Error, Result};

#[cfg(feature = "crypto")]
#[cfg_attr(docsrs, doc(cfg(feature = "crypto")))]
pub use crypto;
