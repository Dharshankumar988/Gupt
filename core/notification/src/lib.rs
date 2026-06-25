use gupt_common::types::MessageId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of notifications that can be generated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationType {
    NewMessage,
    MessageDelivered,
    PeerDiscovered,
    TransferComplete,
    TransferFailed,
    ConnectionLost,
}

/// The payload for a notification to be passed to the mobile OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: String,
    pub notification_type: NotificationType,
    pub metadata: HashMap<String, String>,
}

/// Engine responsible for creating standardized notifications.
#[derive(Debug, Default)]
pub struct NotificationEngine {}

impl NotificationEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_message_notification(sender: &str, preview: &str) -> NotificationPayload {
        NotificationPayload {
            title: format!("New message from {}", sender),
            body: preview.to_string(),
            notification_type: NotificationType::NewMessage,
            metadata: HashMap::new(),
        }
    }

    pub fn create_delivery_notification(message_id: &MessageId) -> NotificationPayload {
        let mut metadata = HashMap::new();
        metadata.insert("message_id".to_string(), message_id.as_ref().to_string());
        
        NotificationPayload {
            title: "Message Delivered".to_string(),
            body: "Your message was securely delivered.".to_string(),
            notification_type: NotificationType::MessageDelivered,
            metadata,
        }
    }

    pub fn create_peer_notification(peer_name: &str) -> NotificationPayload {
        NotificationPayload {
            title: "Peer Discovered".to_string(),
            body: format!("{} is nearby on the mesh network.", peer_name),
            notification_type: NotificationType::PeerDiscovered,
            metadata: HashMap::new(),
        }
    }
}
