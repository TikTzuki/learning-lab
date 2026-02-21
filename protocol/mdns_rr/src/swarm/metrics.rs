use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

static SWARM_METRICS: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    let mut stats = HashMap::new();
    stats.insert("event_tx:MessageReceived".to_string(), 0.to_string());
    stats.insert("gossip_sent".to_string(), 0.to_string());
    stats.insert("gossip_received".to_string(), 0.to_string());
    Arc::new(Mutex::new(stats))
});

pub fn get_swarm_metrics() -> Arc<Mutex<HashMap<String, String>>> {
    SWARM_METRICS.clone()
}
pub async fn increase_stats(key: &str) {
    let it = get_swarm_metrics();
    let mut stats = it.lock().await;
    let current_count = stats.get(key)
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(0);
    stats.insert(key.to_string(), (current_count + 1).to_string());
}

pub async fn increase_sw_gossip_sent() {
    increase_stats("gossip_sent").await;
}

pub async fn increase_sw_gossip_received() {
    increase_stats("gossip_received").await;
}
pub async fn increase_sw_tx_message_received() {
    increase_stats("event_tx:MessageReceived").await;
}
