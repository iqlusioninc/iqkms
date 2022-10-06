//! iqkms client error types

use std::fmt;
pub use tonic::Code as ErrorCode;

/// `Result` type with the `iqkms` crate's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

/// iqkms client errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// Error code.
    code: ErrorCode,

    /// Error message.
    msg: String,
}

impl Error {
    /// Get the [`ErrorCode`] for this error.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Get the error message for this error.
    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        self.msg()
    }
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Error {
        Error {
            code: status.code(),
            msg: status.message().to_owned(),
        }
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(err: tonic::transport::Error) -> Error {
        Error {
            // TODO(tarcieri): better represent this case? (connection failed)
            code: ErrorCode::Unavailable,
            msg: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.msg())
    }
}

impl std::error::Error for Error {}
