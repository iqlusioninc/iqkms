//! iqkms protobuf and gRPC client/server definitions.
//!
//! Mechanically generated using `prost` and `tonic`.

#![allow(clippy::derive_partial_eq_without_eq)]

pub mod ethereum {
    tonic::include_proto!("iqkms.ethereum");
}
