use crate::app::App;
use crate::kafka::broadcast_kafka_topic_handler::BroadcastKafkaTopicHandler;
use crate::kafka::kafka_consumer_task::spawn_kafka_consumer_task;
use crate::settings::get_config;
use crate::swarm::{ticker_task, AppEvent, ChannelHub, ChannelReceivers, KafkaConsumerCmd, WebSocketCommand};
use futures_util::future::err;
use log::{error, info};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, Message};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{broadcast::Sender, mpsc};
use tokio::task::JoinHandle;

pub struct KafkaConsumerManager {
    channel_hub: Arc<ChannelHub>,
    active_tasks: HashMap<String, JoinHandle<()>>,

}

impl KafkaConsumerManager {
    async fn new(channel_hub: Arc<ChannelHub>) -> Self {
        Self {
            channel_hub,
            active_tasks: HashMap::new(),
        }
    }

    async fn command_handler(&mut self, cmd: KafkaConsumerCmd) {
        match cmd {
            KafkaConsumerCmd::Subscribe(topic) => {
                // Logic to subscribe to the topic
                let handler = Box::new(BroadcastKafkaTopicHandler::new(
                    topic.as_str(),
                    self.channel_hub.ws_cmd_tx.clone(),
                    self.channel_hub.swarm_cmd_tx.clone(),
                ));
                match spawn_kafka_consumer_task(topic.as_str(), handler) {
                    Ok(task) => {
                        self.active_tasks.insert(topic.to_string(), task);
                    }
                    Err(_) => {
                        error!("Failed to spawn Kafka consumer task");
                    }
                }
            }
            KafkaConsumerCmd::Unsubscribe(topic) => {
                // Logic to unsubscribe from the topic
                println!("Unsubscribing from topic: {}", topic);
            }
        }
    }

    // async fn add_new_consumer(&mut self, topic: &str) {
    //     let config = get_config();
    //     let consumer1: StreamConsumer = ClientConfig::new()
    //         .set("group.id", &config.kafka.consumer.group_id)
    //         .set("bootstrap.servers", &config.kafka.bootstrap_servers)
    //         .set("enable.partition.eof", "false")
    //         .set("session.timeout.ms", "6000")
    //         .set("enable.auto.commit", "true")
    //         .set("auto.offset.reset", "latest")
    //         // .set("auto.offset.reset", "earliest")
    //         .create()
    //         .expect("Failed to create Kafka consumer");
    //
    //     println!("Subscribing to topic: {}", topic);
    //     let consumer = Arc::new(consumer1);
    //     consumer.subscribe(&[topic]).expect("TODO: panic message");
    //
    //     self.consumers.insert(topic.to_string(), consumer.clone());
    //
    //     loop {
    //         match consumer.clone().recv().await {
    //             Ok(msg) => {
    //                 if let Some(payload) = msg.payload_view::<str>() {
    //                     match payload {
    //                         Ok(kafka_message) => {
    //                             info!("Received Kafka message: {}", kafka_message);
    //                         }
    //                         Err(e) => {
    //                             info!("Error decoding Kafka message payload: {}", e);
    //                         }
    //                     }
    //                 } else {
    //                     info!("Received empty Kafka message");
    //                 }
    //             }
    //             Err(_e) => error!("Kafka consumer error: {}", _e)
    //         }
    //     }
    // }

    async fn serve(&mut self, mut rx: Receiver<KafkaConsumerCmd>) {
        while let Some(cmd) = rx.recv().await {
            self.command_handler(cmd).await;
        }
    }
}


pub async fn run(channel_hub: Arc<ChannelHub>, kafka_cmd_rx: Receiver<KafkaConsumerCmd>) {
    let mut it = KafkaConsumerManager::new(channel_hub).await;
    let it_task = it.serve(kafka_cmd_rx);

    tokio::select! {
        _ = ticker_task("Kafka") => {
            info!("Ticker task completed");
        }
        _ = it_task => {
            info!("KafkaConsumerManager task completed");
        }
    }
}
