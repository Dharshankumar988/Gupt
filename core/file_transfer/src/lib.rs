use chrono::Utc;
use gupt_common::types::{Timestamp, TransportType, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum FileTransferError {
    #[error("File too large: {size} > {max}")]
    FileTooLarge { size: u64, max: u64 },
    #[error("Chunk failed: {chunk_index}")]
    ChunkFailed { chunk_index: u32 },
    #[error("Transfer not found: {0}")]
    TransferNotFound(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Resume failed: {0}")]
    ResumeFailed(String),
    #[error("Transfer cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_id: Uuid,
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub sha256_hash: [u8; 32],
    pub chunk_size: u32,
    pub total_chunks: u32,
    pub encrypted: bool,
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            file_id: Uuid::new_v4(),
            file_name: String::new(),
            file_size: 0,
            mime_type: String::new(),
            sha256_hash: [0; 32],
            chunk_size: 65536, // 64KB
            total_chunks: 0,
            encrypted: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChunkState {
    Pending,
    InFlight,
    Delivered,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferState {
    pub file_id: Uuid,
    pub metadata: FileMetadata,
    pub chunks_completed: u32,
    pub chunks_total: u32,
    pub state: TransferStatus,
    pub started_at: Timestamp,
    pub transport_used: Option<TransportType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransferStatus {
    Queued,
    InProgress,
    Paused,
    Completed,
    Failed(String),
    Cancelled,
}

pub struct FileTransferEngine {
    active_transfers: HashMap<Uuid, TransferState>,
}

impl FileTransferEngine {
    pub fn new() -> Self {
        Self {
            active_transfers: HashMap::new(),
        }
    }

    pub fn initiate_transfer(&mut self, metadata: FileMetadata, _recipient: &UserId) -> Result<Uuid, FileTransferError> {
        let id = metadata.file_id;
        
        let state = TransferState {
            file_id: id,
            chunks_completed: 0,
            chunks_total: metadata.total_chunks,
            metadata,
            state: TransferStatus::Queued,
            started_at: Timestamp::from(Utc::now()),
            transport_used: None,
        };

        self.active_transfers.insert(id, state);
        Ok(id)
    }

    pub fn get_transfer_state(&self, id: &Uuid) -> Option<&TransferState> {
        self.active_transfers.get(id)
    }

    pub fn pause_transfer(&mut self, id: &Uuid) -> Result<(), FileTransferError> {
        if let Some(state) = self.active_transfers.get_mut(id) {
            state.state = TransferStatus::Paused;
            Ok(())
        } else {
            Err(FileTransferError::TransferNotFound(id.to_string()))
        }
    }

    pub fn resume_transfer(&mut self, id: &Uuid) -> Result<(), FileTransferError> {
        if let Some(state) = self.active_transfers.get_mut(id) {
            state.state = TransferStatus::InProgress;
            Ok(())
        } else {
            Err(FileTransferError::TransferNotFound(id.to_string()))
        }
    }

    pub fn cancel_transfer(&mut self, id: &Uuid) -> Result<(), FileTransferError> {
        if let Some(state) = self.active_transfers.get_mut(id) {
            state.state = TransferStatus::Cancelled;
            Ok(())
        } else {
            Err(FileTransferError::TransferNotFound(id.to_string()))
        }
    }

    pub fn record_chunk_delivered(&mut self, id: &Uuid, _chunk_index: u32) -> Result<(), FileTransferError> {
        if let Some(state) = self.active_transfers.get_mut(id) {
            state.chunks_completed += 1;
            if state.chunks_completed >= state.chunks_total {
                state.state = TransferStatus::Completed;
            }
            Ok(())
        } else {
            Err(FileTransferError::TransferNotFound(id.to_string()))
        }
    }

    pub fn select_transport_for_file(&self, file_size: u64, peer_nearby: bool, internet_available: bool) -> TransportType {
        if file_size < 100_000 && peer_nearby {
            TransportType::Ble
        } else if file_size < 50_000_000 && peer_nearby {
            TransportType::WifiDirect
        } else if internet_available {
            TransportType::InternetRelay
        } else {
            TransportType::Mesh
        }
    }

    pub fn active_transfer_count(&self) -> usize {
        self.active_transfers.values().filter(|t| t.state == TransferStatus::InProgress).count()
    }
}

impl Default for FileTransferEngine {
    fn default() -> Self {
        Self::new()
    }
}
