//! # Gupt Relay
//!
//! Encrypted relay message service for the Gupt backend. Provides validation,
//! storage, polling, acknowledgement, and expiry cleanup for end-to-end
//! encrypted messages transiting through the cloud relay.

use base64::Engine;
use gupt_models::{EncryptedRelayMessage, RelayMessageRequest};
use gupt_repositories::{PgRelayRepository, RelayRepository};
use sqlx::PgPool;
use uuid::Uuid;

/// Errors that can occur during relay operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RelayError {
    /// Failed to store a message in the relay queue.
    #[error("store failed: {0}")]
    StoreFailed(String),

    /// The intended recipient does not exist.
    #[error("recipient not found")]
    RecipientNotFound,

    /// The encrypted payload exceeds the maximum allowed size.
    #[error("payload too large: {size} bytes (max {max})")]
    PayloadTooLarge {
        /// Actual payload size in bytes.
        size: usize,
        /// Maximum allowed size in bytes.
        max: usize,
    },

    /// The recipient's relay queue is full.
    #[error("queue full")]
    QueueFull,

    /// Failed to deliver / acknowledge a message.
    #[error("delivery failed: {0}")]
    DeliveryFailed(String),
}

/// Configuration for the relay service.
#[derive(Debug, Clone)]
pub struct RelayConfig {
    /// Maximum payload size in bytes (default: 256 KB).
    pub max_payload_size: usize,
    /// Maximum number of pending messages per user (default: 1000).
    pub max_queue_per_user: usize,
    /// Default TTL in seconds for messages without an explicit TTL (default: 86400 = 24h).
    pub default_ttl_seconds: i32,
}

impl Default for RelayConfig {
    fn default() -> Self {
        Self {
            max_payload_size: 256 * 1024,
            max_queue_per_user: 1000,
            default_ttl_seconds: 86400,
        }
    }
}

/// The relay service responsible for managing encrypted message transit.
#[derive(Debug, Clone)]
pub struct RelayService {
    /// Active configuration.
    pub config: RelayConfig,
    /// Repository implementation for relay queue persistence.
    repo: PgRelayRepository,
}

impl RelayService {
    /// Creates a new [`RelayService`] with the given configuration.
    pub fn new(config: RelayConfig) -> Self {
        Self {
            config,
            repo: PgRelayRepository,
        }
    }

    /// Validates and stores an encrypted relay message.
    ///
    /// Decodes the base64 payload, checks the size limit, constructs the
    /// database record, and persists it. Returns the new message's UUID.
    ///
    /// # Errors
    ///
    /// Returns [`RelayError::PayloadTooLarge`] if the decoded payload exceeds
    /// the configured maximum, or [`RelayError::StoreFailed`] on database errors.
    pub async fn store_message(
        &self,
        pool: &PgPool,
        sender_id: Uuid,
        request: RelayMessageRequest,
    ) -> Result<Uuid, RelayError> {
        let engine = base64::engine::general_purpose::STANDARD;

        let payload_bytes = engine
            .decode(&request.encrypted_payload)
            .map_err(|e| RelayError::StoreFailed(format!("invalid base64 payload: {e}")))?;

        if payload_bytes.len() > self.config.max_payload_size {
            return Err(RelayError::PayloadTooLarge {
                size: payload_bytes.len(),
                max: self.config.max_payload_size,
            });
        }

        let signature_bytes = engine
            .decode(&request.packet_signature)
            .map_err(|e| RelayError::StoreFailed(format!("invalid base64 signature: {e}")))?;

        let ttl = request.ttl.unwrap_or(self.config.default_ttl_seconds);

        let message = EncryptedRelayMessage {
            id: Uuid::new_v4(),
            sender_id,
            recipient_id: request.recipient_id,
            encrypted_payload: payload_bytes,
            packet_signature: signature_bytes,
            ttl,
            status: "pending".to_string(),
            created_at: chrono::Utc::now(),
        };

        let stored = self
            .repo
            .store(pool, &message)
            .await
            .map_err(|e| RelayError::StoreFailed(e.to_string()))?;

        tracing::info!(
            message_id = %stored.id,
            sender = %sender_id,
            recipient = %request.recipient_id,
            "relay message stored"
        );

        Ok(stored.id)
    }

    /// Polls for pending messages addressed to the given recipient.
    ///
    /// # Errors
    ///
    /// Returns [`RelayError::DeliveryFailed`] on database errors.
    pub async fn poll_messages(
        &self,
        pool: &PgPool,
        recipient_id: Uuid,
    ) -> Result<Vec<EncryptedRelayMessage>, RelayError> {
        self.repo
            .poll_for_recipient(pool, recipient_id)
            .await
            .map_err(|e| RelayError::DeliveryFailed(e.to_string()))
    }

    /// Acknowledges receipt of a relay message, marking it as delivered.
    ///
    /// # Errors
    ///
    /// Returns [`RelayError::DeliveryFailed`] if the message cannot be found
    /// or the status update fails.
    pub async fn acknowledge(&self, pool: &PgPool, message_id: Uuid) -> Result<(), RelayError> {
        self.repo
            .mark_delivered(pool, message_id)
            .await
            .map_err(|e| RelayError::DeliveryFailed(e.to_string()))?;

        tracing::debug!(message_id = %message_id, "relay message acknowledged");
        Ok(())
    }

    /// Purges expired messages from the relay queue.
    ///
    /// Returns the number of messages removed.
    ///
    /// # Errors
    ///
    /// Returns [`RelayError::DeliveryFailed`] on database errors.
    pub async fn cleanup_expired(&self, pool: &PgPool) -> Result<u64, RelayError> {
        let count = self
            .repo
            .purge_expired(pool)
            .await
            .map_err(|e| RelayError::DeliveryFailed(e.to_string()))?;

        tracing::info!(purged = count, "expired relay messages cleaned up");
        Ok(count)
    }
}
