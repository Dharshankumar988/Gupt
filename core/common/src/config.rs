//! Global configuration for the Gupt platform.

use serde::{Deserialize, Serialize};

/// Application-wide configuration.
///
/// Sensible defaults are provided via the [`Default`] implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuptConfig {
    /// Path to the local SQLCipher database file.
    pub db_path: String,
    /// Maximum number of hops a message may traverse in the mesh.
    pub max_hops: u8,
    /// Default time-to-live for messages (seconds).
    pub default_ttl_seconds: u32,
    /// BLE/WiFi-Direct scan interval while the device is moving (ms).
    pub scan_interval_moving_ms: u64,
    /// BLE/WiFi-Direct scan interval while the device is stationary (ms).
    pub scan_interval_stationary_ms: u64,
    /// Whether to prefer local (BLE / WiFi-Direct) connections over internet.
    pub prefer_local_connections: bool,
    /// Whether to allow mobile data for relay traffic.
    pub allow_mobile_data: bool,
    /// Whether mesh forwarding is enabled.
    pub mesh_enabled: bool,
    /// Maximum number of concurrent file transfers.
    pub max_concurrent_transfers: usize,
}

impl Default for GuptConfig {
    fn default() -> Self {
        Self {
            db_path: "gupt.db".to_owned(),
            max_hops: 5,
            default_ttl_seconds: 86_400,
            scan_interval_moving_ms: 15_000,
            scan_interval_stationary_ms: 300_000,
            prefer_local_connections: true,
            allow_mobile_data: false,
            mesh_enabled: true,
            max_concurrent_transfers: 3,
        }
    }
}
