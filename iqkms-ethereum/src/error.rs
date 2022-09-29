//! Error types.

use std::fmt;

/// `Result` type with the `iqkms-ethereum` crate's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Malformed Ethereum address.
    AddressMalformed {
        /// Requested address.
        addr: String,
    },

    /// Malformed Keccak256 digest.
    DigestMalformed,

    /// Signing key not found.
    SigningKeyNotFound {
        /// Requested address.
        addr: String,
    },

    /// Signing operation failed.
    SigningFailed {
        /// Reason why the signing operation failed.
        reason: String,
    },
}

impl Error {
    /// Get the `tonic::Code` associated with this error.
    fn code(&self) -> tonic::Code {
        match self {
            Error::AddressMalformed { .. } => tonic::Code::InvalidArgument,
            Error::DigestMalformed { .. } => tonic::Code::InvalidArgument,
            Error::SigningKeyNotFound { .. } => tonic::Code::NotFound,
            Error::SigningFailed { .. } => tonic::Code::Internal,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AddressMalformed { addr } => {
                write!(f, "Ethereum address malformed: \"{}\"", addr)
            }
            Error::DigestMalformed => write!(f, "Keccak256 digest malformed"),
            Error::SigningKeyNotFound { addr } => write!(f, "signing key not found: \"{}\"", addr),
            Error::SigningFailed { reason } => f.write_str(reason),
        }
    }
}

impl From<Error> for tonic::Status {
    fn from(error: Error) -> tonic::Status {
        tonic::Status::new(error.code(), error.to_string())
    }
}

impl From<signing::Error> for Error {
    fn from(_: signing::Error) -> Error {
        Error::SigningFailed {
            reason: "signing operation failed".to_owned(),
        }
    }
}

impl From<signing::signature::Error> for Error {
    fn from(_: signing::signature::Error) -> Error {
        signing::Error.into()
    }
}

impl From<types::Error> for Error {
    fn from(_: types::Error) -> Error {
        // TODO(tarcieri): non-bogus implementation
        Error::AddressMalformed {
            addr: "0xdeadbeef".to_owned(),
        }
    }
}
