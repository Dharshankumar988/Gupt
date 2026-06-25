mod mapper;

use std::sync::Arc;
use tokio::sync::Mutex;

use gupt_common::types::{ConversationId, MessageId};
use gupt_identity::manager::IdentityManager;
use gupt_routing::RoutingEngine;
use gupt_storage::Database;

// Import the generated UniFFI scaffolding
uniffi::include_scaffolding!("gupt");

// Expose the mapped GuptError so the UDL can bind to it
pub use mapper::GuptError;

// ────────────────────────────────────────────────────────────────────────────
// Dictionaries mapped directly from UDL
// ────────────────────────────────────────────────────────────────────────────

pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub message_type: String,
    pub payload: String,
    pub ttl_seconds: u32,
    pub delivery_status: String,
    pub created_at: String,
}

pub struct Conversation {
    pub id: String,
    pub display_name: Option<String>,
    pub is_group: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_message_at: Option<String>,
}

pub struct Peer {
    pub id: String,
    pub display_name: Option<String>,
    pub trust_score: f64,
    pub is_blocked: bool,
}

// ────────────────────────────────────────────────────────────────────────────
// Main Engine Interface Implementation
// ────────────────────────────────────────────────────────────────────────────

pub struct GuptEngine {
    db: Arc<Mutex<Option<Database>>>,
    identity_manager: Arc<Mutex<IdentityManager>>,
    routing_engine: Arc<Mutex<RoutingEngine>>,
}

// Global static initialization function required by UDL
pub fn initialize(db_path: String, pin: String) -> Result<Arc<GuptEngine>, GuptError> {
    // 1. Open the SQLCipher database
    let db = Database::open(&db_path, &pin)?;
    db.run_migrations()?;
    
    // 2. Initialize Identity Manager
    let mut identity_manager = IdentityManager::default();
    
    // In a real scenario we'd check if an identity exists.
    // For this scaffolding, we attempt to unlock, or create if missing.
    // Note: The actual byte fetching would occur from the local DB.
    if !identity_manager.is_unlocked() {
        // Fallback stub: Create one if unlocking fails/doesn't exist
        identity_manager.create_identity("User", &pin)?;
    }
    
    // 3. Initialize Routing Engine
    let routing_engine = RoutingEngine::new();
    
    let engine = GuptEngine {
        db: Arc::new(Mutex::new(Some(db))),
        identity_manager: Arc::new(Mutex::new(identity_manager)),
        routing_engine: Arc::new(Mutex::new(routing_engine)),
    };
    
    Ok(Arc::new(engine))
}

impl GuptEngine {
    pub fn lock(&self) {
        // We block on the async mutex purely because UniFFI doesn't inherently 
        // make every method async unless requested. 
        // In a production app, we would use async uniFFI methods.
        if let Ok(mut id_mgr) = self.identity_manager.try_lock() {
            id_mgr.lock();
        }
    }

    pub fn is_unlocked(&self) -> bool {
        if let Ok(id_mgr) = self.identity_manager.try_lock() {
            return id_mgr.is_unlocked();
        }
        false
    }

    pub fn send_message(&self, conversation_id: String, content: String) -> Result<String, GuptError> {
        if !self.is_unlocked() {
            return Err(GuptError::Unauthorized("Identity is locked".to_string()));
        }
        
        // This is a stub implementation connecting the UDL to the Rust Core.
        // In reality, this would enqueue the message to the MeshQueueRepository
        // and trigger the RoutingEngine.
        
        let msg_id = uuid::Uuid::new_v4().to_string();
        // println!("Sending message to {}: {}", conversation_id, content);
        
        Ok(msg_id)
    }

    pub fn get_conversations(&self) -> Result<Vec<Conversation>, GuptError> {
        // Stub: Return mock data that the UI expects
        Ok(vec![
            Conversation {
                id: "1".to_string(),
                display_name: Some("Alice".to_string()),
                is_group: false,
                created_at: "2026-06-20T10:00:00Z".to_string(),
                updated_at: "2026-06-25T10:42:00Z".to_string(),
                last_message_at: Some("2026-06-25T10:42:00Z".to_string()),
            }
        ])
    }

    pub fn get_messages(&self, _conversation_id: String) -> Result<Vec<Message>, GuptError> {
        // Stub: Return empty message list. The UI provides its own mocks for now.
        Ok(vec![])
    }

    pub fn get_nearby_peers(&self) -> Result<Vec<Peer>, GuptError> {
        Ok(vec![
            Peer {
                id: "peer_1".to_string(),
                display_name: Some("Bob's Phone".to_string()),
                trust_score: 0.8,
                is_blocked: false,
            }
        ])
    }

    // ────────────────────────────────────────────────────────────────────────────
    // Call Engine Methods
    // ────────────────────────────────────────────────────────────────────────────

    pub fn start_call(&self, phone_number: String) -> Result<String, GuptError> {
        // Stub implementation
        Ok(uuid::Uuid::new_v4().to_string())
    }

    pub fn accept_call(&self, _session_id: String) -> Result<(), GuptError> {
        Ok(())
    }

    pub fn reject_call(&self, _session_id: String) -> Result<(), GuptError> {
        Ok(())
    }

    pub fn end_call(&self, _session_id: String) -> Result<(), GuptError> {
        Ok(())
    }

    pub fn mute(&self) -> Result<(), GuptError> {
        Ok(())
    }

    pub fn unmute(&self) -> Result<(), GuptError> {
        Ok(())
    }

    pub fn toggle_speaker(&self) -> Result<(), GuptError> {
        Ok(())
    }
}
