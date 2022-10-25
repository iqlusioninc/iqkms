//! iqkms protobuf and gRPC client/server definitions.
//!
//! Mechanically generated using `prost` and `tonic`.

#![allow(clippy::derive_partial_eq_without_eq)]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms-sq.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_lifetimes)]

/// Ethereum support.
pub mod ethereum {
    tonic::include_proto!("iqkms.ethereum");
}
