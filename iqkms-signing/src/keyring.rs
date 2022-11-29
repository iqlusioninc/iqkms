use crate::{Error, Result, SigningKey, VerifyingKey};
use crypto::signature::ecdsa;
use std::{
    collections::BTreeMap as Map,
    fmt::{self, Debug},
};

#[cfg(feature = "ethereum")]
use types::ethereum;

/// Keys for producing digital signatures.
#[derive(Default)]
pub struct Keyring {
    /// Signing keys.
    keys: Map<VerifyingKey, SigningKey>,

    /// Ethereum address index.
    #[cfg(feature = "ethereum")]
    eth_index: Map<ethereum::Address, VerifyingKey>,
}

impl Keyring {
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
    pub fn find_by_eth_address(
        &self,
        addr: &ethereum::Address,
    ) -> Result<&ecdsa::secp256k1::SigningKey> {
        let signing_key = self
            .eth_index
            .get(addr)
            .and_then(|vk| self.keys.get(vk))
            .ok_or(Error)?;

        match signing_key {
            SigningKey::EcdsaSecp256k1(key) => Ok(key),
            #[allow(unreachable_patterns)]
            _ => Err(Error),
        }
    }
}

impl Debug for Keyring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Keyring").finish_non_exhaustive()
    }
}
