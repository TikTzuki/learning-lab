use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyMessage {
    pub id: String, // Unique identifier for the message
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
}

impl MyMessage {
    pub fn new(sender: String, content: String) -> Self {
        MyMessage {
            id: Uuid::new_v4().to_string(),
            sender,
            content,
            timestamp: Instant::now().elapsed().as_secs(),
        }
    }
}
