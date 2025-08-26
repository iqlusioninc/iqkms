use crate::{Error, Result, signing_key::SigningKey, verifying_key::VerifyingKey};
use std::{
    collections::BTreeMap as Map,
    fmt::{self, Debug},
};

#[cfg(feature = "ethereum")]
use types::ethereum;

/// Keys for producing digital signatures.
#[derive(Default)]
pub(crate) struct Keyring {
    /// Signing keys.
    keys: Map<VerifyingKey, SigningKey>,

    /// Ethereum address index.
    #[cfg(feature = "ethereum")]
    eth_index: Map<ethereum::Address, VerifyingKey>,
}

impl Keyring {
    /// Add a key to the ring.
    #[allow(dead_code)] // TODO(tarcieri): use me!
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
    pub fn find_by_eth_address(&self, eth_addr: &ethereum::Address) -> Result<&SigningKey> {
        self.eth_index
            .get(eth_addr)
            .and_then(|vk| self.keys.get(vk))
            .ok_or(Error)
    }
}

impl Debug for Keyring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Keyring").finish_non_exhaustive()
    }
}
