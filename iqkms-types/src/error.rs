//! Error types.

/// Box containing a thread-safe + `'static` error suitable for use as a as
/// `std::error::Error::source`.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Error type.
// TODO(tarcieri): convert this into an enum?
#[derive(Clone, Debug)]
pub struct Error;

/// Result type with the `iqkms-types` crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;
