//! Error types.

use std::fmt;
use types::BoxError;

/// Result type with the `iqkms-signing` crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type.
// TODO(tarcieri): convert this into an enum?
#[derive(Clone, Copy, Debug)]
pub struct Error;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error")
    }
}

impl From<BoxError> for Error {
    fn from(_: BoxError) -> Error {
        Error
    }
}

impl From<crypto::signature::Error> for Error {
    fn from(_: crypto::signature::Error) -> Error {
        Error
    }
}

#[cfg(feature = "ethereum")]
impl From<types::Error> for Error {
    fn from(_: types::Error) -> Error {
        Error
    }
}
