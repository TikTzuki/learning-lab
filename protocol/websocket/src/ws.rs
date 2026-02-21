use futures_util::{SinkExt, StreamExt, TryStreamExt};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};
use uuid::Uuid;
// WebSocket client management
type ClientId = String;
pub type WebSocketClients = Arc<Mutex<HashMap<ClientId, UnboundedSender<String>>>>;

async fn handle_new_connection(stream: TcpStream, clients: WebSocketClients) {
    let mut ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let client_id = Uuid::new_v4().to_string();
    // create new channel
    let (tx, mut rx) = unbounded_channel::<String>();

    // Register client
    clients.lock().await.insert(client_id.clone(), tx);
    println!("WebSocket client {} connected", client_id);

    // Split the WebSocket stream into outgoing and incoming parts
    // Send welcome message
    let welcome = serde_json::json!({
        "type": "welcome",
        "client_id": client_id,
        "message": "Connected to libp2p WebSocket bridge"
    });
    if ws_stream.send(WsMessage::Text(welcome.to_string().into())).await.is_err() {
        return;
    }
    let (mut ws_outgoing, mut ws_incoming) = ws_stream.split();

    // if ws_outgoing
    //     .send(WsMessage::Text(welcome.to_string().into()))
    //     .await
    //     .is_err()
    // {
    //     return;
    // }

    // Handle client messages and server broadcasts
    loop {
        select! {
            // Receive messages from web client
            msg = ws_incoming.next() => {
                match msg {
                    Some(Ok(WsMessage::Text(text))) => {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                            if let Some(message_text) = parsed.get("message").and_then(|m| m.as_str()) {
                                println!("📨 Message from web client {}: {}", client_id, message_text);

                                let response:String = serde_json::json!({
                                    "type": "message_received",
                                    "from": "server",
                                    "message": format!("Server received: {}", message_text)
                                }).to_string();

                                if let Err(_) = ws_outgoing.send(WsMessage::Text(response.into())).await {
                                    break;
                                }
                            }
                        }
                    }
                    Some(Ok(WsMessage::Close(_))) => {
                        println!("WebSocket client {} disconnected", client_id);
                        break;
                    }
                    Some(Err(e)) => {
                        println!("WebSocket error for client {}: {}", client_id, e);
                        break;
                    }
                    None => break,
                _ => {}}
            }
            // Send messages to web client
            Some(message) = rx.recv() => {
                if let Err(_) = ws_outgoing.send(WsMessage::Text(message.to_string().into())).await {
                    break;
                }
            }
        }
    }
}

pub async fn handle_input_line(ws_clients: WebSocketClients, line: String) {
    let mut args = line.trim().split_whitespace();
    match args.next() {
        // ... existing /connect, /send, /peers commands ...
        Some("/broadcast") => {
            let message_content = args.collect::<Vec<_>>().join(" ");
            if !message_content.is_empty() {
                let message = serde_json::json!({
                    "type": "broadcast",
                    "from": "server",
                    "message": message_content
                });

                let clients_lock = ws_clients.lock().await;
                let client_count = clients_lock.len();

                for (client_id, sender) in clients_lock.iter() {
                    if let Err(_) = sender.send(message.to_string()) {
                        println!("Failed to send to client {}", client_id);
                    }
                }

                println!("📢 Broadcast to {} web clients: {}", client_count, message_content);
            }
        }
        Some("/clients") => {
            let clients_lock = ws_clients.lock().await;
            if clients_lock.is_empty() {
                println!("No connected web clients");
            } else {
                println!("Connected web clients ({}):", clients_lock.len());
                for client_id in clients_lock.keys() {
                    println!("  {}", client_id);
                }
            }
        }
        _ => {
            println!("Unknown command. Available commands:");
        }
    }
}
pub async fn start_console_listener(ws_clients: WebSocketClients) {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    println!("Console listener started. Type commands:");

    while let Ok(Some(line)) = lines.next_line().await {
        if line.trim().is_empty() {
            continue;
        }

        // Handle the input line
        handle_input_line(ws_clients.clone(), line).await;
    }
}
pub async fn ws_server() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind WebSocket server");
    println!("WebSocket server listening on ws://127.0.0.1:8080");
    println!("WebSocket swarm started. Commands:");
    println!("  /connect <multiaddr> - Connect to a libp2p peer");
    println!("  /send <peer_id> <message> - Send message to libp2p peer");
    println!("  /broadcast <message> - Send message to all web clients");
    println!("  /clients - List connected web clients");
    println!("  /peers - List connected libp2p peers");

    let ws_clients = WebSocketClients::default();

    // Start console input listener
    let clients_for_console = ws_clients.clone();
    tokio::spawn(start_console_listener(clients_for_console));

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New WebSocket connection from: {}", addr);
        let clients_clone = ws_clients.clone();
        tokio::spawn(handle_new_connection(stream, clients_clone));
    }
}
