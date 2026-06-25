//! # gupt-crypto
//!
//! Cryptographic primitives for the Gupt platform.
//!
//! - **Ed25519** signing and verification
//! - **X25519** Diffie–Hellman key agreement
//! - **XChaCha20-Poly1305** authenticated encryption
//! - **Argon2id** password-based key derivation
//! - **HKDF-SHA256** key expansion

pub mod encryption;
pub mod error;
pub mod kdf;
pub mod keys;
pub mod signing;

pub use encryption::EncryptedPayload;
pub use error::CryptoError;
pub use keys::{EncryptionKeyPair, KeyBundle, SigningKeyPair};

/// Convenience result type for cryptographic operations.
pub type Result<T> = std::result::Result<T, CryptoError>;
