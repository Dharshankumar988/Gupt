//! Key derivation functions.
//!
//! - **Argon2id** for password-based key derivation
//! - **HKDF-SHA256** for expanding shared secrets
//! - **SHA-256** hashing utility

use crate::error::CryptoError;
use crate::Result;
use argon2::Argon2;
use hkdf::Hkdf;
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Derive a 32-byte encryption key from a user-supplied `pin` using Argon2id.
///
/// `salt` should be a unique random value stored alongside the ciphertext.
pub fn derive_key_from_pin(pin: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let mut output = [0u8; 32];
    Argon2::default()
        .hash_password_into(pin.as_bytes(), salt, &mut output)
        .map_err(|e| CryptoError::KeyDerivation(format!("argon2id failed: {e}")))?;
    Ok(output)
}

/// Derive a 32-byte key from a shared secret using HKDF-SHA256.
///
/// `context` is used as the HKDF info parameter to bind the derived key to
/// a specific purpose.
pub fn derive_key_from_shared_secret(
    shared_secret: &[u8; 32],
    context: &[u8],
) -> Result<[u8; 32]> {
    let hk = Hkdf::<Sha256>::new(None, shared_secret);
    let mut okm = [0u8; 32];
    hk.expand(context, &mut okm)
        .map_err(|e| CryptoError::KeyDerivation(format!("hkdf expand failed: {e}")))?;
    Ok(okm)
}

/// Generate a random 32-byte salt.
pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    salt
}

/// Compute the SHA-256 hash of `data`.
pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(data);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}
