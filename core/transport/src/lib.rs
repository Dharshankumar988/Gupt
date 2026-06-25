use async_trait::async_trait;
use bytes::Bytes;
use gupt_common::types::{
    BatteryCost, BytesPerSecond, MessageId, PeerId, SignalQuality, Timestamp, TransportType,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Transport {0:?} is not available")]
    NotAvailable(TransportType),
    #[error("Failed to send message: {0}")]
    SendFailed(String),
    #[error("Failed to receive message: {0}")]
    ReceiveFailed(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Operation timed out")]
    Timeout,
    #[error("Not connected to peer")]
    NotConnected,
    #[error("Payload too large: {size} > {max}")]
    PayloadTooLarge { size: usize, max: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryReceipt {
    pub message_id: MessageId,
    pub transport_used: TransportType,
    pub delivered_at: Timestamp,
    pub hops: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingPacket {
    pub data: Vec<u8>,
    pub sender: PeerId,
    pub transport_type: TransportType,
    pub received_at: Timestamp,
    pub rssi: Option<i16>,
}

#[derive(Debug, Clone)]
pub enum TransportEvent {
    PeerDiscovered {
        peer: PeerId,
        transport: TransportType,
        rssi: Option<i16>,
    },
    PeerLost {
        peer: PeerId,
        transport: TransportType,
    },
    DataReceived(IncomingPacket),
    ConnectionEstablished {
        peer: PeerId,
        transport: TransportType,
    },
    ConnectionLost {
        peer: PeerId,
        transport: TransportType,
    },
}

pub trait TransportEventCallback: Send + Sync {
    fn on_event(&self, event: TransportEvent);
}

#[async_trait]
pub trait Transport: Send + Sync {
    fn transport_type(&self) -> TransportType;
    fn is_available(&self) -> bool;
    fn signal_quality(&self) -> SignalQuality;
    fn estimated_throughput(&self) -> BytesPerSecond;
    fn battery_cost(&self) -> BatteryCost;
    fn max_payload_size(&self) -> usize;

    async fn connect(&mut self, peer: &PeerId) -> Result<(), TransportError>;
    async fn disconnect(&mut self, peer: &PeerId) -> Result<(), TransportError>;
    async fn send(&self, peer: &PeerId, data: &[u8]) -> Result<DeliveryReceipt, TransportError>;
    async fn start(&mut self) -> Result<(), TransportError>;
    async fn stop(&mut self) -> Result<(), TransportError>;
}

pub struct TransportManager {
    transports: Vec<Box<dyn Transport>>,
}

impl TransportManager {
    pub fn new() -> Self {
        Self {
            transports: Vec::new(),
        }
    }

    pub fn register_transport(&mut self, transport: Box<dyn Transport>) {
        self.transports.push(transport);
    }

    pub fn available_transports(&self) -> Vec<TransportType> {
        self.transports
            .iter()
            .filter(|t| t.is_available())
            .map(|t| t.transport_type())
            .collect()
    }

    pub fn get_transport(&self, transport_type: TransportType) -> Option<&dyn Transport> {
        self.transports
            .iter()
            .find(|t| t.transport_type() == transport_type)
            .map(|t| t.as_ref())
    }

    pub async fn start_all(&mut self) -> Result<(), TransportError> {
        for transport in &mut self.transports {
            transport.start().await?;
        }
        Ok(())
    }

    pub async fn stop_all(&mut self) -> Result<(), TransportError> {
        for transport in &mut self.transports {
            transport.stop().await?;
        }
        Ok(())
    }
}

impl Default for TransportManager {
    fn default() -> Self {
        Self::new()
    }
}
