//! ECDSA/secp256k1 support.

pub use k256::ecdsa::{
    recoverable::{Id as RecoveryId, Signature as RecoverableSignature},
    Signature, VerifyingKey,
};

use crate::{Error, Result};
use alloc::boxed::Box;
use core::fmt;
use pkcs8::DecodePrivateKey;
use signature::hazmat::PrehashSigner;

/// ECDSA/secp256k1 signing key.
pub struct SigningKey {
    inner: Box<dyn Secp256k1Signer + Send + Sync>,
}

impl SigningKey {
    /// Initialize from a provided signer object.
    ///
    /// Use [`SigningKey::from_bytes`] to initialize from a raw private key.
    pub fn new(signer: Box<dyn Secp256k1Signer + Send + Sync>) -> Self {
        Self { inner: signer }
    }

    /// Initialize from a raw scalar value (big endian).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let signing_key = k256::ecdsa::SigningKey::from_bytes(bytes)?;
        Ok(Self::new(Box::new(signing_key)))
    }

    /// Get the verifying key that corresponds to this signing key.
    pub fn verifying_key(&self) -> VerifyingKey {
        self.inner.verifying_key()
    }
}

impl DecodePrivateKey for SigningKey {}

impl TryFrom<pkcs8::PrivateKeyInfo<'_>> for SigningKey {
    type Error = pkcs8::Error;

    fn try_from(private_key: pkcs8::PrivateKeyInfo<'_>) -> pkcs8::Result<Self> {
        k256::ecdsa::SigningKey::try_from(private_key).map(|key| Self::new(Box::new(key)))
    }
}

impl TryFrom<&[u8]> for SigningKey {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::from_bytes(bytes)
    }
}

impl PrehashSigner<Signature> for SigningKey {
    fn sign_prehash(&self, prehash: &[u8]) -> signature::Result<Signature> {
        self.inner.sign_prehash(prehash)
    }
}

impl PrehashSigner<RecoverableSignature> for SigningKey {
    fn sign_prehash(&self, prehash: &[u8]) -> signature::Result<RecoverableSignature> {
        let prehash = <[u8; 32]>::try_from(prehash).map_err(signature::Error::from_source)?;
        let sig = PrehashSigner::<Signature>::sign_prehash(self, &prehash)?;
        RecoverableSignature::from_digest_bytes_trial_recovery(
            &self.verifying_key(),
            prehash.as_ref().into(),
            &sig,
        )
    }
}

impl fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKey")
            .field("verifying_key", &self.verifying_key())
            .finish()
    }
}

/// ECDSA/secp256k1 signer
pub trait Secp256k1Signer: PrehashSigner<Signature> {
    /// Get the ECDSA verifying key for this signer
    fn verifying_key(&self) -> VerifyingKey;
}

impl<T> Secp256k1Signer for T
where
    T: PrehashSigner<Signature>,
    VerifyingKey: for<'a> From<&'a T>,
{
    fn verifying_key(&self) -> VerifyingKey {
        self.into()
    }
}
