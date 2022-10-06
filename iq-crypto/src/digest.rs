//! Digest algorithms a.k.a. cryptographic hash functions.
//!
//! This module contains a complete re-export of the [`digest`] crate
//! along with feature-gated re-exports of various hash functions.
//!
//! [`digest`]: https://docs.rs/digest

pub use ::digest::*;

#[cfg(feature = "sha2")]
#[cfg_attr(docsrs, doc(cfg(feature = "sha2")))]
pub use sha2;

#[cfg(feature = "sha3")]
#[cfg_attr(docsrs, doc(cfg(feature = "sha3")))]
pub use sha3;
