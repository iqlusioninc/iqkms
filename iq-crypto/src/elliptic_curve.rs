//! Elliptic curves.

pub use ::elliptic_curve::*;

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub use k256 as secp256k1;
