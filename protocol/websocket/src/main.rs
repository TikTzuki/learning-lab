mod behaviour;
mod message;
mod ws;
mod ws_server;
mod ws_handle_benchmark;

use crate::behaviour::{Behaviour as CustomBehaviour, BehaviourEvent};
use libp2p::futures::StreamExt;
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::Transport;
use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use crate::ws_server::benchmark_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    benchmark_server().await;
    // let mut wss_transport = websocket::Config::new(dns::tokio::Transport::system(
    //     tcp::tokio::Transport::new(tcp::Config::default()),
    // )?);

    // // let pk = fs::read("./private.der").await?;
    // // let cert = fs::read("./fullchain.der").await?;
    // // let pk = websocket::tls::PrivateKey::new(pk);
    // // let cert = websocket::tls::Certificate::new(cert);
    // // wss_transport.set_tls_config(websocket::tls::Config::new(pk, vec![cert])?);
    //
    // // create a new libp2p node with gossipsub
    // let mut swarm = SwarmBuilder::with_new_identity()
    //     .with_tokio()
    //     .with_tcp(
    //         tcp::Config::default(),
    //         noise::Config::new,
    //         yamux::Config::default,
    //     )?
    //     .with_other_transport(|local_key| {
    //         wss_transport
    //             .upgrade(Version::V1)
    //             .authenticate(noise::Config::new(local_key).unwrap())
    //             .multiplex(yamux::Config::default())
    //     })?
    //     // .with_websocket(
    //     //     (tls::Config::new, noise::Config::new),
    //     //     yamux::Config::default,
    //     // )
    //     // .await?
    //     .with_behaviour(|_| {
    //         let ping = ping::Behaviour::default();
    //         CustomBehaviour::new(ping)
    //     })?
    //     .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))) // Allows us to observe pings indefinitely.
    //     .build();
    //
    // swarm.listen_on("/ip4/0.0.0.0/tcp/2121/ws".parse()?)?;
    // let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    // let mut pending_requests: HashMap<OutboundRequestId, PeerId> = HashMap::new();
    //
    // let mut wsapp = WSApp::new();
    // let ws_clients = wsapp.get_ws_clients_clone();
    //
    //
    // loop {
    //     select! {
    //         // Handle user input through WSApp
    //         Ok(Some(line)) = stdin.next_line() => {
    //             wsapp.handle_input_line(line).await;
    //         }
    //         // Handle swarm events
    //         event = swarm.select_next_some() => {
    //             handle_swarm_event(&mut swarm, event).await;
    //         }
    //     }
    // }
}

async fn handle_swarm_event(
    swarm: &mut libp2p::Swarm<CustomBehaviour>,
    event: SwarmEvent<BehaviourEvent>,
) {
    match event {
        SwarmEvent::IncomingConnection {
            connection_id,
            local_addr,
            send_back_addr,
        } => println!("Incoming connection: {connection_id} from {send_back_addr} to {local_addr}"),
        SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address}"),
        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
            println!("Connected to peer: {peer_id}");
        }
        SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
            println!("Connection closed with {peer_id}: {cause:?}");
        }
        SwarmEvent::Behaviour(BehaviourEvent::Ping(event)) => {
            println!("Ping: {event:?}");
        }
        _ => {}
    }
}
