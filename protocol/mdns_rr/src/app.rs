use crate::http::http_server;
use crate::kafka::kafka_consumer_manager;
use crate::settings::get_config;
use crate::swarm::{swarm_server, ChannelHub};
use crate::ws::ws_server;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Application bootstrap and service orchestration

pub struct App {
    http_server: JoinHandle<()>,
    kafka_consumer_manager: JoinHandle<()>,
    ws_server: JoinHandle<()>,
    swarm_server: JoinHandle<()>,
}

/**
Responsible for:

Loading config

Initializing metrics

Spawning services (tokio::spawn)

Wiring communication channels between Kafka ↔ libp2p ↔ WebSocket
*/
impl App {
    /**
    Init channels
    mpsc for commands (1 producer → 1 consumer)
    broadcast for events (1 producer → N consumers)

    */
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = get_config();
        let (channel_hub, rx_channel) = ChannelHub::new(config.channel.clone());
        let channel_hub_clone = Arc::new(channel_hub);

        // Initialize application state, services, etc.
        let swarm_server = tokio::spawn(swarm_server::run(
            channel_hub_clone.clone(),
            rx_channel.swarm_cmd_rx,
        ));
        let http_server = tokio::spawn(http_server::run(
            channel_hub_clone.clone()
        ));
        let ws_server = tokio::spawn(ws_server::run(
            channel_hub_clone.clone(),
            rx_channel.ws_cmd_rx,
        ));
        // let ws_distributor = tokio::spawn(ws_event_handler::run(
        //     channel_hub_clone.clone(),
        // ));
        let kafka_consumer_manager = tokio::spawn(kafka_consumer_manager::run(
            channel_hub_clone.clone(),
            rx_channel.kafka_cmd_rx,
        ));


        Ok(App {
            swarm_server,
            http_server,
            ws_server,
            kafka_consumer_manager,
        })
    }

    pub async fn run(self) {
        let _ = tokio::join!(
            self.swarm_server,
            self.http_server,
            self.ws_server,
            self.kafka_consumer_manager,
        );
    }
}