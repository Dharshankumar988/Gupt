//! # Gupt Repositories
//!
//! Repository layer providing typed database access for all Gupt backend
//! entities. Each repository trait defines the data-access contract while
//! the `Pg*` implementations supply real SQLx queries against PostgreSQL.

use async_trait::async_trait;
use chrono::Utc;
use gupt_models::{
    CloudBackup, CreateUserRequest, DeviceRegistration, DeviceRegistrationRequest,
    EncryptedRelayMessage, PublicKey, PublicKeyUpload, RefreshToken, User,
};
use sqlx::PgPool;
use uuid::Uuid;

/// Errors that can occur during repository operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RepositoryError {
    /// A database query failed.
    #[error("query failed: {0}")]
    QueryFailed(String),

    /// The requested entity was not found.
    #[error("not found")]
    NotFound,

    /// A unique constraint was violated (e.g. duplicate username).
    #[error("duplicate: {0}")]
    Duplicate(String),
}

// ---------------------------------------------------------------------------
// UserRepository
// ---------------------------------------------------------------------------

/// Data-access contract for the `users` table.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a new user and returns the created record.
    async fn create(
        &self,
        pool: &PgPool,
        request: &CreateUserRequest,
    ) -> Result<User, RepositoryError>;

    /// Finds a user by their primary key.
    async fn find_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Option<User>, RepositoryError>;

    /// Finds a user by their unique username.
    async fn find_by_username(
        &self,
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<User>, RepositoryError>;

    /// Touches the `updated_at` timestamp for the given user.
    async fn update_timestamp(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError>;
}

/// PostgreSQL implementation of [`UserRepository`].
#[derive(Debug, Clone)]
pub struct PgUserRepository;

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(
        &self,
        pool: &PgPool,
        request: &CreateUserRequest,
    ) -> Result<User, RepositoryError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username) VALUES ($1) RETURNING id, username, created_at, updated_at",
        )
        .bind(&request.username)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.is_unique_violation() {
                    return RepositoryError::Duplicate(format!(
                        "username '{}' already exists",
                        request.username
                    ));
                }
            }
            RepositoryError::QueryFailed(e.to_string())
        })
    }

    async fn find_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn find_by_username(
        &self,
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, created_at, updated_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn update_timestamp(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError> {
        let rows_affected = sqlx::query("UPDATE users SET updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?
            .rows_affected();

        if rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// PublicKeyRepository
// ---------------------------------------------------------------------------

/// Data-access contract for the `public_keys` table.
#[async_trait]
pub trait PublicKeyRepository: Send + Sync {
    /// Inserts or updates the public keys for a given user.
    async fn upsert(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        keys: &PublicKeyUpload,
    ) -> Result<PublicKey, RepositoryError>;

    /// Finds the public keys for a given user.
    async fn find_by_user_id(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<PublicKey>, RepositoryError>;
}

/// PostgreSQL implementation of [`PublicKeyRepository`].
#[derive(Debug, Clone)]
pub struct PgPublicKeyRepository;

#[async_trait]
impl PublicKeyRepository for PgPublicKeyRepository {
    async fn upsert(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        keys: &PublicKeyUpload,
    ) -> Result<PublicKey, RepositoryError> {
        sqlx::query_as::<_, PublicKey>(
            r#"
            INSERT INTO public_keys (user_id, signing_public_key, encryption_public_key)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE SET
                signing_public_key = EXCLUDED.signing_public_key,
                encryption_public_key = EXCLUDED.encryption_public_key,
                rotated_at = NOW()
            RETURNING id, user_id, signing_public_key, encryption_public_key, created_at, rotated_at
            "#,
        )
        .bind(user_id)
        .bind(&keys.signing_public_key)
        .bind(&keys.encryption_public_key)
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn find_by_user_id(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<PublicKey>, RepositoryError> {
        sqlx::query_as::<_, PublicKey>(
            "SELECT id, user_id, signing_public_key, encryption_public_key, created_at, rotated_at FROM public_keys WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// RefreshTokenRepository
// ---------------------------------------------------------------------------

/// Data-access contract for the `refresh_tokens` table.
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// Stores a new refresh token record.
    async fn create(
        &self,
        pool: &PgPool,
        token: &RefreshToken,
    ) -> Result<RefreshToken, RepositoryError>;

    /// Finds a non-revoked, non-expired refresh token by its hash.
    async fn find_valid(
        &self,
        pool: &PgPool,
        hash: &str,
    ) -> Result<Option<RefreshToken>, RepositoryError>;

    /// Revokes a single refresh token by its ID.
    async fn revoke(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError>;

    /// Revokes all refresh tokens belonging to a user.
    async fn revoke_all_for_user(&self, pool: &PgPool, user_id: Uuid)
        -> Result<(), RepositoryError>;
}

/// PostgreSQL implementation of [`RefreshTokenRepository`].
#[derive(Debug, Clone)]
pub struct PgRefreshTokenRepository;

#[async_trait]
impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn create(
        &self,
        pool: &PgPool,
        token: &RefreshToken,
    ) -> Result<RefreshToken, RepositoryError> {
        sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (user_id, device_id, refresh_token_hash, expires_at, revoked)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, device_id, refresh_token_hash, expires_at, revoked
            "#,
        )
        .bind(token.user_id)
        .bind(&token.device_id)
        .bind(&token.refresh_token_hash)
        .bind(token.expires_at)
        .bind(token.revoked)
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn find_valid(
        &self,
        pool: &PgPool,
        hash: &str,
    ) -> Result<Option<RefreshToken>, RepositoryError> {
        sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT id, user_id, device_id, refresh_token_hash, expires_at, revoked
            FROM refresh_tokens
            WHERE refresh_token_hash = $1
              AND revoked = FALSE
              AND expires_at > NOW()
            "#,
        )
        .bind(hash)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn revoke(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError> {
        let rows = sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?
            .rows_affected();

        if rows == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }

    async fn revoke_all_for_user(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<(), RepositoryError> {
        sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// DeviceRepository
// ---------------------------------------------------------------------------

/// Data-access contract for the `device_registrations` table.
#[async_trait]
pub trait DeviceRepository: Send + Sync {
    /// Registers a new device for a user.
    async fn register(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        request: &DeviceRegistrationRequest,
    ) -> Result<DeviceRegistration, RepositoryError>;

    /// Lists all devices registered to a user.
    async fn find_by_user(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<DeviceRegistration>, RepositoryError>;

    /// Unregisters (deletes) a device by its ID.
    async fn unregister(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError>;

    /// Updates the `last_seen` timestamp for a device.
    async fn update_last_seen(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError>;
}

/// PostgreSQL implementation of [`DeviceRepository`].
#[derive(Debug, Clone)]
pub struct PgDeviceRepository;

#[async_trait]
impl DeviceRepository for PgDeviceRepository {
    async fn register(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        request: &DeviceRegistrationRequest,
    ) -> Result<DeviceRegistration, RepositoryError> {
        sqlx::query_as::<_, DeviceRegistration>(
            r#"
            INSERT INTO device_registrations (user_id, device_name, device_type, device_public_key, push_token)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, device_name, device_type, device_public_key, push_token, last_seen
            "#,
        )
        .bind(user_id)
        .bind(&request.device_name)
        .bind(&request.device_type)
        .bind(&request.device_public_key)
        .bind(&request.push_token)
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn find_by_user(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<DeviceRegistration>, RepositoryError> {
        sqlx::query_as::<_, DeviceRegistration>(
            "SELECT id, user_id, device_name, device_type, device_public_key, push_token, last_seen FROM device_registrations WHERE user_id = $1 ORDER BY last_seen DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn unregister(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError> {
        let rows = sqlx::query("DELETE FROM device_registrations WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?
            .rows_affected();

        if rows == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }

    async fn update_last_seen(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError> {
        let rows =
            sqlx::query("UPDATE device_registrations SET last_seen = NOW() WHERE id = $1")
                .bind(id)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?
                .rows_affected();

        if rows == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// RelayRepository
// ---------------------------------------------------------------------------

/// Data-access contract for the `encrypted_relay_queue` table.
#[async_trait]
pub trait RelayRepository: Send + Sync {
    /// Stores an encrypted relay message.
    async fn store(
        &self,
        pool: &PgPool,
        message: &EncryptedRelayMessage,
    ) -> Result<EncryptedRelayMessage, RepositoryError>;

    /// Polls for pending messages addressed to the given recipient.
    async fn poll_for_recipient(
        &self,
        pool: &PgPool,
        recipient_id: Uuid,
    ) -> Result<Vec<EncryptedRelayMessage>, RepositoryError>;

    /// Marks a relay message as delivered.
    async fn mark_delivered(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError>;

    /// Purges expired messages from the queue. Returns the number of rows deleted.
    async fn purge_expired(&self, pool: &PgPool) -> Result<u64, RepositoryError>;
}

/// PostgreSQL implementation of [`RelayRepository`].
#[derive(Debug, Clone)]
pub struct PgRelayRepository;

#[async_trait]
impl RelayRepository for PgRelayRepository {
    async fn store(
        &self,
        pool: &PgPool,
        message: &EncryptedRelayMessage,
    ) -> Result<EncryptedRelayMessage, RepositoryError> {
        sqlx::query_as::<_, EncryptedRelayMessage>(
            r#"
            INSERT INTO encrypted_relay_queue (sender_id, recipient_id, encrypted_payload, packet_signature, ttl, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, sender_id, recipient_id, encrypted_payload, packet_signature, ttl, status, created_at
            "#,
        )
        .bind(message.sender_id)
        .bind(message.recipient_id)
        .bind(&message.encrypted_payload)
        .bind(&message.packet_signature)
        .bind(message.ttl)
        .bind(&message.status)
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn poll_for_recipient(
        &self,
        pool: &PgPool,
        recipient_id: Uuid,
    ) -> Result<Vec<EncryptedRelayMessage>, RepositoryError> {
        sqlx::query_as::<_, EncryptedRelayMessage>(
            r#"
            SELECT id, sender_id, recipient_id, encrypted_payload, packet_signature, ttl, status, created_at
            FROM encrypted_relay_queue
            WHERE recipient_id = $1
              AND status = 'pending'
              AND created_at + (ttl || ' seconds')::INTERVAL > NOW()
            ORDER BY created_at ASC
            "#,
        )
        .bind(recipient_id)
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn mark_delivered(&self, pool: &PgPool, id: Uuid) -> Result<(), RepositoryError> {
        let rows = sqlx::query(
            "UPDATE encrypted_relay_queue SET status = 'delivered' WHERE id = $1 AND status = 'pending'",
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?
        .rows_affected();

        if rows == 0 {
            return Err(RepositoryError::NotFound);
        }
        Ok(())
    }

    async fn purge_expired(&self, pool: &PgPool) -> Result<u64, RepositoryError> {
        let result = sqlx::query(
            "DELETE FROM encrypted_relay_queue WHERE created_at + (ttl || ' seconds')::INTERVAL <= NOW()",
        )
        .execute(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))?;

        Ok(result.rows_affected())
    }
}

// ---------------------------------------------------------------------------
// BackupRepository
// ---------------------------------------------------------------------------

/// Data-access contract for the `cloud_backups` table.
#[async_trait]
pub trait BackupRepository: Send + Sync {
    /// Inserts or updates a cloud backup for the given user.
    async fn upsert(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        blob: &[u8],
        version: i32,
    ) -> Result<CloudBackup, RepositoryError>;

    /// Finds the latest cloud backup for the given user.
    async fn find_latest(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<CloudBackup>, RepositoryError>;
}

/// PostgreSQL implementation of [`BackupRepository`].
#[derive(Debug, Clone)]
pub struct PgBackupRepository;

#[async_trait]
impl BackupRepository for PgBackupRepository {
    async fn upsert(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        blob: &[u8],
        version: i32,
    ) -> Result<CloudBackup, RepositoryError> {
        sqlx::query_as::<_, CloudBackup>(
            r#"
            INSERT INTO cloud_backups (user_id, encrypted_backup_blob, backup_version)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE SET
                encrypted_backup_blob = EXCLUDED.encrypted_backup_blob,
                backup_version = EXCLUDED.backup_version,
                updated_at = NOW()
            RETURNING id, user_id, encrypted_backup_blob, backup_version, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(blob)
        .bind(version)
        .fetch_one(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }

    async fn find_latest(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<CloudBackup>, RepositoryError> {
        sqlx::query_as::<_, CloudBackup>(
            "SELECT id, user_id, encrypted_backup_blob, backup_version, created_at, updated_at FROM cloud_backups WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryFailed(e.to_string()))
    }
}
