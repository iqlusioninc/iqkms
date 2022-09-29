//! iqkms client library

mod error;

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub mod ethereum;

pub use crate::error::{Error, ErrorCode, Result};
pub use proto;
pub use tokio;
pub use tonic;

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub use types;

/// Boxed `Error` trait object.
pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
