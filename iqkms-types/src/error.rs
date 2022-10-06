//! Error types.

/// Error type.
// TODO(tarcieri): convert this into an enum?
#[derive(Clone, Debug)]
pub struct Error;

/// Result type with the `iqkms-types` crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;
