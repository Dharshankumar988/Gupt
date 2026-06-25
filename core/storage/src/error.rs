//! Storage-specific error types.

use thiserror::Error;

/// Errors produced by storage operations.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StorageError {
    /// Opening the database failed.
    #[error("failed to open database: {0}")]
    DatabaseOpen(String),

    /// A SQL query failed.
    #[error("query failed: {0}")]
    QueryFailed(String),

    /// A schema migration failed.
    #[error("migration failed: {0}")]
    MigrationFailed(String),

    /// The requested record was not found.
    #[error("not found: {0}")]
    NotFound(String),

    /// Serialization or deserialization of a stored value failed.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// The database is locked by another process.
    #[error("database is locked")]
    DatabaseLocked,
}

impl From<rusqlite::Error> for StorageError {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::QueryReturnedNoRows => {
                StorageError::NotFound("query returned no rows".into())
            }
            _ => StorageError::QueryFailed(e.to_string()),
        }
    }
}
