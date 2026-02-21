pub mod swarm_server;
pub mod metrics;

use crate::settings::settings::ChannelConfig;
use libp2p::PeerId;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::{broadcast, mpsc};
use tokio::time::interval;

fn deserialize_peer_id<'de, D>(deserializer: D) -> Result<PeerId, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    PeerId::from_str(&s).map_err(serde::de::Error::custom)
}

fn serialize_peer_id<S>(peer_id: &PeerId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&peer_id.to_string())
}

// Channel message types for inter-module communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppSwarmEvent {
    PeerDiscovered {
        #[serde(serialize_with = "serialize_peer_id", deserialize_with = "deserialize_peer_id")]
        peer_id: PeerId,
        address: String,
    },
    PeerDisconnected {
        #[serde(serialize_with = "serialize_peer_id", deserialize_with = "deserialize_peer_id")]
        peer_id: PeerId
    },
    MessageReceived {
        #[serde(serialize_with = "serialize_peer_id", deserialize_with = "deserialize_peer_id")]
        from: PeerId,
        content: String,
    },
    MessageSent {
        #[serde(serialize_with = "serialize_peer_id", deserialize_with = "deserialize_peer_id")]
        to: PeerId,
        content: String,
    },
}

#[derive(Debug, Clone)]
pub enum SwarmCommand {
    // TCP Streams
    BroadcastRRMessage { content: String },
    SendRRMessage { to: PeerId, content: String },
    // Gossips
    BroadcastGossipMessage { content: String },
    PublishTopic { topic: String, content: String },
    SubscribeTopic { topic: String },

    ConnectToPeer { peer_id: PeerId, address: String },
    DisconnectFromPeer { peer_id: PeerId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketEvent {
    ClientConnected { client_id: String },
    ClientDisconnected { client_id: String },
    MessageFromClient { client_id: String, content: String },
}

#[derive(Debug, Clone)]
pub enum WebSocketCommand {
    SendToClient { client_id: String, content: String },
    BroadcastToAll { content: String },
    DisconnectClient { client_id: String },
}


#[derive(Debug, Clone)]
pub enum KafkaConsumerCmd {
    Subscribe(String),
    Unsubscribe(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KafkaConsumerEvent {
    MessageReceived { topic: String, content: String },
    SubscriptionConfirmed { topic: String },
    SubscriptionFailed { topic: String, error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpEvent {
    SubscribeRequest { topic: String },
    UnsubscribeRequest { topic: String },
    StatusRequest,
}

// Central event bus for all modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    Swarm(AppSwarmEvent),
    WebSocket(WebSocketEvent),
    KafkaConsumer(KafkaConsumerEvent),
    Http(HttpEvent),
}


// Central channel hub for all modules
pub struct ChannelHub {
    // Command channels (mpsc - single producer, single consumer)
    pub swarm_cmd_tx: mpsc::UnboundedSender<SwarmCommand>,
    pub ws_cmd_tx: mpsc::UnboundedSender<WebSocketCommand>,
    pub kafka_cmd_tx: mpsc::Sender<KafkaConsumerCmd>,

    // Event channels (broadcast - single producer, multiple consumers)
    pub event_tx: broadcast::Sender<AppEvent>,
}

impl ChannelHub {
    pub fn new(config: ChannelConfig) -> (Self, ChannelReceivers) {
        // let (swarm_cmd_tx, swarm_cmd_rx) = mpsc::channel(config.command_buffer_size);
        // let (ws_cmd_tx, ws_cmd_rx) = mpsc::channel(config.command_buffer_size);
        // let (kafka_cmd_tx, kafka_cmd_rx) = mpsc::channel(config.command_buffer_size);
        // let (event_tx, _) = broadcast::channel(config.event_buffer_size);
        let (swarm_cmd_tx, swarm_cmd_rx) = unbounded_channel();
        let (ws_cmd_tx, ws_cmd_rx) = mpsc::unbounded_channel();
        let (kafka_cmd_tx, kafka_cmd_rx) = mpsc::channel(config.command_buffer_size);
        let (event_tx, _) = broadcast::channel(config.event_buffer_size);

        let hub = Self {
            swarm_cmd_tx,
            ws_cmd_tx,
            kafka_cmd_tx,
            event_tx,
        };

        let receivers = ChannelReceivers {
            swarm_cmd_rx,
            ws_cmd_rx,
            kafka_cmd_rx,
        };

        (hub, receivers)
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<AppEvent> {
        self.event_tx.subscribe()
    }
}

pub struct ChannelReceivers {
    pub swarm_cmd_rx: mpsc::UnboundedReceiver<SwarmCommand>,
    pub ws_cmd_rx: mpsc::UnboundedReceiver<WebSocketCommand>,
    pub kafka_cmd_rx: mpsc::Receiver<KafkaConsumerCmd>,
}

pub async fn ticker_task(name: &str) {
    let mut interval = interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        // debug!("Tick - {name} server is running");
    }
}

// Re-export commonly used types
// pub use default_kafka_consumer::{ResponseMessage, KafkaConsumerTask, spawn_kafka_consumer_task};
// pub use kafka_consumer_manager::{KafkaConsumerManager, ConsumerStats, create_consumer_manager};
