//! Identity-specific error types.

use thiserror::Error;

/// Errors produced by identity operations.
#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum IdentityError {
    /// The identity system has not been initialized yet.
    #[error("identity not initialized")]
    NotInitialized,

    /// The supplied PIN is invalid.
    #[error("invalid PIN")]
    InvalidPin,

    /// Key generation failed.
    #[error("key generation failed: {0}")]
    KeyGenerationFailed(String),

    /// Persisting identity data failed.
    #[error("storage failed: {0}")]
    StorageFailed(String),

    /// The current session has expired.
    #[error("session expired")]
    SessionExpired,

    /// The caller is not authorized to perform the requested action.
    #[error("unauthorized")]
    Unauthorized,
}
