//! Database schema definitions and migration logic.

use crate::error::StorageError;
use crate::Result;
use rusqlite::Connection;

/// Current schema version.
pub const SCHEMA_VERSION: u32 = 1;

/// Create all application tables and indexes.
///
/// All statements use `CREATE TABLE IF NOT EXISTS` and `CREATE INDEX IF NOT
/// EXISTS` so calling this function is safe to repeat.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(CREATE_ALL_TABLES_SQL)
        .map_err(|e| StorageError::MigrationFailed(e.to_string()))?;
    Ok(())
}

/// Combined DDL for every table and index in the Gupt local database.
const CREATE_ALL_TABLES_SQL: &str = r#"
-- ─── Schema Version ───
CREATE TABLE IF NOT EXISTS schema_version (
    version     INTEGER NOT NULL
);

-- ─── Identity ───
CREATE TABLE IF NOT EXISTS identity (
    id                      TEXT PRIMARY KEY NOT NULL,
    user_id                 TEXT NOT NULL UNIQUE,
    device_id               TEXT NOT NULL,
    signing_public_key      BLOB NOT NULL,
    encryption_public_key   BLOB NOT NULL,
    encrypted_signing_secret    BLOB NOT NULL,
    encrypted_encryption_secret BLOB NOT NULL,
    key_salt                BLOB NOT NULL,
    created_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ─── Conversations ───
CREATE TABLE IF NOT EXISTS conversations (
    id              TEXT PRIMARY KEY NOT NULL,
    display_name    TEXT,
    is_group        INTEGER NOT NULL DEFAULT 0,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    last_message_at TEXT,
    participant_ids TEXT NOT NULL DEFAULT '[]'
);
CREATE INDEX IF NOT EXISTS idx_conversations_updated ON conversations (updated_at);

-- ─── Messages ───
CREATE TABLE IF NOT EXISTS messages (
    id                  TEXT PRIMARY KEY NOT NULL,
    conversation_id     TEXT NOT NULL,
    sender_id           TEXT NOT NULL,
    recipient_id        TEXT NOT NULL,
    message_type        TEXT NOT NULL DEFAULT 'Text',
    encrypted_payload   BLOB NOT NULL,
    signature           BLOB NOT NULL,
    nonce               BLOB NOT NULL,
    ttl_seconds         INTEGER NOT NULL DEFAULT 86400,
    hop_count           INTEGER NOT NULL DEFAULT 0,
    max_hops            INTEGER NOT NULL DEFAULT 5,
    delivery_status     TEXT NOT NULL DEFAULT 'Pending',
    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    delivered_at        TEXT,
    read_at             TEXT,
    FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages (conversation_id, created_at);
CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages (sender_id);
CREATE INDEX IF NOT EXISTS idx_messages_status ON messages (delivery_status);

-- ─── Known Peers ───
CREATE TABLE IF NOT EXISTS known_peers (
    id                      TEXT PRIMARY KEY NOT NULL,
    display_name            TEXT,
    signing_public_key      BLOB,
    encryption_public_key   BLOB,
    last_seen_at            TEXT,
    last_transport          TEXT,
    trust_score             REAL NOT NULL DEFAULT 0.5,
    is_blocked              INTEGER NOT NULL DEFAULT 0,
    metadata                TEXT NOT NULL DEFAULT '{}',
    created_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_known_peers_trust ON known_peers (trust_score);

-- ─── Mesh Queue ───
CREATE TABLE IF NOT EXISTS mesh_queue (
    id              TEXT PRIMARY KEY NOT NULL,
    envelope_data   BLOB NOT NULL,
    target_peer_id  TEXT NOT NULL,
    priority        INTEGER NOT NULL DEFAULT 0,
    retry_count     INTEGER NOT NULL DEFAULT 0,
    max_retries     INTEGER NOT NULL DEFAULT 10,
    expires_at      TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_mesh_queue_priority ON mesh_queue (priority DESC, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_mesh_queue_expires ON mesh_queue (expires_at);

-- ─── Pending Transfers ───
CREATE TABLE IF NOT EXISTS pending_transfers (
    id                  TEXT PRIMARY KEY NOT NULL,
    message_id          TEXT NOT NULL,
    file_path           TEXT NOT NULL,
    file_size_bytes     INTEGER NOT NULL,
    transferred_bytes   INTEGER NOT NULL DEFAULT 0,
    checksum            TEXT NOT NULL,
    transport_type      TEXT NOT NULL,
    status              TEXT NOT NULL DEFAULT 'Pending',
    created_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at          TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_pending_transfers_status ON pending_transfers (status);

-- ─── Delivered Messages (dedup cache) ───
CREATE TABLE IF NOT EXISTS delivered_messages (
    message_id      TEXT PRIMARY KEY NOT NULL,
    delivered_at    TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ─── Attachments ───
CREATE TABLE IF NOT EXISTS attachments (
    id              TEXT PRIMARY KEY NOT NULL,
    message_id      TEXT NOT NULL,
    file_name       TEXT NOT NULL,
    mime_type       TEXT NOT NULL,
    size_bytes      INTEGER NOT NULL,
    encrypted_blob  BLOB,
    storage_path    TEXT,
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_attachments_message ON attachments (message_id);

-- ─── Device Settings ───
CREATE TABLE IF NOT EXISTS device_settings (
    key         TEXT PRIMARY KEY NOT NULL,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- ─── Trust Scores ───
CREATE TABLE IF NOT EXISTS trust_scores (
    peer_id                 TEXT PRIMARY KEY NOT NULL,
    score                   REAL NOT NULL DEFAULT 0.5,
    successful_transfers    INTEGER NOT NULL DEFAULT 0,
    failed_transfers        INTEGER NOT NULL DEFAULT 0,
    total_encounters        INTEGER NOT NULL DEFAULT 0,
    relay_reliability       REAL NOT NULL DEFAULT 0.0,
    last_encounter          TEXT,
    spam_flags              INTEGER NOT NULL DEFAULT 0,
    created_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at              TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    FOREIGN KEY (peer_id) REFERENCES known_peers (id) ON DELETE CASCADE
);

-- ─── Audit Log ───
CREATE TABLE IF NOT EXISTS audit_log (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type  TEXT NOT NULL,
    actor_id    TEXT,
    target_id   TEXT,
    details     TEXT NOT NULL DEFAULT '{}',
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_audit_log_type ON audit_log (event_type);
CREATE INDEX IF NOT EXISTS idx_audit_log_created ON audit_log (created_at);
"#;
