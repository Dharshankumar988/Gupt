//! # Gupt Database
//!
//! Provides connection pool management and migration running for the
//! Gupt backend's Supabase PostgreSQL database.

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Errors that can occur during database operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DatabaseError {
    /// Failed to establish a connection to the database.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    /// Failed to run database migrations.
    #[error("migration failed: {0}")]
    MigrationFailed(String),

    /// The connection pool has been exhausted.
    #[error("connection pool exhausted")]
    PoolExhausted,
}

/// Configuration required to connect to the database.
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// The full PostgreSQL connection URL.
    pub database_url: String,
    /// Maximum number of connections in the pool.
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: String::new(),
            max_connections: 5,
        }
    }
}

/// Creates a PostgreSQL connection pool from the given configuration.
///
/// # Errors
///
/// Returns [`DatabaseError::ConnectionFailed`] if the pool cannot be created.
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, DatabaseError> {
    tracing::info!(
        max_connections = config.max_connections,
        "creating database connection pool"
    );

    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))
}

/// Runs embedded SQLx migrations against the given pool.
///
/// Migrations are embedded at compile time from the `migrations/` directory
/// relative to this crate's `Cargo.toml`.
///
/// # Errors
///
/// Returns [`DatabaseError::MigrationFailed`] if any migration fails.
pub async fn run_migrations(pool: &PgPool) -> Result<(), DatabaseError> {
    tracing::info!("running database migrations");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))
}

/// Loads a [`DatabaseConfig`] from environment variables.
///
/// Reads `DATABASE_URL` from the environment (or a `.env` file via dotenvy).
/// Optionally reads `DATABASE_MAX_CONNECTIONS` (defaults to 5).
///
/// # Errors
///
/// Returns [`DatabaseError::ConnectionFailed`] if `DATABASE_URL` is not set.
pub fn load_config_from_env() -> Result<DatabaseConfig, DatabaseError> {
    // Attempt to load .env file; ignore errors if it doesn't exist.
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| DatabaseError::ConnectionFailed("DATABASE_URL is not set".to_string()))?;

    let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(5);

    Ok(DatabaseConfig {
        database_url,
        max_connections,
    })
}
