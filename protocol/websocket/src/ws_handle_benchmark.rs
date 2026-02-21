use std::time::{SystemTime, UNIX_EPOCH};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc::unbounded_channel;
use tungstenite::Utf8Bytes;
use uuid::Uuid;
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    c: i32,
}
fn map_message(msg: Utf8Bytes) -> Value {
    let request: Request = serde_json::from_str(msg.as_str()).unwrap();
    let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as i64,
        Err(_e) => 0,
    };
    let message = json!({
        "c": request.c,
        "ts": ts
    });
    return message;
}
pub async fn handle_benchmark_connection(
    clients: crate::ws::WebSocketClients,
    stream: TcpStream,
    addr: std::net::SocketAddr,
) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");
    println!("WebSocket connection established");

    let client_id = Uuid::new_v4().to_string();
    let (tx, mut rx) = unbounded_channel::<String>();

    // Register client
    clients.lock().await.insert(client_id.clone(), tx);
    println!("WebSocket client {} connected", client_id);

    // Split the WebSocket stream into outgoing and incoming parts
    let (mut ws_outgoing, mut ws_incoming) = ws_stream.split();
    while let Some(msg) = ws_incoming.next().await {
        match msg {
            Ok(WsMessage::Text(text)) => {
                let resp = map_message(text.into());
                ws_outgoing.send(WsMessage::Text(resp.to_string().into())).await.unwrap();
            }
            Err(e) => {
                println!("Error processing WebSocket message: {}", e);
                break;
            }
            _ => {}
        }
    }
    // loop {
    //     select! {
    //         msg = ws_incoming.next() => {
    //                 match msg{
    //                     Some(Ok(WsMessage::Text(text))) => {
    //                         let resp = map_message(text);
    //                        ws_outgoing.send(WsMessage::Text(resp.to_string().into())).await.unwrap()
    //                     }
    //                 _ => {}}
    //         }
    //         // Send messages to web client
    //         Some(message) = rx.recv() => {
    //             if let Err(_) = ws_outgoing.send(WsMessage::Text(message.to_string().into())).await {
    //                 break;
    //             }
    //         }
    //     }
    // }
}
