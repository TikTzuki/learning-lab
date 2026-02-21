use crate::kafka::kafka_consumer_task::KafkaMessageHandler;
use crate::swarm::{SwarmCommand, WebSocketCommand};
use async_trait::async_trait;
use log::{debug, info, warn};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use serde_json::json;
use std::str::Utf8Error;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::UnboundedSender;

pub struct BroadcastKafkaTopicHandler {
    pub topic: String,
    pub ws_tx: UnboundedSender<WebSocketCommand>,
    pub swarm_tx: UnboundedSender<SwarmCommand>,
}

impl BroadcastKafkaTopicHandler {
    pub fn new(
        topic: &str,
        ws_tx: UnboundedSender<WebSocketCommand>,
        swarm_tx: UnboundedSender<SwarmCommand>,
    ) -> Self {
        Self { topic: String::from(topic), ws_tx, swarm_tx }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MarketData {
    pub client_id: String,
    pub round: i32,
    pub c: i32,
    pub ts: Option<u128>,
    pub content: Option<String>,
}

impl MarketData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[async_trait]
impl KafkaMessageHandler for BroadcastKafkaTopicHandler {
    async fn handle_message<'a>(&self, message: BorrowedMessage<'a>) -> Result<(), KafkaError> {
        if let Some(payload) = message.payload_view::<str>() {
            debug!("Received Kafka message from topic {}: {:?}", self.topic, payload);

            // Transform kafka data
            // let payload = payload.unwrap_or(""); Transform the payload if necessary
            match payload {
                Ok(payload) => {
                    match serde_json::from_str::<MarketData>(payload) {
                        Ok(mut market_data) => {
                            debug!("Transformed MarketData: {:?}", market_data);
                            market_data.ts = Some(
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u128 // Using seconds for i32
                            );
                            let json_string = serde_json::to_string(&market_data)
                                .unwrap_or_else(|e| {
                                    warn!("Failed to serialize MarketData: {}", e);
                                    String::new()
                                });
                            let _ = self.swarm_tx.send(SwarmCommand::BroadcastGossipMessage {
                                content: json_string.clone(),
                            });
                            let _ = self.ws_tx.send(WebSocketCommand::BroadcastToAll {
                                content: json_string,
                            });
                            // let _ = self.swarm_tx.send(SwarmCommand::BroadcastRRMessage {
                            //     content: json_string.clone(),
                            // }).await;
                        }
                        Err(e) => {
                            warn!("Error parsing MarketData: {}", e);
                        }
                    }
                }
                Err(_) => { warn!("Failed to decode message payload as UTF-8"); }
            }
        }
        Ok(())
    }
}