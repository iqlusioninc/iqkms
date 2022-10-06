//! Algorithms supported by this library.

/// Signature algorithms.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Algorithm {
    /// ECDSA with NIST P-256.
    #[cfg(feature = "nistp256")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nistp256")))]
    EcdsaNistP256,

    /// ECDSA with NIST P-384.
    #[cfg(feature = "nistp384")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nistp384")))]
    EcdsaNistP384,

    /// ECDSA with secp256k1.
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1,

    /// Ed25519.
    #[cfg(feature = "ed25519")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ed25519")))]
    Ed25519,
}

impl Algorithm {
    /// Is the algorithm ECDSA?
    #[cfg(feature = "ecdsa")]
    pub fn is_ecdsa(self) -> bool {
        #[cfg(feature = "nistp256")]
        if self == Algorithm::EcdsaNistP256 {
            return true;
        }

        #[cfg(feature = "nistp384")]
        if self == Algorithm::EcdsaNistP384 {
            return true;
        }

        #[cfg(feature = "secp256k1")]
        if self == Algorithm::EcdsaSecp256k1 {
            return true;
        }

        false
    }
}
