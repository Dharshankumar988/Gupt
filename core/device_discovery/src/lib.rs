use chrono::Utc;
use gupt_common::types::{PeerId, Timestamp, TransportType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiscoveryError {
    #[error("Scan failed: {0}")]
    ScanFailed(String),
    #[error("Advertise failed: {0}")]
    AdvertiseFailed(String),
    #[error("Transport not supported: {0:?}")]
    NotSupported(TransportType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCapabilities {
    pub supports_ble: bool,
    pub supports_wifi_direct: bool,
    pub supports_internet_relay: bool,
    pub supports_mesh: bool,
    pub max_payload_size: usize,
}

impl Default for PeerCapabilities {
    fn default() -> Self {
        Self {
            supports_ble: true,
            supports_wifi_direct: false,
            supports_internet_relay: false,
            supports_mesh: true,
            max_payload_size: 100_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    pub peer_id: PeerId,
    pub display_name: Option<String>,
    pub transport_type: TransportType,
    pub rssi: Option<i16>,
    pub discovered_at: Timestamp,
    pub last_seen: Timestamp,
    pub capabilities: PeerCapabilities,
}

pub struct DiscoveryManager {
    known_peers: HashMap<PeerId, DiscoveredPeer>,
    is_scanning: bool,
    is_advertising: bool,
}

impl DiscoveryManager {
    pub fn new() -> Self {
        Self {
            known_peers: HashMap::new(),
            is_scanning: false,
            is_advertising: false,
        }
    }

    pub fn on_peer_discovered(&mut self, mut peer: DiscoveredPeer) {
        peer.last_seen = Timestamp::from(Utc::now());
        self.known_peers.insert(peer.peer_id.clone(), peer);
    }

    pub fn on_peer_lost(&mut self, peer_id: &PeerId) {
        self.known_peers.remove(peer_id);
    }

    pub fn get_nearby_peers(&self) -> Vec<&DiscoveredPeer> {
        self.known_peers.values().collect()
    }

    pub fn get_peer(&self, id: &PeerId) -> Option<&DiscoveredPeer> {
        self.known_peers.get(id)
    }

    pub fn purge_stale(&mut self, max_age_seconds: u64) -> usize {
        let initial_len = self.known_peers.len();
        let now = Utc::now();
        self.known_peers.retain(|_, p| {
            let age = now.signed_duration_since(p.last_seen.as_ref().clone()).num_seconds();
            age <= max_age_seconds as i64
        });
        initial_len - self.known_peers.len()
    }

    pub fn set_scanning(&mut self, active: bool) {
        self.is_scanning = active;
    }

    pub fn set_advertising(&mut self, active: bool) {
        self.is_advertising = active;
    }

    pub fn is_scanning(&self) -> bool {
        self.is_scanning
    }

    pub fn is_advertising(&self) -> bool {
        self.is_advertising
    }
}

impl Default for DiscoveryManager {
    fn default() -> Self {
        Self::new()
    }
}
