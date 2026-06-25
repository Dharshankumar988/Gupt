//! Repository trait definitions for data access.
//!
//! These traits define the contract for database operations. Concrete
//! implementations will be provided once the storage layer is fully wired.

use crate::Result;
use gupt_common::{
    ConversationId, DeliveryStatus, MessageEnvelope, MessageId, PeerId,
};

/// Persistence operations for messages.
pub trait MessageRepository {
    /// Persist a new message.
    fn save(&self, envelope: &MessageEnvelope) -> Result<()>;

    /// Retrieve a message by its unique identifier.
    fn get_by_id(&self, id: &MessageId) -> Result<MessageEnvelope>;

    /// List all messages in a conversation, ordered by timestamp ascending.
    fn get_by_conversation(&self, conversation_id: &ConversationId) -> Result<Vec<MessageEnvelope>>;

    /// Update the delivery status of a message.
    fn update_status(&self, id: &MessageId, status: DeliveryStatus) -> Result<()>;

    /// Delete a message by its unique identifier.
    fn delete(&self, id: &MessageId) -> Result<()>;
}

/// Persistence operations for conversations.
pub trait ConversationRepository {
    /// Create a new conversation and return its identifier.
    fn create(&self, display_name: Option<&str>, is_group: bool) -> Result<ConversationId>;

    /// Retrieve a conversation by identifier.
    fn get_by_id(&self, id: &ConversationId) -> Result<ConversationRecord>;

    /// List all conversations, most-recently-updated first.
    fn list_all(&self) -> Result<Vec<ConversationRecord>>;

    /// Delete a conversation and its messages.
    fn delete(&self, id: &ConversationId) -> Result<()>;
}

/// Persistence operations for known mesh peers.
pub trait PeerRepository {
    /// Insert or update a peer record.
    fn upsert(&self, peer: &PeerRecord) -> Result<()>;

    /// Retrieve a peer by identifier.
    fn get_by_id(&self, id: &PeerId) -> Result<PeerRecord>;

    /// List all known peers.
    fn list_all(&self) -> Result<Vec<PeerRecord>>;

    /// Remove a peer record.
    fn remove(&self, id: &PeerId) -> Result<()>;
}

/// Persistence operations for the mesh forwarding queue.
pub trait MeshQueueRepository {
    /// Add a message to the mesh forwarding queue.
    fn enqueue(&self, entry: &MeshQueueEntry) -> Result<()>;

    /// Dequeue up to `limit` entries, ordered by priority then age.
    fn dequeue_batch(&self, limit: usize) -> Result<Vec<MeshQueueEntry>>;

    /// Mark a queued message as delivered and remove it.
    fn mark_delivered(&self, id: &str) -> Result<()>;

    /// Remove all entries whose TTL has expired.
    fn remove_expired(&self) -> Result<u64>;
}

// ──────────────────────────── Supporting types ────────────────────────────

/// In-memory representation of a conversation row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConversationRecord {
    /// Conversation identifier.
    pub id: ConversationId,
    /// Human-readable name.
    pub display_name: Option<String>,
    /// Whether this is a group conversation.
    pub is_group: bool,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
    /// ISO-8601 last-updated timestamp.
    pub updated_at: String,
}

/// In-memory representation of a peer row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerRecord {
    /// Peer identifier.
    pub id: PeerId,
    /// Human-readable display name.
    pub display_name: Option<String>,
    /// Ed25519 public signing key.
    pub signing_public_key: Option<Vec<u8>>,
    /// X25519 public encryption key.
    pub encryption_public_key: Option<Vec<u8>>,
    /// Current trust score.
    pub trust_score: f64,
    /// Whether the peer is blocked.
    pub is_blocked: bool,
}

/// In-memory representation of a mesh queue entry.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeshQueueEntry {
    /// Queue entry identifier.
    pub id: String,
    /// Serialized envelope data.
    pub envelope_data: Vec<u8>,
    /// Target peer identifier.
    pub target_peer_id: String,
    /// Scheduling priority (higher = more urgent).
    pub priority: i32,
    /// Number of send attempts so far.
    pub retry_count: u32,
    /// Maximum allowed retries.
    pub max_retries: u32,
    /// ISO-8601 expiry timestamp.
    pub expires_at: String,
}
