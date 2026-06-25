//! SQLCipher database wrapper.

use crate::error::StorageError;
use crate::schema;
use crate::Result;
use rusqlite::Connection;

/// An encrypted SQLCipher database connection.
pub struct Database {
    /// The underlying rusqlite connection.
    conn: Option<Connection>,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("is_open", &self.conn.is_some())
            .finish()
    }
}

impl Database {
    /// Open (or create) an encrypted SQLCipher database at `path`.
    ///
    /// The `passphrase` is used as the SQLCipher encryption key. After opening,
    /// the connection is configured with hardened SQLCipher PRAGMAs and verified
    /// with a probe query.
    pub fn open(path: &str, passphrase: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| StorageError::DatabaseOpen(e.to_string()))?;

        // Set SQLCipher encryption key.
        conn.execute_batch(&format!("PRAGMA key = '{}';", passphrase))
            .map_err(|e| StorageError::DatabaseOpen(format!("PRAGMA key failed: {e}")))?;

        // Harden cipher settings.
        conn.execute_batch(
            "PRAGMA cipher_page_size = 4096;
             PRAGMA kdf_iter = 256000;
             PRAGMA cipher_memory_security = ON;",
        )
        .map_err(|e| StorageError::DatabaseOpen(format!("cipher pragmas failed: {e}")))?;

        // Verify the database is accessible (will fail if passphrase is wrong).
        conn.execute_batch("SELECT count(*) FROM sqlite_master;")
            .map_err(|e| StorageError::DatabaseOpen(format!("verification failed: {e}")))?;

        // Enable WAL mode for better concurrency.
        conn.execute_batch("PRAGMA journal_mode = WAL;")
            .map_err(|e| StorageError::DatabaseOpen(format!("WAL mode failed: {e}")))?;

        Ok(Self { conn: Some(conn) })
    }

    /// Close the database connection, consuming the `Database`.
    pub fn close(mut self) {
        if let Some(conn) = self.conn.take() {
            // Best-effort close — if it fails we drop the connection anyway.
            let _ = conn.close();
        }
    }

    /// Whether the database connection is currently open.
    pub fn is_open(&self) -> bool {
        self.conn.is_some()
    }

    /// Run all pending schema migrations.
    pub fn run_migrations(&self) -> Result<()> {
        let conn = self
            .conn
            .as_ref()
            .ok_or(StorageError::DatabaseOpen("database not open".into()))?;
        schema::create_tables(conn)?;
        Ok(())
    }

    /// Borrow the underlying connection for direct queries.
    ///
    /// Prefer using repository traits instead.
    pub fn connection(&self) -> Result<&Connection> {
        self.conn
            .as_ref()
            .ok_or(StorageError::DatabaseOpen("database not open".into()))
    }
}
