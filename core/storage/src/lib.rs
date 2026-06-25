//! # gupt-storage
//!
//! SQLCipher-backed local storage for the Gupt platform.

pub mod database;
pub mod error;
pub mod repositories;
pub mod schema;

pub use database::Database;
pub use error::StorageError;

/// Convenience result type for storage operations.
pub type Result<T> = std::result::Result<T, StorageError>;
