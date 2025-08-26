use crate::{Error, Result, VerifyingKey};
use crypto::{
    digest::{Digest, sha2::Sha256},
    rand::{OsRng, RngCore},
    signature::{ecdsa, hazmat::PrehashSigner},
};
use std::fmt::{self, Debug};
use types::Bytes;

/// Signing key.
pub enum SigningKey {
    /// ECDSA/secp256k1
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    EcdsaSecp256k1(ecdsa::secp256k1::SigningKey),
}

impl SigningKey {
    /// Generate a random ECDSA/secp256k1 key.
    // TODO(tarcieri): unified `generate` method with algorithm parameter
    #[cfg(feature = "secp256k1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
    #[allow(dead_code)] // TODO(tarcieri): use me!
    pub fn generate_secp256k1() -> Self {
        let mut bytes = [0u8; 32];

        loop {
            OsRng.fill_bytes(&mut bytes);

            if let Ok(signing_key) = ecdsa::secp256k1::SigningKey::from_bytes(&bytes) {
                // TODO(tarcieri): zeroize bytes
                return signing_key.into();
            }
        }
    }

    /// Sign the given message with this key.
    // TODO(tarcieri): support for customizing hash function used
    #[allow(dead_code)] // TODO(tarcieri): use me!
    pub fn sign(&self, msg: &[u8]) -> Result<Bytes> {
        self.sign_prehash(&Sha256::digest(msg))
    }

    /// Sign the given prehashed message digest with this key.
    pub fn sign_prehash(&self, msg_digest: &[u8]) -> Result<Bytes> {
        match self {
            #[cfg(feature = "secp256k1")]
            Self::EcdsaSecp256k1(sk) => {
                PrehashSigner::<ecdsa::secp256k1::Signature>::sign_prehash(sk, msg_digest)
                    .map(|sig| sig.to_vec().into())
                    .map_err(|_| Error)
            }
        }
    }

    /// Get the [`VerifyingKey`] that corresponds to this signing key.
    pub fn verifying_key(&self) -> VerifyingKey {
        match self {
            #[cfg(feature = "secp256k1")]
            SigningKey::EcdsaSecp256k1(sk) => VerifyingKey::EcdsaSecp256k1(sk.verifying_key()),
        }
    }
}

impl Debug for SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKey")
            .field("verifying_key", &self.verifying_key())
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "secp256k1")]
#[cfg_attr(docsrs, doc(cfg(feature = "secp256k1")))]
impl From<ecdsa::secp256k1::SigningKey> for SigningKey {
    #[inline]
    fn from(key: ecdsa::secp256k1::SigningKey) -> SigningKey {
        SigningKey::EcdsaSecp256k1(key)
    }
}
