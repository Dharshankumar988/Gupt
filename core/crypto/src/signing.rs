//! Ed25519 message signing and verification.

use crate::error::CryptoError;
use crate::keys::SigningKeyPair;
use crate::Result;
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};

/// Sign `message` with the given [`SigningKeyPair`], returning the 64-byte
/// Ed25519 signature.
pub fn sign(message: &[u8], signing_key: &SigningKeyPair) -> Result<Vec<u8>> {
    let sig = signing_key
        .inner()
        .try_sign(message)
        .map_err(|e| CryptoError::Signing(e.to_string()))?;
    Ok(sig.to_bytes().to_vec())
}

/// Verify an Ed25519 `signature` over `message` using the given 32-byte
/// `public_key`.
///
/// Returns `true` if the signature is valid, `false` otherwise.
pub fn verify(message: &[u8], signature: &[u8], public_key: &[u8; 32]) -> Result<bool> {
    let verifying_key = VerifyingKey::from_bytes(public_key)
        .map_err(|e| CryptoError::InvalidKey(e.to_string()))?;

    let sig_bytes: [u8; 64] = signature
        .try_into()
        .map_err(|_| CryptoError::InvalidSignature("signature must be 64 bytes".into()))?;

    let sig = Signature::from_bytes(&sig_bytes);

    match verifying_key.verify(message, &sig) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}
