use chrono::Utc;
use gupt_common::types::{MessageId, PeerId, Timestamp, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Message TTL expired: {0}")]
    TTLExpired(MessageId),
    #[error("Maximum hops reached: {0}")]
    MaxHopsReached(MessageId),
    #[error("Duplicate message: {0}")]
    DuplicateMessage(MessageId),
    #[error("No relay available")]
    NoRelayAvailable,
    #[error("Relay failed: {0}")]
    RelayFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshPacket {
    pub message_id: MessageId,
    pub sender: UserId,
    pub recipient: UserId,
    pub encrypted_payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
    pub ttl_seconds: u32,
    pub created_at: Timestamp,
    pub hop_count: u8,
    pub max_hops: u8,
    pub relay_path: Vec<PeerId>,
}

impl MeshPacket {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let created = self.created_at.as_ref().clone();
        let age_seconds = now.signed_duration_since(created).num_seconds();
        age_seconds > self.ttl_seconds as i64
    }

    pub fn can_hop(&self) -> bool {
        self.hop_count < self.max_hops && !self.is_expired()
    }

    pub fn increment_hop(&mut self, relay: PeerId) {
        self.hop_count += 1;
        self.relay_path.push(relay);
    }
}

pub enum MeshAction {
    DeliverLocally(MeshPacket),
    RelayForward(MeshPacket),
    Duplicate,
    Expired,
    MaxHopsReached,
}

pub struct MeshEngine {
    seen_messages: HashSet<MessageId>,
    queued_packets: Vec<MeshPacket>,
}

impl MeshEngine {
    pub fn new() -> Self {
        Self {
            seen_messages: HashSet::new(),
            queued_packets: Vec::new(),
        }
    }

    pub fn queue_for_mesh(&mut self, packet: MeshPacket) -> Result<(), MeshError> {
        if packet.is_expired() {
            return Err(MeshError::TTLExpired(packet.message_id.clone()));
        }
        if !packet.can_hop() {
            return Err(MeshError::MaxHopsReached(packet.message_id.clone()));
        }

        self.queued_packets.push(packet);
        Ok(())
    }

    pub fn is_duplicate(&self, id: &MessageId) -> bool {
        self.seen_messages.contains(id)
    }

    pub fn mark_seen(&mut self, id: &MessageId) {
        self.seen_messages.insert(id.clone());
    }

    pub fn process_incoming(&mut self, packet: MeshPacket, my_user_id: &UserId) -> MeshAction {
        if self.is_duplicate(&packet.message_id) {
            return MeshAction::Duplicate;
        }

        self.mark_seen(&packet.message_id);

        if packet.is_expired() {
            return MeshAction::Expired;
        }

        if packet.recipient.as_ref() == my_user_id.as_ref() {
            return MeshAction::DeliverLocally(packet);
        }

        if !packet.can_hop() {
            return MeshAction::MaxHopsReached;
        }

        MeshAction::RelayForward(packet)
    }

    pub fn get_pending_for_relay(&self, peer: &PeerId) -> Vec<&MeshPacket> {
        self.queued_packets
            .iter()
            .filter(|p| !p.relay_path.contains(peer) && !p.is_expired() && p.can_hop())
            .collect()
    }

    pub fn remove_delivered(&mut self, id: &MessageId) {
        self.queued_packets.retain(|p| p.message_id.as_ref() != id.as_ref());
    }

    pub fn purge_expired(&mut self) -> usize {
        let initial_len = self.queued_packets.len();
        self.queued_packets.retain(|p| !p.is_expired());
        initial_len - self.queued_packets.len()
    }
}

impl Default for MeshEngine {
    fn default() -> Self {
        Self::new()
    }
}
