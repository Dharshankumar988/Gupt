//! Error types for the Gupt platform.
//!
//! [`GuptError`] is the top-level error enum shared across all crates.

use thiserror::Error;

/// Top-level error type used throughout the Gupt platform.
#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum GuptError {
    /// A cryptographic operation failed.
    #[error("crypto error: {0}")]
    Crypto(String),

    /// A storage / database operation failed.
    #[error("storage error: {0}")]
    Storage(String),

    /// A transport-layer operation failed.
    #[error("transport error: {0}")]
    Transport(String),

    /// An identity operation failed.
    #[error("identity error: {0}")]
    Identity(String),

    /// A network-level error occurred.
    #[error("network error: {0}")]
    Network(String),

    /// Serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(String),

    /// The caller supplied invalid input.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// The requested resource was not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// The caller is not authorized.
    #[error("unauthorized: {0}")]
    Unauthorized(String),

    /// An internal / unexpected error occurred.
    #[error("internal error: {0}")]
    Internal(String),
}

/// Convenience result type using [`GuptError`].
pub type Result<T> = std::result::Result<T, GuptError>;
