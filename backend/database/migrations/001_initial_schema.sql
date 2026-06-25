-- 001_initial_schema.sql
-- Gupt — Initial database schema for Supabase PostgreSQL.

-- =========================================================================
-- Tables
-- =========================================================================

CREATE TABLE IF NOT EXISTS users (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    username        VARCHAR(50) UNIQUE NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS public_keys (
    id                      UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id                 UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    signing_public_key      TEXT        NOT NULL,
    encryption_public_key   TEXT        NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    rotated_at              TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
    id                  UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id           VARCHAR(255) NOT NULL,
    refresh_token_hash  VARCHAR(64)  NOT NULL,
    expires_at          TIMESTAMPTZ  NOT NULL,
    revoked             BOOLEAN      NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS device_registrations (
    id                UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id           UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_name       VARCHAR(255) NOT NULL,
    device_type       VARCHAR(50)  NOT NULL,
    device_public_key TEXT         NOT NULL,
    push_token        TEXT,
    last_seen         TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS encrypted_relay_queue (
    id                  UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id           UUID        REFERENCES users(id),
    recipient_id        UUID        REFERENCES users(id),
    encrypted_payload   BYTEA       NOT NULL,
    packet_signature    BYTEA       NOT NULL,
    ttl                 INTEGER     NOT NULL DEFAULT 86400,
    status              VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cloud_backups (
    id                      UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id                 UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,
    encrypted_backup_blob   BYTEA       NOT NULL,
    backup_version          INTEGER     NOT NULL DEFAULT 1,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =========================================================================
-- Indexes
-- =========================================================================

CREATE INDEX IF NOT EXISTS idx_users_username
    ON users (username);

CREATE INDEX IF NOT EXISTS idx_public_keys_user_id
    ON public_keys (user_id);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_hash
    ON refresh_tokens (refresh_token_hash);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id
    ON refresh_tokens (user_id);

CREATE INDEX IF NOT EXISTS idx_device_registrations_user_id
    ON device_registrations (user_id);

CREATE INDEX IF NOT EXISTS idx_relay_queue_recipient_status
    ON encrypted_relay_queue (recipient_id, status);

CREATE INDEX IF NOT EXISTS idx_relay_queue_created_at
    ON encrypted_relay_queue (created_at);

CREATE INDEX IF NOT EXISTS idx_cloud_backups_user_id
    ON cloud_backups (user_id);

-- =========================================================================
-- Trigger: auto-update `updated_at` column
-- =========================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_cloud_backups_updated_at
    BEFORE UPDATE ON cloud_backups
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
