use chrono::Utc;
use gupt_common::types::{MessageEnvelope, MessageId, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Queue is full (capacity: {capacity})")]
    QueueFull { capacity: usize },
    #[error("Message expired: {0}")]
    MessageExpired(MessageId),
    #[error("Message not found: {0}")]
    MessageNotFound(MessageId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum QueuePriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedMessage {
    pub envelope: MessageEnvelope,
    pub priority: QueuePriority,
    pub queued_at: Timestamp,
    pub retry_count: u32,
    pub next_retry_at: Option<Timestamp>,
}

// We want BinaryHeap to be a max-heap where highest priority (Critical=0) comes first.
// Since QueuePriority Ord derives so that Critical < Low, we need a wrapper to reverse it,
// or implement Ord on QueuedMessage to prioritize lower `QueuePriority` values.
impl PartialEq for QueuedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.envelope.id == other.envelope.id
    }
}

impl Eq for QueuedMessage {}

impl PartialOrd for QueuedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering so Critical (0) is "greater" than Low (3) for max-heap
        other.priority.cmp(&self.priority)
            // Tie-breaker: older messages first
            .then_with(|| self.queued_at.cmp(&other.queued_at).reverse())
    }
}

pub struct MessageQueue {
    messages: BinaryHeap<QueuedMessage>,
    max_capacity: usize,
}

impl MessageQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            messages: BinaryHeap::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    pub fn enqueue(&mut self, message: QueuedMessage) -> Result<(), QueueError> {
        if self.messages.len() >= self.max_capacity {
            return Err(QueueError::QueueFull { capacity: self.max_capacity });
        }
        self.messages.push(message);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<QueuedMessage> {
        self.messages.pop()
    }

    pub fn peek(&self) -> Option<&QueuedMessage> {
        self.messages.peek()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn remove(&mut self, id: &MessageId) -> Option<QueuedMessage> {
        let mut new_heap = BinaryHeap::with_capacity(self.max_capacity);
        let mut found = None;

        for msg in self.messages.drain() {
            if msg.envelope.id.as_ref() == id.as_ref() {
                found = Some(msg);
            } else {
                new_heap.push(msg);
            }
        }
        
        self.messages = new_heap;
        found
    }

    pub fn purge_expired(&mut self) -> usize {
        let initial_len = self.messages.len();
        let now = Utc::now();
        let mut new_heap = BinaryHeap::with_capacity(self.max_capacity);

        for msg in self.messages.drain() {
            let age_seconds = now.signed_duration_since(msg.queued_at.as_ref().clone()).num_seconds();
            if age_seconds <= msg.envelope.ttl_seconds as i64 {
                new_heap.push(msg);
            }
        }

        self.messages = new_heap;
        initial_len - self.messages.len()
    }

    pub fn get_by_id(&self, id: &MessageId) -> Option<&QueuedMessage> {
        self.messages.iter().find(|m| m.envelope.id.as_ref() == id.as_ref())
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new(10_000)
    }
}
