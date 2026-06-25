//! Session management for authenticated identity access.

use gupt_common::{DeviceId, Timestamp};
use serde::{Deserialize, Serialize};

/// An authenticated session bound to a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Opaque access token for the current session.
    pub access_token: String,
    /// Refresh token used to obtain a new access token.
    pub refresh_token: String,
    /// When the access token expires.
    pub expires_at: Timestamp,
    /// The device this session is bound to.
    pub device_id: DeviceId,
}

impl Session {
    /// Returns `true` if the session has expired.
    pub fn is_expired(&self) -> bool {
        Timestamp::now().0 >= self.expires_at.0
    }

    /// Returns `true` if the session will expire within the next 5 minutes
    /// and should be refreshed proactively.
    pub fn needs_refresh(&self) -> bool {
        let five_minutes = chrono::Duration::minutes(5);
        let refresh_threshold = self
            .expires_at
            .0
            .checked_sub_signed(five_minutes)
            .unwrap_or(self.expires_at.0);
        Timestamp::now().0 >= refresh_threshold
    }
}
