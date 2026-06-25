//! XChaCha20-Poly1305 authenticated encryption.

use crate::error::CryptoError;
use crate::Result;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// The result of an encryption operation, containing the ciphertext and the
/// random nonce that was used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPayload {
    /// The encrypted data (ciphertext + Poly1305 tag).
    pub ciphertext: Vec<u8>,
    /// The 24-byte nonce used for encryption.
    pub nonce: Vec<u8>,
}

/// Generate a random 24-byte nonce suitable for XChaCha20-Poly1305.
pub fn generate_nonce() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    rand::rngs::OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Encrypt `plaintext` using XChaCha20-Poly1305 with the given 32-byte `key`.
///
/// A fresh random nonce is generated for each call.
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedPayload> {
    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::Encryption(format!("invalid key: {e}")))?;

    let nonce_bytes = generate_nonce();
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::Encryption(e.to_string()))?;

    Ok(EncryptedPayload {
        ciphertext,
        nonce: nonce_bytes.to_vec(),
    })
}

/// Decrypt an [`EncryptedPayload`] using XChaCha20-Poly1305 with the given
/// 32-byte `key`.
pub fn decrypt(payload: &EncryptedPayload, key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::Decryption(format!("invalid key: {e}")))?;

    let nonce_bytes: [u8; 24] = payload
        .nonce
        .as_slice()
        .try_into()
        .map_err(|_| CryptoError::InvalidNonce("nonce must be 24 bytes".into()))?;
    let nonce = XNonce::from_slice(&nonce_bytes);

    cipher
        .decrypt(nonce, payload.ciphertext.as_slice())
        .map_err(|e| CryptoError::Decryption(e.to_string()))
}
