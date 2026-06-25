//! # gupt-common
//!
//! Shared types, error definitions, and configuration used by all other
//! crates in the Gupt hybrid adaptive communication platform.

pub mod config;
pub mod error;
pub mod types;

pub use config::GuptConfig;
pub use error::{GuptError, Result};
pub use types::*;
