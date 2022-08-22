//! Ethereum support.

use crate::{
    ecdsa::{elliptic_curve::sec1::ToEncodedPoint, secp256k1::VerifyingKey},
    Error, Result,
};
use base16ct as hex;
use sec1::consts::U32;
use sha3::{digest::Update, Digest, Keccak256};
use std::{fmt, str::FromStr};

/// Ethereum addresses.
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Address(pub [u8; Self::LENGTH]);

impl Address {
    /// Length of an address in bytes.
    pub const LENGTH: usize = 20;

    /// Prefix of Ethereum addresses.
    pub const PREFIX: &'static str = "0x";

    /// Borrow the address as a byte array.
    pub fn as_bytes(&self) -> &[u8; Self::LENGTH] {
        &self.0
    }

    /// Encode address as EIP-55 mixed-case checksum encoding.
    ///
    /// <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md>
    pub fn to_checksum(&self) -> String {
        let addr_hex = hex::lower::encode_string(self.as_bytes());
        let hash_hex = hex::lower::encode_string(&Keccak256::digest(addr_hex.as_bytes()));
        let mut ret = Self::PREFIX.to_owned();

        for (&c, &h) in addr_hex.as_bytes().iter().zip(hash_hex.as_bytes()) {
            ret.push(if h >= 56 {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            } as char);
        }

        ret
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Address> {
        let s = s.strip_prefix(Self::PREFIX).ok_or(Error)?;
        let mut bytes = [0u8; Self::LENGTH];

        // TODO(tarcieri): validate checksum
        if hex::mixed::decode(s, &mut bytes).map_err(|_| Error)?.len() == Self::LENGTH {
            Ok(Self(bytes))
        } else {
            Err(Error)
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        self.to_checksum()
    }
}

impl TryFrom<&sec1::EncodedPoint<U32>> for Address {
    type Error = Error;

    fn try_from(point: &sec1::EncodedPoint<U32>) -> Result<Address> {
        match point.coordinates() {
            sec1::point::Coordinates::Uncompressed { x, y } => {
                let digest = Keccak256::new().chain(x).chain(y).finalize();

                // Take the last 20 bytes of the digest as the address
                digest[(Address::LENGTH - 20)..]
                    .try_into()
                    .map(Address)
                    .map_err(|_| Error)
            }
            _ => Err(Error),
        }
    }
}

impl TryFrom<sec1::EncodedPoint<U32>> for Address {
    type Error = Error;

    fn try_from(point: sec1::EncodedPoint<U32>) -> Result<Address> {
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

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({})", self.to_string())
    }
}

impl fmt::LowerHex for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.as_bytes() {
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
            assert_eq!(addr.to_checksum(), checksummed_addr);
        }
    }
}
