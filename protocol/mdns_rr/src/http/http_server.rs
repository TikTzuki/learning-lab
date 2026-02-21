use crate::http::command_handler::{handle_cmd, CommandRequest};
use crate::settings::get_config;
use crate::swarm::{ticker_task, ChannelHub, KafkaConsumerCmd};
use axum::extract::{Json, Query};
use axum::routing::{get, post, IntoMakeService};
use axum::Router;
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;

pub async fn run(channel_hub: Arc<ChannelHub>) {
    let kafka_tx: Sender<KafkaConsumerCmd> = channel_hub.kafka_cmd_tx.clone();
    let router = Router::new()
        .route("/kafka/subscribe", get(|query: Query<HashMap<String, String>>| async move {
            info!("Received request to subscribe to Kafka topic");
            let topic = query.get("topic")
                .expect("Topic parameter is required");

            kafka_tx.send(KafkaConsumerCmd::Subscribe(topic.clone()))
                .await
                .expect("Failed to send subscribe command to Kafka consumer manager");

            "Subscription request received"
        }))
        .route("/commands", post(|Json(cmd): Json<CommandRequest>| async move {
            handle_cmd(channel_hub.clone(), cmd).await
        }))
        .route("/health", get(|| async { "OK" }));

    let addr: SocketAddr = get_config().http.address.parse().expect("Invalid metrics address format");
    let tcp_listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("HTTP server listening on {}", addr);
            listener
        }
        Err(e) => {
            error!("Failed to bind Http server to {}: {}", addr, e);
            return;
        }
    };

    // Create the axum server task
    let server_task = axum::serve(tcp_listener, router.into_make_service());

    // Run both tasks concurrently
    tokio::join!(
        ticker_task("HTTP"),
        server_task,
    );
}
