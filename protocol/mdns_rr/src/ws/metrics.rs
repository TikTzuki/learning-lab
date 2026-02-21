use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

static WS_METRICS: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    let mut stats = HashMap::new();
    stats.insert("event_rx:MessageReceived".to_string(), 0.to_string());
    stats.insert("messages_sent".to_string(), 0.to_string());
    stats.insert("messages_received".to_string(), 0.to_string());
    Arc::new(Mutex::new(stats))
});

pub fn get_ws_metrics() -> Arc<Mutex<HashMap<String, String>>> {
    WS_METRICS.clone()
}

pub async fn increase_stats(key: &str) {
    let it = get_ws_metrics();
    let mut stats = it.lock().await;
    let current_count = stats.get(key)
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(0);
    stats.insert(key.to_string(), (current_count + 1).to_string());
}

pub async fn increase_ws_messages_sent() {
    increase_stats("messages_sent").await;
}

pub async fn increase_ws_messages_received() {
    increase_stats("messages_received").await;
}
pub async fn increase_ws_rx_message_received() {
    increase_stats("event_rx:MessageReceived").await;
}