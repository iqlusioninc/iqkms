//! Digital signature algorithms.

#[cfg(feature = "ecdsa")]
#[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
pub mod ecdsa;

mod algorithm;

pub use self::algorithm::Algorithm;
pub use ::signature::{
    DigestSigner, DigestVerifier, Error, Keypair, PrehashSignature, RandomizedDigestSigner,
    RandomizedSigner, Result, Signature, Signer, SignerMut, Verifier, hazmat,
};
