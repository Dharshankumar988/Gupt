use chrono::Utc;
use gupt_common::types::{MessageId, Timestamp};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Retries exhausted for message: {0}")]
    RetryExhausted(MessageId),
    #[error("Conflict resolution failed: {0}")]
    ConflictResolution(String),
    #[error("Backup failed: {0}")]
    BackupFailed(String),
    #[error("Network unavailable")]
    NetworkUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 10,
            initial_delay_ms: 1000,
            max_delay_ms: 300_000, // 5 minutes
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryPolicy {
    pub fn delay_for_attempt(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0;
        }
        
        let mut delay = self.initial_delay_ms as f64 * self.backoff_multiplier.powi((attempt - 1) as i32);
        
        if delay > self.max_delay_ms as f64 {
            delay = self.max_delay_ms as f64;
        }

        // Apply jitter (0.5 - 1.5)
        let mut rng = rand::thread_rng();
        let jitter = rng.gen_range(0.5..1.5);
        
        (delay * jitter) as u64
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncState {
    Idle,
    Syncing,
    RetryScheduled { attempt: u32, next_retry_at: Timestamp },
    Failed { error: String },
}

#[derive(Debug, Clone)]
pub struct PendingMessage {
    pub message_id: MessageId,
    pub payload: Vec<u8>,
    pub attempts: u32,
    pub last_attempt: Option<Timestamp>,
    pub state: SyncState,
    pub created_at: Timestamp,
}

pub struct SyncEngine {
    retry_policy: RetryPolicy,
    pending: Vec<PendingMessage>,
}

impl SyncEngine {
    pub fn new(retry_policy: RetryPolicy) -> Self {
        Self {
            retry_policy,
            pending: Vec::new(),
        }
    }

    pub fn add_pending(&mut self, message: PendingMessage) {
        self.pending.push(message);
    }

    pub fn get_ready_for_retry(&mut self) -> Vec<&PendingMessage> {
        let now = Utc::now();
        self.pending
            .iter_mut()
            .filter(|m| {
                if let SyncState::RetryScheduled { next_retry_at, .. } = &m.state {
                    now >= next_retry_at.as_ref().clone()
                } else {
                    false
                }
            })
            .map(|m| &*m)
            .collect()
    }

    pub fn record_attempt(&mut self, id: &MessageId, success: bool) {
        if let Some(msg) = self.pending.iter_mut().find(|m| m.message_id.as_ref() == id.as_ref()) {
            msg.attempts += 1;
            msg.last_attempt = Some(Timestamp::from(Utc::now()));

            if success {
                msg.state = SyncState::Idle; // Or remove it entirely depending on flow
            } else if msg.attempts >= self.retry_policy.max_retries {
                msg.state = SyncState::Failed { error: "Max retries exceeded".to_string() };
            } else {
                let delay = self.retry_policy.delay_for_attempt(msg.attempts);
                let next_retry_at = Timestamp::from(Utc::now() + chrono::Duration::milliseconds(delay as i64));
                msg.state = SyncState::RetryScheduled { attempt: msg.attempts, next_retry_at };
            }
        }
    }

    pub fn should_retry(&self, message: &PendingMessage) -> bool {
        message.attempts < self.retry_policy.max_retries
    }

    pub fn remove_delivered(&mut self, id: &MessageId) {
        self.pending.retain(|m| m.message_id.as_ref() != id.as_ref());
    }
}

impl Default for SyncEngine {
    fn default() -> Self {
        Self::new(RetryPolicy::default())
    }
}
