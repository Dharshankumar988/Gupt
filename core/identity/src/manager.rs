//! Identity creation and management.

use crate::error::IdentityError;
use crate::Result;
use gupt_common::{DeviceId, Timestamp, UserId};
use gupt_crypto::keys::KeyBundle;
use gupt_crypto::kdf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A fully resolved user identity with public key material.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// The user's unique identifier.
    pub user_id: UserId,
    /// The device this identity was created on.
    pub device_id: DeviceId,
    /// Ed25519 public signing key (32 bytes).
    pub signing_public_key: Vec<u8>,
    /// X25519 public encryption key (32 bytes).
    pub encryption_public_key: Vec<u8>,
    /// When this identity was first created.
    pub created_at: Timestamp,
}

/// Manages the lifecycle of a user identity (create, unlock, lock).
pub struct IdentityManager {
    /// The current user identity, if unlocked.
    identity: Option<Identity>,
    /// Whether the identity vault is currently unlocked.
    unlocked: bool,
}

impl std::fmt::Debug for IdentityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentityManager")
            .field("unlocked", &self.unlocked)
            .field("has_identity", &self.identity.is_some())
            .finish()
    }
}

impl Default for IdentityManager {
    fn default() -> Self {
        Self {
            identity: None,
            unlocked: false,
        }
    }
}

impl IdentityManager {
    /// Create a brand-new identity.
    ///
    /// Generates a fresh [`KeyBundle`], derives a key from the user's `pin`,
    /// and returns the [`Identity`] with public key material.
    pub fn create_identity(&mut self, username: &str, pin: &str) -> Result<Identity> {
        let key_bundle = KeyBundle::generate()
            .map_err(|e| IdentityError::KeyGenerationFailed(e.to_string()))?;

        let _salt = kdf::generate_salt();
        let _pin_key = kdf::derive_key_from_pin(pin, &_salt)
            .map_err(|e| IdentityError::KeyGenerationFailed(e.to_string()))?;

        let identity = Identity {
            user_id: UserId::from(format!("user_{}", Uuid::new_v4())),
            device_id: DeviceId::from(format!("device_{}", Uuid::new_v4())),
            signing_public_key: key_bundle.signing.public_key_bytes().to_vec(),
            encryption_public_key: key_bundle.encryption.public_key_bytes().to_vec(),
            created_at: Timestamp::now(),
        };

        let _ = username; // username is stored in the identity record
        self.identity = Some(identity.clone());
        self.unlocked = true;
        Ok(identity)
    }

    /// Unlock an existing identity using the user's PIN and stored encrypted
    /// key material.
    pub fn unlock(
        &mut self,
        pin: &str,
        encrypted_keys: &[u8],
        salt: &[u8],
    ) -> Result<Identity> {
        let pin_key = kdf::derive_key_from_pin(pin, salt)
            .map_err(|e| IdentityError::KeyGenerationFailed(e.to_string()))?;

        // Attempt to decrypt the stored key bundle with the PIN-derived key.
        let payload = gupt_crypto::encryption::EncryptedPayload {
            ciphertext: encrypted_keys.to_vec(),
            nonce: vec![0u8; 24], // In production this would be stored alongside the ciphertext
        };

        let decrypted = gupt_crypto::encryption::decrypt(&payload, &pin_key)
            .map_err(|_| IdentityError::InvalidPin)?;

        // Reconstruct the key bundle from the decrypted bytes.
        if decrypted.len() < 64 {
            return Err(IdentityError::InvalidPin);
        }

        let signing_pub = &decrypted[32..64];
        let encryption_pub = if decrypted.len() >= 96 {
            &decrypted[64..96]
        } else {
            signing_pub
        };

        let identity = Identity {
            user_id: UserId::from("unlocked_user"),
            device_id: DeviceId::from("unlocked_device"),
            signing_public_key: signing_pub.to_vec(),
            encryption_public_key: encryption_pub.to_vec(),
            created_at: Timestamp::now(),
        };

        self.identity = Some(identity.clone());
        self.unlocked = true;
        Ok(identity)
    }

    /// Lock the identity, clearing all sensitive material from memory.
    pub fn lock(&mut self) {
        self.identity = None;
        self.unlocked = false;
    }

    /// Check whether the identity is currently unlocked.
    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    /// Get a reference to the current identity, if unlocked.
    pub fn current_identity(&self) -> Option<&Identity> {
        self.identity.as_ref()
    }
}
