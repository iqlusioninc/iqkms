//! Elliptic Curve Digital Signature Algorithm (ECDSA) support.

pub use ::ecdsa::*;

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub mod secp256k1;

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
pub use {self::secp256k1::Secp256k1Signer, k256::Secp256k1};
