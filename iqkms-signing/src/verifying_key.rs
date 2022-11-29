use crypto::signature::ecdsa;

/// Verifying key.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum VerifyingKey {
    /// ECDSA/secp256k1
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1(ecdsa::secp256k1::VerifyingKey),
}
