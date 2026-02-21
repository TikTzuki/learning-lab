use crate::kafka::broadcast_kafka_topic_handler::MarketData;
use crate::settings::{get_config, AppConfig};
use crate::swarm::{ticker_task, AppEvent, AppSwarmEvent, ChannelHub, KafkaConsumerEvent, WebSocketCommand, WebSocketEvent};
use crate::utils::append_string_to_file;
use crate::ws::metrics::{increase_ws_messages_sent, increase_ws_rx_message_received};
use futures_util::{SinkExt, StreamExt};
use libp2p::bytes::Bytes;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};

pub struct WebSocketServer {
    clients: HashMap<String, mpsc::UnboundedSender<String>>,
    event_tx: broadcast::Sender<AppEvent>,
    mut_event_rx: broadcast::Receiver<AppEvent>,
}

impl WebSocketServer {
    pub fn new(
        event_tx: broadcast::Sender<AppEvent>,
        event_rx: broadcast::Receiver<AppEvent>,
    ) -> Self {
        Self {
            clients: HashMap::new(),
            event_tx,
            mut_event_rx: event_rx,
        }
    }

    async fn handle_command(&mut self, command: WebSocketCommand) {
        match command {
            WebSocketCommand::SendToClient { client_id, content } => {
                if let Some(sender) = self.clients.get(&client_id) {
                    if let Err(e) = sender.send(content) {
                        warn!("WebSocketCommand::SendToClient fail {}: {}", client_id, e);
                        self.clients.remove(&client_id);
                    }
                } else {
                    warn!("Client {} not found", client_id);
                }
            }
            WebSocketCommand::BroadcastToAll { content } => {
                let mut disconnected_clients = Vec::new();
                debug!("Broadcasting message to all clients: {}", content);

                let mut failed_messages: Vec<String> = Vec::new();
                for (client_id, sender) in &self.clients {
                    match sender.send(content.clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            let debug_content = serde_json::from_str::<MarketData>(content.as_str()).unwrap();
                            failed_messages.push(
                                format!("{},{},{},{:?}\n",
                                        client_id, debug_content.round,
                                        debug_content.c, debug_content.ts.unwrap_or(0)));
                            warn!("WebSocketCommand::BroadcastToAll: fail {}: {}", client_id, e);
                            disconnected_clients.push(client_id.clone());
                        }
                    }
                }
                if !failed_messages.is_empty() {
                    append_string_to_file(
                        format!("logs/ws-fail-{}.csv", 123),
                        failed_messages.join("").as_str(),
                    )
                        .await.expect("TODO: panic message");
                }

                // Remove disconnected clients
                for client_id in disconnected_clients {
                    self.clients.remove(&client_id);
                    let _ = self.event_tx.send(AppEvent::WebSocket(WebSocketEvent::ClientDisconnected {
                        client_id,
                    }));
                }
            }
            WebSocketCommand::DisconnectClient { client_id } => {
                if self.clients.remove(&client_id).is_some() {
                    let _ = self.event_tx.send(AppEvent::WebSocket(WebSocketEvent::ClientDisconnected {
                        client_id,
                    }));
                }
            }
        }
    }

    async fn handle_app_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Swarm(swarm_event) => {
                match swarm_event {
                    AppSwarmEvent::MessageReceived { from, content, .. } => {
                        debug!("WebSocketServer received message from {}: {}", from, content);
                        increase_ws_rx_message_received().await;
                        self.handle_command(WebSocketCommand::BroadcastToAll { content }).await;
                    }
                    _ => { warn!("Unhandled AppSwarmEvent: {:?}", swarm_event); }
                }
            }
            AppEvent::KafkaConsumer(kafka_event) => {
                match kafka_event {
                    KafkaConsumerEvent::MessageReceived { topic, content, .. } => {
                        debug!("Kafka message from {}: {}", topic, content);
                        self.handle_command(WebSocketCommand::BroadcastToAll { content }).await;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    async fn handle_client_connection(
        &mut self,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        let client_id = addr.to_string();
        self.clients.insert(client_id.clone(), tx);

        let _ = self.event_tx.send(AppEvent::WebSocket(WebSocketEvent::ClientConnected {
            client_id: client_id.clone(),
        }));

        info!("WebSocket client connected: {}", client_id);

        // Spawn task to handle this client
        let event_tx = self.event_tx.clone();
        let client_id_clone = client_id.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Handle incoming messages from client
                    msg = ws_receiver.next() => {
                        match msg {
                            Some(Ok(WsMessage::Text(text))) => {
                                debug!("Received message from {}: {}", client_id_clone, text);
                                let _ = event_tx.send(AppEvent::WebSocket(WebSocketEvent::MessageFromClient {
                                    client_id: client_id_clone.clone(),
                                    content: text.to_string(),
                                }));
                            }
                            Some(Ok(WsMessage::Close(_))) | None => {
                                info!("Client {} disconnected", client_id_clone);
                                let _ = event_tx.send(AppEvent::WebSocket(WebSocketEvent::ClientDisconnected {
                                    client_id: client_id_clone.clone(),
                                }));
                                break;
                            }
                            Some(Err(e)) => {
                                error!("WebSocket error for client {}: {}", client_id_clone, e);
                                break;
                            }
                            _ => {}
                        }
                    }
                    // Handle outgoing messages to client
                    Some(message) = rx.recv() => {
                        match ws_sender.send(WsMessage::Text( message.to_string().into())).await {
                        Ok(_) => {
                            increase_ws_messages_sent().await;
                            }
                            Err(e) => {
                            error!("ws_sender.send: fail {}: {}", client_id_clone, e);
                            }}
                    }
                }
            }
        });

        Ok(())
    }

    async fn serve(&mut self, mut cmd_rx: mpsc::UnboundedReceiver<WebSocketCommand>) {
        let config = get_config();
        let addr = format!("{}:{}", config.ws.host, config.ws.port);

        let listener = match TcpListener::bind(&addr).await {
            Ok(listener) => {
                info!("WebSocket server listening on {}", addr);
                listener
            }
            Err(e) => {
                error!("Failed to bind WebSocket server to {}: {}", addr, e);
                return;
            }
        };
        loop {
            tokio::select! {
                Ok((stream, addr)) = listener.accept() => {
                    if let Err(e) = self.handle_client_connection(stream, addr).await {
                        error!("Failed to handle WebSocket connection from {}: {}", addr, e);
                    }
                }
                Some(command) = cmd_rx.recv() => {
                    self.handle_command(command).await;
                }
                result = self.mut_event_rx.recv() => {
                    match result{
                        Ok(event) => {
                            self.handle_app_event(event).await;
                        }
                        Err(e) => {
                            debug!("Event_rx error: {}", e);
                        }
                    }
                }
            }
        }
    }
}

pub async fn run(
    channel_hub: Arc<ChannelHub>,
    cmd_rx: mpsc::UnboundedReceiver<WebSocketCommand>,
) {
    let event_tx: broadcast::Sender<AppEvent> = channel_hub.event_tx.clone();
    let event_rx: broadcast::Receiver<AppEvent> = channel_hub.event_tx.subscribe();

    let mut ws_server = WebSocketServer::new(event_tx, event_rx);
    let serve_task = ws_server.serve(cmd_rx);

    tokio::select! {
        _ = ticker_task("WebSocket Server") => {
            info!("WebSocket ticker task completed");
        }
        _ = serve_task => {
            info!("WebSocket server task completed");
        }
    }
}