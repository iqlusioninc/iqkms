//! Ethereum support.

// Re-export select types from the `ethereum-types` crate.
pub use ethereum_types::{
    BigEndianHash, FromDecStrErr, FromStrRadixErr, FromStrRadixErrKind, H128, H160, H256, H264,
    H32, H512, H520, H64, U128, U256, U512, U64,
};

use crate::{Error, Result};
use crypto::{
    digest::{sha3::Keccak256, Digest, Update},
    elliptic_curve::{
        sec1::{self, ToEncodedPoint},
        secp256k1::EncodedPoint,
    },
    signature::ecdsa::secp256k1::VerifyingKey,
};
use std::{fmt, str::FromStr};

/// EIP-155 chain ID.
///
/// <https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids>
pub type ChainId = u64;

/// Address serialized as bytes.
type AddrBytes = [u8; Address::LENGTH];

/// Ethereum addresses.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Address {
    /// Keccak256 digest of the public key.
    pub hash: H160,

    /// Chain ID to use in EIP-1191 checksum.
    ///
    /// <https://eips.ethereum.org/EIPS/eip-1191>
    pub chain_id: Option<ChainId>,
}

impl Address {
    /// Length of an address in bytes.
    pub const LENGTH: usize = 20;

    /// Prefix of Ethereum addresses.
    pub const PREFIX: &'static str = "0x";

    /// Serialize the digest as bytes.
    pub fn to_bytes(&self) -> AddrBytes {
        self.hash.into()
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        self.hash.as_ref()
    }
}

impl From<AddrBytes> for Address {
    fn from(bytes: AddrBytes) -> Address {
        H160::from(bytes).into()
    }
}

impl From<H160> for Address {
    fn from(hash: H160) -> Address {
        Address {
            hash,
            chain_id: None,
        }
    }
}

impl From<Address> for AddrBytes {
    fn from(addr: Address) -> AddrBytes {
        addr.to_bytes()
    }
}

impl From<Address> for H160 {
    fn from(addr: Address) -> H160 {
        addr.hash
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Address> {
        let s = s.strip_prefix(Self::PREFIX).ok_or(Error)?;
        let mut bytes = AddrBytes::default();

        // TODO(tarcieri): validate EIP-55 checksum
        if hex::mixed::decode(s, &mut bytes).map_err(|_| Error)?.len() == Self::LENGTH {
            Ok(Self {
                hash: bytes.into(),
                chain_id: None,
            })
        } else {
            Err(Error)
        }
    }
}

/// Encode address as EIP-55 mixed-case checksum encoding.
///
/// <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md>
impl ToString for Address {
    fn to_string(&self) -> String {
        let addr_hex = hex::lower::encode_string(self.as_ref());

        let prefixed_addr = match self.chain_id {
            Some(chain_id) => format!("{}0x{}", chain_id, addr_hex),
            None => addr_hex.clone(),
        };

        let hash_hex = hex::lower::encode_string(&Keccak256::digest(prefixed_addr.as_bytes()));
        let mut ret = Self::PREFIX.to_owned();

        for (mut c, &h) in addr_hex.as_bytes().iter().copied().zip(hash_hex.as_bytes()) {
            if h >= 56 {
                c = c.to_ascii_uppercase();
            }

            ret.push(c.into());
        }

        ret
    }
}

impl TryFrom<&EncodedPoint> for Address {
    type Error = Error;

    fn try_from(point: &EncodedPoint) -> Result<Address> {
        match point.coordinates() {
            sec1::Coordinates::Uncompressed { x, y } => {
                let digest = Keccak256::new().chain(x).chain(y).finalize();

                // Take the last 20 bytes of the digest as the address
                #[allow(clippy::integer_arithmetic)]
                digest[(Address::LENGTH - 20)..].try_into()
            }
            _ => Err(Error),
        }
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Address> {
        AddrBytes::try_from(bytes)
            .map(|bytes| H160::from(bytes).into())
            .map_err(|_| Error)
    }
}

impl TryFrom<EncodedPoint> for Address {
    type Error = Error;

    fn try_from(point: EncodedPoint) -> Result<Address> {
        Address::try_from(&point)
    }
}

impl TryFrom<&VerifyingKey> for Address {
    type Error = Error;

    fn try_from(verifying_key: &VerifyingKey) -> Result<Address> {
        verifying_key.to_encoded_point(false).try_into()
    }
}

impl TryFrom<VerifyingKey> for Address {
    type Error = Error;

    fn try_from(verifying_key: VerifyingKey) -> Result<Address> {
        Address::try_from(&verifying_key)
    }
}

impl fmt::LowerHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.to_bytes() {
            write!(f, "{:x}", byte)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn checksum_encoding() {
        // Note: adapted from the list in ethers-rs
        // https://docs.rs/ethers-core/0.17.0/src/ethers_core/utils/mod.rs.html#467-545
        let addr_list = [
            "0x27b1fdb04752bbc536007a920d24acb045561c26",
            "0x3599689E6292b81B2d85451025146515070129Bb",
            "0x42712D45473476b98452f434e72461577D686318",
            "0x52908400098527886E0F7030069857D2E4169EE7",
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
            "0x6549f4939460DE12611948b3f82b88C3C8975323",
            "0x66f9664f97F2b50F62D13eA064982f936dE76657",
            "0x88021160C5C792225E4E5452585947470010289D",
        ];

        for checksummed_addr in addr_list {
            let addr = checksummed_addr.parse::<Address>().unwrap();
            assert_eq!(addr.to_string(), checksummed_addr);
        }
    }
}
