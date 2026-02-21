use crate::ws::WebSocketClients;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use crate::ws_handle_benchmark::handle_benchmark_connection;

type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    c: i32,
}
fn notify(mut ws: &Tx, msg: Message) {
    let request: Request = serde_json::from_str(msg.to_text().unwrap()).unwrap();
    let ts = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as i64,
        Err(_e) => 0,
    };
    let message = json!({
        "c": request.c,
        "ts": ts
    });
    ws.unbounded_send(Message::Text(message.to_string().into()))
        .unwrap()
}
pub async fn handle_connection(
    peer_map: WebSocketClients,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();

    // {
    //     peer_map.lock().await.insert(addr.to_string(), tx);
    // }

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        // println!(
        //     "Received a message from {}: {}",
        //     addr,
        //     msg.to_text().unwrap()
        // );

        // let peers = peer_map.lock();
        // notify(peers.get(&&addr).unwrap(), msg.clone());

        // We want to broadcast the message to everyone except ourselves.
        // let broadcast_recipients = peers
        //     .iter()
        //     .filter(|(peer_addr, _)| peer_addr != &&addr)
        //     .map(|(_, ws_sink)| ws_sink);

        // for recp in broadcast_recipients {
        //     recp.unbounded_send(msg.clone()).unwrap();
        // }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;
    println!("{} disconnected", &addr);
    // peer_map.lock().remove(&addr);
}

pub async fn benchmark_server() {
    // let state = crate::ws_server::PeerMap::new(Mutex::new(HashMap::new()));
    // Create the event loop and TCP listener we'll accept connections on.

    let addr = "127.0.0.1:8080";
    let try_socket = TcpListener::bind(addr).await;
    let listener = try_socket.expect("Failed to bind");

    println!("WebSocket server listening on ws://{}", addr);
    let ws_clients = WebSocketClients::default();
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);
        tokio::spawn(handle_benchmark_connection(ws_clients.clone(), stream, addr));
    }
}
