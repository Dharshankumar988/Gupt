//! Shared type definitions for the Gupt platform.
//!
//! Contains newtype wrappers, enums, and core data structures used
//! across all Gupt crates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

// ──────────────────────────── Newtype Wrappers ────────────────────────────

/// Unique identifier for a message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub Uuid);

impl MessageId {
    /// Create a new random `MessageId`.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MessageId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

impl AsRef<Uuid> for MessageId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

/// Unique identifier for a user.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for UserId {
    fn from(v: &str) -> Self {
        Self(v.to_owned())
    }
}

impl AsRef<String> for UserId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

/// Unique identifier for a device.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for DeviceId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for DeviceId {
    fn from(v: &str) -> Self {
        Self(v.to_owned())
    }
}

impl AsRef<String> for DeviceId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

/// Unique identifier for a peer in the mesh network.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(pub String);

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PeerId {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<&str> for PeerId {
    fn from(v: &str) -> Self {
        Self(v.to_owned())
    }
}

impl AsRef<String> for PeerId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

/// Unique identifier for a conversation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversationId(pub Uuid);

impl ConversationId {
    /// Create a new random `ConversationId`.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ConversationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConversationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ConversationId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

impl AsRef<Uuid> for ConversationId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

/// UTC timestamp wrapper.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(pub chrono::DateTime<chrono::Utc>);

impl Timestamp {
    /// Return the current UTC time.
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(v: chrono::DateTime<chrono::Utc>) -> Self {
        Self(v)
    }
}

impl AsRef<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

/// Throughput measurement in bytes per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BytesPerSecond(pub u64);

impl fmt::Display for BytesPerSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} B/s", self.0)
    }
}

impl From<u64> for BytesPerSecond {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl AsRef<u64> for BytesPerSecond {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

// ──────────────────────────── Enums ────────────────────────────

/// The content type of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MessageType {
    /// Plain text message.
    Text,
    /// Image attachment.
    Image,
    /// Generic file attachment.
    File,
    /// Audio clip.
    Audio,
    /// Video clip.
    Video,
    /// GPS location.
    Location,
}

/// Delivery lifecycle of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeliveryStatus {
    /// Queued locally, not yet sent.
    Pending,
    /// Sent to at least one relay / peer.
    Sent,
    /// Confirmed received by recipient device.
    Delivered,
    /// Recipient has read the message.
    Read,
    /// Delivery failed permanently.
    Failed,
    /// TTL expired before delivery.
    Expired,
}

/// Available transport mechanisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TransportType {
    /// Bluetooth Low Energy.
    Ble,
    /// WiFi Direct / P2P.
    WifiDirect,
    /// Cloud relay over the internet.
    InternetRelay,
    /// Multi-hop mesh forwarding.
    Mesh,
}

/// State of a transport connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnectionState {
    /// No active connection.
    Disconnected,
    /// Connection attempt in progress.
    Connecting,
    /// Connection established and usable.
    Connected,
    /// Graceful shutdown in progress.
    Disconnecting,
}

/// Estimated battery cost of a transport operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BatteryCost {
    /// Minimal battery impact.
    VeryLow,
    /// Low battery impact.
    Low,
    /// Moderate battery impact.
    Medium,
    /// Significant battery impact.
    High,
    /// Severe battery impact.
    VeryHigh,
}

/// Device motion classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeviceMotion {
    /// Device is stationary.
    Stationary,
    /// Device is in motion.
    Moving,
    /// Motion state is unknown.
    Unknown,
}

// ──────────────────────────── Structs ────────────────────────────

/// Radio signal quality measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalQuality {
    /// Received Signal Strength Indicator (dBm).
    pub rssi: i16,
    /// Normalised quality percentage (0–100).
    pub quality_percent: u8,
}

/// Current state of the device battery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatteryState {
    /// Battery level as a percentage (0–100).
    pub level_percent: u8,
    /// Whether the device is currently charging.
    pub is_charging: bool,
    /// Whether battery-saver mode is enabled.
    pub is_saver_mode: bool,
}

/// An encrypted message ready for transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    /// Unique message identifier.
    pub id: MessageId,
    /// Sender user identifier.
    pub sender: UserId,
    /// Recipient user identifier.
    pub recipient: UserId,
    /// Conversation this message belongs to.
    pub conversation_id: ConversationId,
    /// Content type of the message.
    pub message_type: MessageType,
    /// Encrypted message payload bytes.
    pub encrypted_payload: Vec<u8>,
    /// Cryptographic signature over the envelope.
    pub signature: Vec<u8>,
    /// Nonce used during encryption.
    pub nonce: Vec<u8>,
    /// Time-to-live in seconds before the message expires.
    pub ttl_seconds: u32,
    /// Number of hops this envelope has already traversed.
    pub hop_count: u8,
    /// Maximum number of hops allowed.
    pub max_hops: u8,
    /// Timestamp when the message was created.
    pub timestamp: Timestamp,
    /// Current delivery status.
    pub delivery_status: DeliveryStatus,
}

/// Scored transport option for the routing engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportScore {
    /// The transport being scored.
    pub transport_type: TransportType,
    /// Overall composite score (higher is better).
    pub score: f64,
    /// Individual scoring factors and their weights.
    pub factors: HashMap<String, f64>,
}
