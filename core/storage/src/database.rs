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

    pub fn connection(&self) -> Result<&Connection> {
        self.conn
            .as_ref()
            .ok_or(StorageError::DatabaseOpen("database not open".into()))
    }
}

use crate::repositories::{PendingOutboxEntry, PendingOutboxRepository};
use rusqlite::params;

impl PendingOutboxRepository for Database {
    fn enqueue(&self, entry: &PendingOutboxEntry) -> Result<()> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO pending_outbox (id, conversation_id, sender_id, recipient_id, encrypted_payload, nonce, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                entry.id,
                entry.conversation_id,
                entry.sender_id,
                entry.recipient_id,
                entry.encrypted_payload,
                entry.nonce,
                entry.created_at,
            ],
        ).map_err(|e| StorageError::QueryFailed(e.to_string()))?;
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<PendingOutboxEntry>> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare("SELECT id, conversation_id, sender_id, recipient_id, encrypted_payload, nonce, created_at FROM pending_outbox")
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;
        
        let rows = stmt.query_map([], |row| {
            Ok(PendingOutboxEntry {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                sender_id: row.get(2)?,
                recipient_id: row.get(3)?,
                encrypted_payload: row.get(4)?,
                nonce: row.get(5)?,
                created_at: row.get(6)?,
            })
        }).map_err(|e| StorageError::QueryFailed(e.to_string()))?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row.map_err(|e| StorageError::QueryFailed(e.to_string()))?);
        }
        Ok(entries)
    }

    fn remove(&self, id: &str) -> Result<()> {
        let conn = self.connection()?;
        conn.execute("DELETE FROM pending_outbox WHERE id = ?1", params![id])
            .map_err(|e| StorageError::QueryFailed(e.to_string()))?;
        Ok(())
    }
}
