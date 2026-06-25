//! # Gupt Models
//!
//! Database models mapping to Supabase PostgreSQL tables and
//! request/response DTOs for the Gupt backend API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Database Models
// ---------------------------------------------------------------------------

/// Represents a registered user in the system.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Unique identifier for the user.
    pub id: Uuid,
    /// Human-readable username (unique).
    pub username: String,
    /// Timestamp when the user was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user record was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Stores a user's public keys used for signing and encryption.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PublicKey {
    /// Unique identifier for this key record.
    pub id: Uuid,
    /// The user who owns these keys.
    pub user_id: Uuid,
    /// Base64-encoded Ed25519 signing public key.
    pub signing_public_key: String,
    /// Base64-encoded X25519 encryption public key.
    pub encryption_public_key: String,
    /// Timestamp when the keys were first uploaded.
    pub created_at: DateTime<Utc>,
    /// Timestamp when the keys were last rotated, if ever.
    pub rotated_at: Option<DateTime<Utc>>,
}

/// A hashed refresh token tied to a specific user and device.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefreshToken {
    /// Unique identifier for this token record.
    pub id: Uuid,
    /// The user this token belongs to.
    pub user_id: Uuid,
    /// Identifier for the device that requested the token.
    pub device_id: String,
    /// SHA-256 hash of the actual refresh token.
    pub refresh_token_hash: String,
    /// When this token expires.
    pub expires_at: DateTime<Utc>,
    /// Whether this token has been revoked.
    pub revoked: bool,
}

/// A registered device for a user, used for push notifications and
/// device-specific key management.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeviceRegistration {
    /// Unique identifier for this device registration.
    pub id: Uuid,
    /// The user who owns this device.
    pub user_id: Uuid,
    /// Human-readable name for the device (e.g. "Pixel 8 Pro").
    pub device_name: String,
    /// Type of device (e.g. "android", "ios", "desktop").
    pub device_type: String,
    /// Base64-encoded device-specific public key.
    pub device_public_key: String,
    /// Optional push notification token (FCM / APNs).
    pub push_token: Option<String>,
    /// When this device was last seen online.
    pub last_seen: DateTime<Utc>,
}

/// An encrypted message stored in the relay queue for asynchronous delivery.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EncryptedRelayMessage {
    /// Unique identifier for this relay message.
    pub id: Uuid,
    /// The user who sent the message.
    pub sender_id: Uuid,
    /// The intended recipient.
    pub recipient_id: Uuid,
    /// The end-to-end encrypted payload (opaque to the server).
    pub encrypted_payload: Vec<u8>,
    /// Cryptographic signature over the packet.
    pub packet_signature: Vec<u8>,
    /// Time-to-live in seconds before the message is purged.
    pub ttl: i32,
    /// Delivery status: "pending", "delivered", "expired".
    pub status: String,
    /// When the message was stored on the relay.
    pub created_at: DateTime<Utc>,
}

/// An encrypted cloud backup for a user's local data.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CloudBackup {
    /// Unique identifier for this backup record.
    pub id: Uuid,
    /// The user who owns this backup.
    pub user_id: Uuid,
    /// The encrypted backup blob (opaque to the server).
    pub encrypted_backup_blob: Vec<u8>,
    /// Monotonically increasing version number.
    pub backup_version: i32,
    /// When this backup was first created.
    pub created_at: DateTime<Utc>,
    /// When this backup was last updated.
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Request / Response DTOs
// ---------------------------------------------------------------------------

/// Request body for creating a new user account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// Desired username for the new account.
    pub username: String,
}

/// Request body for logging in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    /// The username to authenticate.
    pub username: String,
    /// Signed challenge response proving key ownership.
    pub challenge_response: String,
    /// Identifier for the device performing the login.
    pub device_id: String,
}

/// Response body containing authentication tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Short-lived JWT access token.
    pub access_token: String,
    /// Long-lived refresh token (hex-encoded).
    pub refresh_token: String,
    /// Number of seconds until the access token expires.
    pub expires_in: i64,
}

/// Request body for uploading or rotating public keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyUpload {
    /// Base64-encoded Ed25519 signing public key.
    pub signing_public_key: String,
    /// Base64-encoded X25519 encryption public key.
    pub encryption_public_key: String,
}

/// Request body for sending an encrypted relay message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayMessageRequest {
    /// The UUID of the intended recipient.
    pub recipient_id: Uuid,
    /// Base64-encoded encrypted payload.
    pub encrypted_payload: String,
    /// Base64-encoded packet signature.
    pub packet_signature: String,
    /// Optional TTL override in seconds.
    pub ttl: Option<i32>,
}

/// Request body for registering a new device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRegistrationRequest {
    /// Human-readable name for the device.
    pub device_name: String,
    /// Type of device (e.g. "android", "ios", "desktop").
    pub device_type: String,
    /// Base64-encoded device-specific public key.
    pub device_public_key: String,
    /// Optional push notification token (FCM / APNs).
    pub push_token: Option<String>,
}
