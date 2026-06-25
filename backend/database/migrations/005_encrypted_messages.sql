-- 005_encrypted_messages.sql
-- Structured cloud table for storing encrypted messages for sync and DTN routing.

CREATE TABLE IF NOT EXISTS encrypted_messages (
    id                  UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id     UUID        NOT NULL,
    sender_id           UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    recipient_id        UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    encrypted_payload   TEXT        NOT NULL, -- Base64 encoded XChaCha20-Poly1305 ciphertext
    nonce               TEXT        NOT NULL, -- Base64 encoded 24-byte nonce
    message_type        VARCHAR(50) NOT NULL DEFAULT 'text',
    transport_used      VARCHAR(50) NOT NULL DEFAULT 'CLOUD',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for quickly fetching messages for a specific conversation
CREATE INDEX IF NOT EXISTS idx_encrypted_messages_conversation
    ON encrypted_messages (conversation_id);

-- Index for fetching messages meant for a specific recipient (for synchronization)
CREATE INDEX IF NOT EXISTS idx_encrypted_messages_recipient
    ON encrypted_messages (recipient_id);

-- Add Realtime support for this table so the app can subscribe to inserts
ALTER PUBLICATION supabase_realtime ADD TABLE encrypted_messages;
