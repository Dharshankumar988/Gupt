//! Crypto-specific error types.

use thiserror::Error;

/// Errors produced by cryptographic operations.
#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum CryptoError {
    /// Key generation failed.
    #[error("key generation error: {0}")]
    KeyGeneration(String),

    /// Signing failed.
    #[error("signing error: {0}")]
    Signing(String),

    /// Signature verification failed.
    #[error("verification error: {0}")]
    Verification(String),

    /// Encryption failed.
    #[error("encryption error: {0}")]
    Encryption(String),

    /// Decryption failed.
    #[error("decryption error: {0}")]
    Decryption(String),

    /// Key derivation failed.
    #[error("key derivation error: {0}")]
    KeyDerivation(String),

    /// The supplied key is invalid.
    #[error("invalid key: {0}")]
    InvalidKey(String),

    /// The supplied nonce is invalid.
    #[error("invalid nonce: {0}")]
    InvalidNonce(String),

    /// The supplied signature is invalid.
    #[error("invalid signature: {0}")]
    InvalidSignature(String),
}
