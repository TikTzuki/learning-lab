use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    c: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    c: i32,
    ts: i64,
}

pub struct WebSocketClient {
    url: String,
    message_count: Arc<AtomicU64>,
    sent_count: Arc<AtomicU64>,
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
}

impl WebSocketClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            message_count: Arc::new(AtomicU64::new(0)),
            sent_count: Arc::new(AtomicU64::new(0)),
            reconnect_attempts: 0,
            max_reconnect_attempts: 10,
        }
    }

    pub async fn connect_and_run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.try_connect().await {
                Ok(_) => {
                    info!("WebSocket connection closed normally");
                    self.reconnect_attempts = 0;
                }
                Err(e) => {
                    error!("WebSocket connection failed: {}", e);
                    self.reconnect_attempts += 1;

                    if self.reconnect_attempts >= self.max_reconnect_attempts {
                        error!("Max reconnection attempts reached. Giving up.");
                        return Err(e);
                    }

                    let backoff_duration = Duration::from_secs(2_u64.pow(self.reconnect_attempts.min(6)));
                    warn!("Reconnecting in {:?} (attempt {}/{})",
                          backoff_duration, self.reconnect_attempts, self.max_reconnect_attempts);
                    sleep(backoff_duration).await;
                }
            }
        }
    }

    async fn try_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Connecting to WebSocket server at {}", self.url);
        let (ws_stream, _) = connect_async(&self.url).await?;
        info!("WebSocket connection established");

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Send initial test message
        let test_message = Request { c: 1 };
        let message = Message::Text(serde_json::to_string(&test_message)?);
        ws_sender.send(message).await?;
        self.sent_count.fetch_add(1, Ordering::Relaxed);
        info!("Sent initial message");

        // Start statistics reporting task
        let stats_handle = {
            let message_count = self.message_count.clone();
            let sent_count = self.sent_count.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    let received = message_count.load(Ordering::Relaxed);
                    let sent = sent_count.load(Ordering::Relaxed);
                    info!("Stats: Sent: {}, Received: {}, Rate: {} msg/s",
                          sent, received, received as f64 / 10.0);
                }
            })
        };

        // Main message receiving loop
        while let Some(message) = ws_receiver.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    self.message_count.fetch_add(1, Ordering::Relaxed);

                    match serde_json::from_str::<Response>(&text) {
                        Ok(response) => {
                            let count = self.message_count.load(Ordering::Relaxed);
                            info!("Message #{}: Counter: {}, Timestamp: {}",
                                  count, response.c, response.ts);
                        }
                        Err(_) => {
                            let count = self.message_count.load(Ordering::Relaxed);
                            info!("Message #{}: Raw: {}", count, text);
                        }
                    }
                }
                Ok(Message::Close(frame)) => {
                    info!("WebSocket closed: {:?}", frame);
                    break;
                }
                Ok(Message::Ping(data)) => {
                    info!("Received ping, sending pong");
                    // The underlying library should handle this automatically
                }
                Ok(Message::Pong(_)) => {
                    info!("Received pong");
                }
                Ok(_) => {
                    // Handle other message types if needed
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }

        // Clean up tasks
        // sender_handle.abort();
        stats_handle.abort();

        let final_count = self.message_count.load(Ordering::Relaxed);
        let final_sent = self.sent_count.load(Ordering::Relaxed);
        info!("Connection ended. Total sent: {}, Total received: {}", final_sent, final_count);

        Ok(())
    }

    pub fn get_message_count(&self) -> u64 {
        self.message_count.load(Ordering::Relaxed)
    }

    pub fn get_sent_count(&self) -> u64 {
        self.sent_count.load(Ordering::Relaxed)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let ws_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "ws://127.0.0.1:8081".to_string());

    info!("Starting WebSocket client, connecting to: {}", ws_url);

    let mut client = WebSocketClient::new(ws_url);

    // Handle Ctrl+C gracefully
    let client_stats = (client.message_count.clone(), client.sent_count.clone());
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        let (msg_count, sent_count) = client_stats;
        let received = msg_count.load(Ordering::Relaxed);
        let sent = sent_count.load(Ordering::Relaxed);
        println!("Shutting down... Final stats: Sent: {}, Received: {}", sent, received);
        std::process::exit(0);
    });

    client.connect_and_run().await?;

    Ok(())
}