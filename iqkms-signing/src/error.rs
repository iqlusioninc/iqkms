//! Error types.

/// Error type.
// TODO(tarcieri): convert this into an enum?
#[derive(Clone, Copy, Debug)]
pub struct Error;

/// Result type with the `iqkms-signing` crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

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
