use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub p2p: P2PConfig,
    pub gossip: GossipConfig,
    pub queue: QueueConfig,
    pub logging: LoggingConfig,
    pub mdns: MdnsConfig,
    pub ws: WebSocketServerConfig,
    pub http: HttpConfig,
    pub kafka: KafkaConfig,
    pub channel: ChannelConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct GossipConfig {
    pub max_transmit_size: usize,
    pub heartbeat_interval: u64,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ServerConfig {
    pub listen_address: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct P2PConfig {
    pub protocol_name: String,
    pub max_streams: u32,
    pub max_concurrent_streams: u32,
    pub semaphore_permits: u32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct QueueConfig {
    pub capacity: usize,
    pub high_priority_threshold: usize,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LoggingConfig {
    pub level: String,
    pub enable_debug: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MdnsConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct WebSocketServerConfig {
    pub host: String,
    pub port: u16,
    pub max_clients: usize,
    pub heartbeat_interval: u32,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HttpConfig {
    pub enabled: bool,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub consumer: KafkaConsumerConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct KafkaConsumerConfig {
    pub group_id: String,
    pub default_topic: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ChannelConfig {
    pub command_buffer_size: usize,
    pub event_buffer_size: usize,
}

impl AppConfig {
    pub fn load(config_filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = format!("{}{}", config_filename, ".toml");

        if !Path::new(&config_path).exists() {
            return Err(format!("Configuration file '{}' not found", config_path).into());
        }

        let settings = config::Config::builder()
            .add_source(config::File::with_name(config_filename))
            .add_source(config::Environment::with_prefix("APP")
                .separator("__")
                .list_separator(","))
            .build()?;
        println!("Loaded configuration from '{:?}'", settings);
        let app_config: AppConfig = settings.try_deserialize()?;
        Ok(app_config)
    }
}

static SETTINGS: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    Arc::new(AppConfig::load("config").expect("Failed to load config"))
});

pub fn get_config() -> Arc<AppConfig> { SETTINGS.clone() }
