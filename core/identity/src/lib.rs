//! # gupt-identity
//!
//! Identity creation, unlocking, and session management for the Gupt platform.

pub mod error;
pub mod manager;
pub mod session;

pub use error::IdentityError;
pub use manager::{Identity, IdentityManager};
pub use session::Session;

/// Convenience result type for identity operations.
pub type Result<T> = std::result::Result<T, IdentityError>;
