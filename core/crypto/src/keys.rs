//! Cryptographic key pair management.
//!
//! Provides Ed25519 signing keys and X25519 key-agreement keys.

use crate::error::CryptoError;
use crate::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};
use zeroize::Zeroize;

/// Ed25519 signing key pair.
///
/// The secret key is zeroized when the struct is dropped.
pub struct SigningKeyPair {
    /// The secret signing key.
    signing_key: SigningKey,
    /// The corresponding public verifying key.
    verifying_key: VerifyingKey,
}

impl std::fmt::Debug for SigningKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SigningKeyPair")
            .field("public_key", &hex::encode(self.verifying_key.as_bytes()))
            .finish()
    }
}

impl Drop for SigningKeyPair {
    fn drop(&mut self) {
        // Zeroize the secret key material on drop.
        let bytes = self.signing_key.to_bytes();
        let mut owned = bytes;
        owned.zeroize();
    }
}

impl SigningKeyPair {
    /// Generate a new random Ed25519 signing key pair.
    pub fn generate() -> Result<Self> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Reconstruct a signing key pair from 32 secret-key bytes.
    pub fn from_bytes(secret: &[u8]) -> Result<Self> {
        let bytes: [u8; 32] = secret
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("signing key must be 32 bytes".into()))?;
        let signing_key = SigningKey::from_bytes(&bytes);
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            signing_key,
            verifying_key,
        })
    }

    /// Return the 32-byte public verifying key.
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    /// Return the 32-byte secret signing key.
    pub fn secret_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Borrow the inner [`SigningKey`] for signing operations.
    pub(crate) fn inner(&self) -> &SigningKey {
        &self.signing_key
    }
}

/// X25519 Diffie–Hellman key pair for key agreement.
pub struct EncryptionKeyPair {
    /// The static secret key.
    secret: StaticSecret,
    /// The corresponding public key.
    public: X25519PublicKey,
}

impl std::fmt::Debug for EncryptionKeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptionKeyPair")
            .field("public_key", &hex::encode(self.public.as_bytes()))
            .finish()
    }
}

impl EncryptionKeyPair {
    /// Generate a new random X25519 key pair.
    pub fn generate() -> Result<Self> {
        let secret = StaticSecret::random_from_rng(OsRng);
        let public = X25519PublicKey::from(&secret);
        Ok(Self { secret, public })
    }

    /// Reconstruct a key pair from 32 secret-key bytes.
    pub fn from_bytes(secret_bytes: &[u8]) -> Result<Self> {
        let bytes: [u8; 32] = secret_bytes
            .try_into()
            .map_err(|_| CryptoError::InvalidKey("encryption key must be 32 bytes".into()))?;
        let secret = StaticSecret::from(bytes);
        let public = X25519PublicKey::from(&secret);
        Ok(Self { secret, public })
    }

    /// Return the 32-byte public key.
    pub fn public_key_bytes(&self) -> [u8; 32] {
        *self.public.as_bytes()
    }

    /// Perform X25519 Diffie–Hellman to derive a shared secret.
    pub fn derive_shared_secret(&self, their_public: &[u8; 32]) -> Result<[u8; 32]> {
        let their_pk = X25519PublicKey::from(*their_public);
        let shared = self.secret.diffie_hellman(&their_pk);
        Ok(*shared.as_bytes())
    }
}

/// A bundle containing both signing and encryption key pairs.
#[derive(Debug)]
pub struct KeyBundle {
    /// Ed25519 signing key pair.
    pub signing: SigningKeyPair,
    /// X25519 encryption key pair.
    pub encryption: EncryptionKeyPair,
}

impl KeyBundle {
    /// Generate a fresh key bundle with random keys.
    pub fn generate() -> Result<Self> {
        Ok(Self {
            signing: SigningKeyPair::generate()?,
            encryption: EncryptionKeyPair::generate()?,
        })
    }
}
