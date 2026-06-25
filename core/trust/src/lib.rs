//! # gupt-trust
//!
//! Trust scoring engine for mesh peers in the Gupt platform.

use gupt_common::PeerId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during trust evaluation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TrustError {
    /// Peer not found in trust store.
    #[error("peer not found: {0}")]
    PeerNotFound(String),
}

/// Trust score for a peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    /// The peer being scored.
    pub peer_id: PeerId,
    /// Overall trust score (0.0–1.0).
    pub score: f64,
    /// Number of successful relays.
    pub successful_relays: u32,
    /// Number of failed relays.
    pub failed_relays: u32,
}

/// Trust engine that manages peer trust scores.
#[derive(Debug, Default)]
pub struct TrustEngine {
    /// Known peer scores.
    scores: std::collections::HashMap<PeerId, TrustScore>,
}

impl TrustEngine {
    /// Creates a new trust engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the trust score for a peer, defaulting to 0.5 for unknown peers.
    pub fn get_score(&self, peer_id: &PeerId) -> f64 {
        self.scores
            .get(peer_id)
            .map(|s| s.score)
            .unwrap_or(0.5)
    }

    /// Records a successful relay by the given peer.
    pub fn record_success(&mut self, peer_id: &PeerId) {
        let entry = self.scores.entry(peer_id.clone()).or_insert_with(|| TrustScore {
            peer_id: peer_id.clone(),
            score: 0.5,
            successful_relays: 0,
            failed_relays: 0,
        });
        entry.successful_relays += 1;
        entry.score = Self::compute_score(entry.successful_relays, entry.failed_relays);
    }

    /// Records a failed relay by the given peer.
    pub fn record_failure(&mut self, peer_id: &PeerId) {
        let entry = self.scores.entry(peer_id.clone()).or_insert_with(|| TrustScore {
            peer_id: peer_id.clone(),
            score: 0.5,
            successful_relays: 0,
            failed_relays: 0,
        });
        entry.failed_relays += 1;
        entry.score = Self::compute_score(entry.successful_relays, entry.failed_relays);
    }

    /// Computes trust score from success/failure counts using a simple ratio.
    fn compute_score(successes: u32, failures: u32) -> f64 {
        let total = successes + failures;
        if total == 0 {
            return 0.5;
        }
        // Bayesian-style: add 1 success and 1 failure as priors
        (successes as f64 + 1.0) / (total as f64 + 2.0)
    }
}
