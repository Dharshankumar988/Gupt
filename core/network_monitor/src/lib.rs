//! # gupt-network-monitor
//!
//! Network state monitoring for the Gupt platform.

use gupt_common::TransportType;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during network monitoring.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NetworkMonitorError {
    /// Monitoring not available.
    #[error("network monitoring not available: {0}")]
    NotAvailable(String),
}

/// Connectivity status of the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConnectivityStatus {
    /// No network connectivity.
    Offline,
    /// Connected via Wi-Fi.
    Wifi,
    /// Connected via mobile data.
    MobileData,
    /// Connected via Ethernet.
    Ethernet,
}

/// Snapshot of the current network state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    /// Current connectivity status.
    pub connectivity: ConnectivityStatus,
    /// Whether internet is reachable.
    pub internet_available: bool,
    /// Whether Bluetooth LE is available.
    pub ble_available: bool,
    /// Whether Wi-Fi Direct is available.
    pub wifi_direct_available: bool,
    /// Current Wi-Fi RSSI (if connected).
    pub wifi_rssi: Option<i16>,
    /// Whether the device is in airplane mode.
    pub airplane_mode: bool,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            connectivity: ConnectivityStatus::Offline,
            internet_available: false,
            ble_available: false,
            wifi_direct_available: false,
            wifi_rssi: None,
            airplane_mode: false,
        }
    }
}

/// Network monitor that tracks device connectivity.
#[derive(Debug)]
pub struct NetworkMonitor {
    /// Current network state.
    state: NetworkState,
}

impl NetworkMonitor {
    /// Creates a new network monitor with default (offline) state.
    pub fn new() -> Self {
        Self {
            state: NetworkState::default(),
        }
    }

    /// Returns the current network state.
    pub fn state(&self) -> &NetworkState {
        &self.state
    }

    /// Updates the network state.
    pub fn update_state(&mut self, state: NetworkState) {
        self.state = state;
    }

    /// Checks if a given transport type is currently available.
    pub fn is_transport_available(&self, transport: TransportType) -> bool {
        match transport {
            TransportType::Ble => self.state.ble_available,
            TransportType::WifiDirect => self.state.wifi_direct_available,
            TransportType::InternetRelay => self.state.internet_available,
            TransportType::Mesh => self.state.ble_available || self.state.wifi_direct_available,
            _ => false,
        }
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}
