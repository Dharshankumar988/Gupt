use gupt_battery::BatteryState;
use gupt_common::types::{PeerId, TransportScore, TransportType};
use gupt_network_monitor::NetworkState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoutingError {
    #[error("No transport available")]
    NoTransportAvailable,
    #[error("All transports failed: {0:?}")]
    AllTransportsFailed(Vec<String>),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingWeight {
    pub proximity: f64,
    pub signal_quality: f64,
    pub latency: f64,
    pub throughput_match: f64,
    pub battery_cost: f64,
    pub reliability: f64,
    pub privacy: f64,
    pub congestion: f64,
    pub trust: f64,
}

impl Default for RoutingWeight {
    fn default() -> Self {
        Self {
            proximity: 0.15,
            signal_quality: 0.12,
            latency: 0.15,
            throughput_match: 0.10,
            battery_cost: 0.12,
            reliability: 0.10,
            privacy: 0.10,
            congestion: 0.08,
            trust: 0.08,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct UserRoutingPreferences {
    pub prefer_local: bool,
    pub allow_mobile_data: bool,
    pub mesh_enabled: bool,
    pub max_hops: u8,
    pub prefer_battery_saving: bool,
}

impl Default for UserRoutingPreferences {
    fn default() -> Self {
        Self {
            prefer_local: true,
            allow_mobile_data: false,
            mesh_enabled: true,
            max_hops: 5,
            prefer_battery_saving: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutingContext {
    pub target_peer: PeerId,
    pub payload_size: usize,
    pub message_priority: MessagePriority,
    pub battery_state: BatteryState,
    pub network_state: NetworkState,
    pub available_transports: Vec<TransportType>,
    pub peer_rssi: Option<i16>,
    pub peer_trust_score: f64,
    pub user_preferences: UserRoutingPreferences,
}

#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub primary_transport: TransportType,
    pub fallback_transports: Vec<TransportType>,
    pub use_multipath: bool,
    pub scores: Vec<TransportScore>,
}

pub struct RoutingEngine {
    weights: RoutingWeight,
}

impl RoutingEngine {
    pub fn new(weights: RoutingWeight) -> Self {
        Self { weights }
    }

    pub fn select_route(
        &self,
        context: &RoutingContext,
    ) -> Result<RoutingDecision, RoutingError> {
        if context.available_transports.is_empty() {
            return Err(RoutingError::NoTransportAvailable);
        }

        let mut scored_transports: Vec<TransportScore> = context
            .available_transports
            .iter()
            .map(|t| self.score_transport(*t, context))
            .collect();

        scored_transports.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let primary_transport = scored_transports[0].transport_type;
        let mut fallback_transports = Vec::new();
        let mut use_multipath = false;

        if scored_transports.len() > 1 {
            fallback_transports.push(scored_transports[1].transport_type);
            
            // If top two scores are within 15%, consider multipath
            if (scored_transports[0].score - scored_transports[1].score) / scored_transports[0].score < 0.15 {
                use_multipath = true;
            }
        }

        Ok(RoutingDecision {
            primary_transport,
            fallback_transports,
            use_multipath,
            scores: scored_transports,
        })
    }

    pub fn score_transport(
        &self,
        transport: TransportType,
        context: &RoutingContext,
    ) -> TransportScore {
        let mut factors = HashMap::new();
        let mut total_score = 0.0;

        match transport {
            TransportType::Ble => {
                let prox_score = match context.peer_rssi {
                    Some(rssi) if rssi > -60 => 1.0,
                    Some(rssi) if rssi > -80 => 0.5,
                    Some(_) => 0.2,
                    None => 0.0,
                };
                factors.insert("proximity".to_string(), prox_score);
                total_score += prox_score * self.weights.proximity;

                factors.insert("battery_cost".to_string(), 1.0); // low cost is good
                total_score += 1.0 * self.weights.battery_cost;

                factors.insert("privacy".to_string(), 1.0);
                total_score += 1.0 * self.weights.privacy;

                // Penalty for large payloads over BLE
                let throughput_score = if context.payload_size > 100_000 { 0.1 } else { 0.8 };
                factors.insert("throughput_match".to_string(), throughput_score);
                total_score += throughput_score * self.weights.throughput_match;
            }
            TransportType::WifiDirect => {
                factors.insert("proximity".to_string(), 0.8);
                total_score += 0.8 * self.weights.proximity;

                factors.insert("battery_cost".to_string(), 0.5);
                total_score += 0.5 * self.weights.battery_cost;

                factors.insert("privacy".to_string(), 1.0);
                total_score += 1.0 * self.weights.privacy;

                factors.insert("throughput_match".to_string(), 1.0);
                total_score += 1.0 * self.weights.throughput_match;
            }
            TransportType::InternetRelay => {
                let avail_score = if context.network_state.has_internet {
                    if context.network_state.has_wifi {
                        1.0
                    } else if context.network_state.has_mobile_data && context.user_preferences.allow_mobile_data {
                        0.8
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                factors.insert("availability".to_string(), avail_score);
                total_score += avail_score * self.weights.reliability;

                factors.insert("proximity".to_string(), 0.0); // Not relevant
                
                factors.insert("battery_cost".to_string(), 0.7);
                total_score += 0.7 * self.weights.battery_cost;

                factors.insert("privacy".to_string(), 0.3); // Relay has lower privacy
                total_score += 0.3 * self.weights.privacy;

                factors.insert("throughput_match".to_string(), 1.0);
                total_score += 1.0 * self.weights.throughput_match;
            }
            TransportType::Mesh => {
                let avail_score = if context.user_preferences.mesh_enabled { 0.8 } else { 0.0 };
                factors.insert("availability".to_string(), avail_score);
                total_score += avail_score * self.weights.reliability;

                factors.insert("privacy".to_string(), 0.8);
                total_score += 0.8 * self.weights.privacy;
                
                factors.insert("trust".to_string(), context.peer_trust_score);
                total_score += context.peer_trust_score * self.weights.trust;
            }
        }

        TransportScore {
            transport_type: transport,
            score: total_score,
            factors,
        }
    }

    pub fn normalize(value: f64, min: f64, max: f64) -> f64 {
        if max == min {
            return 0.0;
        }
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    }
}
