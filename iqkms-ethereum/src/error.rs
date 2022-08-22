//! Error types.

use std::fmt;

/// Error type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Malformed Ethereum address.
    AddressMalformed {
        /// Requested address.
        addr: String,
    },

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
