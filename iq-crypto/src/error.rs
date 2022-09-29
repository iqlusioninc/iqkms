use core::fmt::{self, Display};

/// `Result` type with the `iq-crypto` crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Opaque error type for `iqkms-crypto`.
///
/// Using an opaque error type avoids potential sidechannel leakage between
/// error cases.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("iq-crypto error")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

#[cfg(feature = "signature")]
#[cfg_attr(docsrs, doc(cfg(feature = "signature")))]
impl From<signature::Error> for Error {
    fn from(_: signature::Error) -> Error {
        Error
    }
}
