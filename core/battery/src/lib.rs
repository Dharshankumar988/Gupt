//! # gupt-battery
//!
//! Battery-aware power management for the Gupt platform.

use gupt_common::BatteryState;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during battery monitoring.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BatteryError {
    /// Battery monitoring is not available on this platform.
    #[error("battery monitoring not available: {0}")]
    NotAvailable(String),
}

/// Power profile for adapting behavior based on battery state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PowerProfile {
    /// Full performance, no restrictions.
    Performance,
    /// Balanced power usage.
    Balanced,
    /// Aggressive power saving.
    PowerSaver,
    /// Critical battery level, minimal operations.
    Critical,
}

/// Battery monitor that tracks device power state.
#[derive(Debug)]
pub struct BatteryMonitor {
    /// Current battery state.
    state: BatteryState,
    /// Active power profile.
    profile: PowerProfile,
}

impl BatteryMonitor {
    /// Creates a new battery monitor with default state.
    pub fn new() -> Self {
        Self {
            state: BatteryState::default(),
            profile: PowerProfile::Balanced,
        }
    }

    /// Returns the current battery state.
    pub fn state(&self) -> &BatteryState {
        &self.state
    }

    /// Updates the battery state.
    pub fn update_state(&mut self, state: BatteryState) {
        self.state = state;
        self.profile = Self::compute_profile(&state);
    }

    /// Returns the current power profile.
    pub fn profile(&self) -> PowerProfile {
        self.profile
    }

    /// Computes the appropriate power profile for a given battery state.
    fn compute_profile(state: &BatteryState) -> PowerProfile {
        if state.is_charging {
            return PowerProfile::Performance;
        }
        if state.power_save_mode || state.level_percent < 10 {
            return PowerProfile::Critical;
        }
        if state.level_percent < 20 {
            return PowerProfile::PowerSaver;
        }
        if state.level_percent < 50 {
            return PowerProfile::Balanced;
        }
        PowerProfile::Performance
    }
}

impl Default for BatteryMonitor {
    fn default() -> Self {
        Self::new()
    }
}
