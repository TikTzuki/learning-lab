use crate::swarm::metrics::get_swarm_metrics;
use crate::swarm::{ChannelHub, KafkaConsumerCmd, SwarmCommand, WebSocketCommand};
use crate::ws::metrics::get_ws_metrics;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CommandRequest {
    pub command: String,
    pub args: Option<HashMap<String, String>>,
}

pub async fn handle_cmd(
    channel_hub: Arc<ChannelHub>,
    cmd: CommandRequest,
) -> &'static str {
    let args = cmd.args.unwrap_or_default();

    match cmd.command.as_str() {
        "swarm_send" => {
            if !args.contains_key("peer_id") || !args.contains_key("content") {
                return "Missing required parameters: peer_id and content";
            }
            let peer_id = PeerId::from_str(&args["peer_id"]).unwrap();
            let content = args["content"].clone();
            let _ = channel_hub.swarm_cmd_tx.send(SwarmCommand::SendRRMessage { to: peer_id, content });
        }
        "swarm_broadcast" => {
            if !args.contains_key("content") {
                return "Missing required parameter: content";
            }
            let content = args["content"].clone();
            let _ = channel_hub.swarm_cmd_tx.send(SwarmCommand::BroadcastGossipMessage { content });
        }
        "swarm_publish" => {
            if !args.contains_key("topic") || !args.contains_key("content") {
                return "Missing required parameters: topic and content";
            }
            let topic = args["topic"].clone();
            let content = args["content"].clone();
            let _ = channel_hub.swarm_cmd_tx.send(SwarmCommand::PublishTopic { topic, content });
        }
        "swarm_subscribe" => {
            if !args.contains_key("topic") {
                return "Missing required parameter: topic";
            }
            let topic = args["topic"].clone();
            let _ = channel_hub.swarm_cmd_tx.send(SwarmCommand::SubscribeTopic { topic });
        }
        "swarm_connect" => {
            if !args.contains_key("peer_id") || !args.contains_key("address") {
                return "Missing required parameters: peer_id and address";
            }
            let peer_id = PeerId::from_str(&args["peer_id"]).unwrap();
            let address = args["address"].clone();
            let _ = channel_hub.swarm_cmd_tx.send(SwarmCommand::ConnectToPeer { peer_id, address });
        }
        "swarm_disconnect" => {
            if !args.contains_key("peer_id") {
                return "Missing required parameter: peer_id";
            }
            let peer_id = PeerId::from_str(&args["peer_id"]).unwrap();
            let _ = channel_hub.swarm_cmd_tx.send(SwarmCommand::DisconnectFromPeer { peer_id });
        }
        "swarm_stats" => {
            let it = get_swarm_metrics();
            let stats = it.lock().await;
            return match serde_json::to_string(&*stats) {
                Ok(json) => Box::leak(json.into_boxed_str()),
                Err(_) => "Failed to serialize Swarm statistics"
            };
        }
        "ws_send" => {
            if !args.contains_key("client_id") || !args.contains_key("content") {
                return "Missing required parameters: client_id and content";
            }
            let client_id = args["client_id"].clone();
            let content = args["content"].clone();
            let _ = channel_hub.ws_cmd_tx.send(WebSocketCommand::SendToClient { client_id, content });
        }
        "ws_broadcast" => {
            if !args.contains_key("content") {
                return "Missing required parameter: content";
            }
            let content = args["content"].clone();
            let _ = channel_hub.ws_cmd_tx.send(WebSocketCommand::BroadcastToAll { content });
        }
        "ws_disconnect" => {
            let client_id = args["client_id"].clone();
            let _ = channel_hub.ws_cmd_tx.send(WebSocketCommand::DisconnectClient { client_id });
        }
        "ws_stats" => {
            let it = get_ws_metrics();
            let stats = it.lock().await;
            return match serde_json::to_string(&*stats) {
                Ok(json) => Box::leak(json.into_boxed_str()),
                Err(_) => "Failed to serialize WebSocket statistics"
            };
        }
        "kafka_subscribe" => {
            let topic = args["topic"].clone();
            let _ = channel_hub.kafka_cmd_tx.send(KafkaConsumerCmd::Subscribe(topic)).await;
        }
        "kafka_unsubscribe" => {
            let topic = args["topic"].clone();
            let _ = channel_hub.kafka_cmd_tx.send(KafkaConsumerCmd::Unsubscribe(topic)).await;
        }
        "h" => {
            return "Available commands:\n\
                    \n\
                    Swarm Commands:\n\
                    - swarm_send: Send message to specific peer (args: peer_id, content)\n\
                    - swarm_broadcast: Broadcast message to all peers (args: content)\n\
                    - swarm_publish: Publish message to topic (args: topic, content)\n\
                    - swarm_subscribe: Subscribe to topic (args: topic)\n\
                    - swarm_connect: Connect to peer (args: peer_id, address)\n\
                    - swarm_disconnect: Disconnect from peer (args: peer_id)\n\
                    \n\
                    WebSocket Commands:\n\
                    - ws_send: Send message to specific client (args: client_id, content)\n\
                    - ws_broadcast: Broadcast message to all WebSocket clients (args: content)\n\
                    - ws_disconnect: Disconnect WebSocket client (args: client_id)\n\
                    \n\
                    Kafka Commands:\n\
                    - kafka_subscribe: Subscribe to Kafka topic (args: topic)\n\
                    - kafka_unsubscribe: Unsubscribe from Kafka topic (args: topic)\n\
                    \n\
                    Other:\n\
                    - h: Show this help message"
        }
        _ => {
            return "Unknown command, available commands are: \
                    swarm_send, swarm_broadcast, swarm_publish, swarm_subscribe, \
                    swarm_connect, swarm_disconnect, ws_send, ws_broadcast, ws_disconnect, \
                    kafka_subscribe, kafka_unsubscribe"
        }
    }

    "Command executed successfully"
}