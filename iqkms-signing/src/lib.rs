//! iqkms keyring: data structure which stores available keys.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "ethereum")]
#[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
pub mod ethereum;

mod error;

pub use crate::error::{Error, Result};
pub use rand_core::{OsRng, RngCore};

use signatory::{ecdsa, signature::Signer};
use std::collections::BTreeMap as Map;

/// Keys for producing digital signatures.
#[derive(Default)]
pub struct KeyRing {
    /// Signing keys.
    keys: Map<VerifyingKey, SigningKey>,

    /// Ethereum address index.
    #[cfg(feature = "ethereum")]
    eth_index: Map<ethereum::Address, VerifyingKey>,
}

impl KeyRing {
    /// Create a new key ring.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a key to the ring.
    pub fn add(&mut self, signing_key: SigningKey) -> Result<()> {
        let verifying_key = signing_key.verifying_key();

        #[cfg(feature = "ethereum")]
        #[allow(irrefutable_let_patterns)]
        if let VerifyingKey::EcdsaSecp256k1(vk) = &verifying_key {
            self.eth_index.insert(vk.try_into()?, verifying_key.clone());
        }

        if self.keys.insert(verifying_key, signing_key).is_some() {
            Err(Error)
        } else {
            Ok(())
        }
    }

    /// Find a key by its Ethereum address.
    #[cfg(feature = "ethereum")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ethereum")))]
    pub fn find_by_eth_address(&self, addr: &ethereum::Address) -> Result<&SigningKey> {
        self.eth_index
            .get(addr)
            .and_then(|vk| self.keys.get(vk))
            .ok_or(Error)
    }
}

/// Signing key types.
pub enum SigningKey {
    /// ECDSA/secp256k1
    EcdsaSecp256k1(ecdsa::secp256k1::SigningKey),
}

impl SigningKey {
    /// Generate a random ECDSA/secp256k1 key.
    // TODO(tarcieri): unified `generate` method with algorithm parameter
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
    pub fn sign(&self, msg: &[u8]) -> Result<Vec<u8>> {
        match self {
            Self::EcdsaSecp256k1(sk) => {
                let sig: ecdsa::secp256k1::Signature = sk.try_sign(msg).map_err(|_| Error)?;
                Ok(sig.to_vec())
            }
        }
    }

    /// Get the [`VerifyingKey`] that corresponds to this signing key.
    pub fn verifying_key(&self) -> VerifyingKey {
        match self {
            SigningKey::EcdsaSecp256k1(sk) => VerifyingKey::EcdsaSecp256k1(sk.verifying_key()),
        }
    }
}

impl From<ecdsa::secp256k1::SigningKey> for SigningKey {
    #[inline]
    fn from(key: ecdsa::secp256k1::SigningKey) -> SigningKey {
        SigningKey::EcdsaSecp256k1(key)
    }
}

/// Verifying key types.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum VerifyingKey {
    /// ECDSA/secp256k1
    EcdsaSecp256k1(ecdsa::secp256k1::VerifyingKey),
}