use gupt_common::error::GuptError as CoreGuptError;
use gupt_crypto::error::CryptoError;
use gupt_identity::error::IdentityError;
use gupt_storage::error::StorageError;

// The flat error enum defined in gupt.udl
#[derive(Debug, thiserror::Error)]
pub enum GuptError {
    #[error("Cryptography Error: {0}")]
    Crypto(String),
    #[error("Storage Error: {0}")]
    Storage(String),
    #[error("Transport Error: {0}")]
    Transport(String),
    #[error("Identity Error: {0}")]
    Identity(String),
    #[error("Network Error: {0}")]
    Network(String),
    #[error("Serialization Error: {0}")]
    Serialization(String),
    #[error("Invalid Input: {0}")]
    InvalidInput(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal Error: {0}")]
    Internal(String),
}

impl From<CoreGuptError> for GuptError {
    fn from(err: CoreGuptError) -> Self {
        match err {
            CoreGuptError::Crypto(msg) => GuptError::Crypto(msg),
            CoreGuptError::Storage(msg) => GuptError::Storage(msg),
            CoreGuptError::Transport(msg) => GuptError::Transport(msg),
            CoreGuptError::Identity(msg) => GuptError::Identity(msg),
            CoreGuptError::Network(msg) => GuptError::Network(msg),
            CoreGuptError::Serialization(msg) => GuptError::Serialization(msg),
            CoreGuptError::InvalidInput(msg) => GuptError::InvalidInput(msg),
            CoreGuptError::NotFound(msg) => GuptError::NotFound(msg),
            CoreGuptError::Unauthorized(msg) => GuptError::Unauthorized(msg),
            CoreGuptError::Internal(msg) => GuptError::Internal(msg),
        }
    }
}

impl From<IdentityError> for GuptError {
    fn from(err: IdentityError) -> Self {
        match err {
            IdentityError::InvalidPin => GuptError::Unauthorized("Invalid PIN".to_string()),
            IdentityError::Unauthorized => GuptError::Unauthorized("Unauthorized access".to_string()),
            _ => GuptError::Identity(err.to_string()),
        }
    }
}

impl From<StorageError> for GuptError {
    fn from(err: StorageError) -> Self {
        match err {
            StorageError::NotFound(msg) => GuptError::NotFound(msg),
            _ => GuptError::Storage(err.to_string()),
        }
    }
}

impl From<CryptoError> for GuptError {
    fn from(err: CryptoError) -> Self {
        GuptError::Crypto(err.to_string())
    }
}
