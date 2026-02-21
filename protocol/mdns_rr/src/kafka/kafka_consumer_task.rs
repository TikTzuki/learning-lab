use crate::settings::get_config;
use async_trait::async_trait;
use log::{error, info, warn};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, Message};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

#[async_trait]
pub trait KafkaMessageHandler: Send + Sync {
    async fn handle_message<'a>(&self, message: BorrowedMessage<'a>) -> Result<(), KafkaError>;
}

pub struct KafkaConsumerTask {
    consumer: StreamConsumer,
    topic: String,
    handler: Box<dyn KafkaMessageHandler>,
}

impl KafkaConsumerTask {
    pub fn new(
        topic: &str,
        handler: Box<dyn KafkaMessageHandler>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = get_config();

        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", &config.kafka.consumer.group_id)
            .set("bootstrap.servers", &config.kafka.bootstrap_servers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .create()
            .map_err(|e| format!("Failed to create Kafka consumer: {}", e))?;

        consumer.subscribe(&[&topic])
            .map_err(|e| format!("Failed to subscribe to topic {}: {}", topic, e))?;

        info!("Created Kafka consumer for topic: {}", topic);

        Ok(KafkaConsumerTask {
            consumer,
            topic: String::from(topic),
            handler,
        })
    }

    pub async fn start_consuming(self) {
        info!("Start consume: {}", self.topic);
        while let Ok(recv_result) = self.consumer.recv().await {
            match self.handler.handle_message(recv_result).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Error handling Kafka message from topic {}: {}", self.topic, e);
                }
            }
            // if let Some(payload) = recv_result.payload_view::<str>() {
            //     info!("Received Kafka message from topic {}: {}", self.topic, payload.unwrap_or("No payload"));
            // }
        }

        // loop {
        //     match self.consumer.recv().await {
        //         Ok(msg) => {
        //             if let Some(payload) = msg.payload_view::<str>() {
        //                 match payload {
        //                     Ok(kafka_message) => {
        //                         info!("Received Kafka message from topic {}: {}", self.topic, kafka_message);
        //
        //                         // Try to parse as JSON, fallback to plain text
        //                         let response_message = match serde_json::from_str::<ResponseMessage>(kafka_message) {
        //                             Ok(parsed_msg) => parsed_msg,
        //                             Err(_) => {
        //                                 // Create a simple response message from plain text
        //                                 ResponseMessage {
        //                                     id: uuid::Uuid::new_v4().to_string(),
        //                                     content: kafka_message.to_string(),
        //                                     timestamp: std::time::SystemTime::now()
        //                                         .duration_since(std::time::UNIX_EPOCH)
        //                                         .unwrap_or_default()
        //                                         .as_secs(),
        //                                     sender: "kafka".to_string(),
        //                                 }
        //                             }
        //                         };
        //
        //                         // Send message through channel
        //                         if let Err(e) = self.message_sender.send(response_message) {
        //                             warn!("Failed to send message through channel: {}", e);
        //                         }
        //                     }
        //                     Err(e) => {
        //                         warn!("Error decoding Kafka message payload from topic {}: {}", self.topic, e);
        //                     }
        //                 }
        //             } else {
        //                 warn!("Received Kafka message with no payload from topic: {}", self.topic);
        //             }
        //         }
        //     }
        // }
    }
}

pub fn spawn_kafka_consumer_task(topic: &str, handler: Box<dyn KafkaMessageHandler>) -> Result<tokio::task::JoinHandle<()>, Box<dyn std::error::Error>> {
    match KafkaConsumerTask::new(topic, handler) {
        Ok(consumer_task) => {
            info!("New consumer task: {}", topic);
            Ok(tokio::spawn(async move {
                consumer_task.start_consuming().await;
            }))
        }
        Err(e) => {
            Err(e)
        }
    }
}
