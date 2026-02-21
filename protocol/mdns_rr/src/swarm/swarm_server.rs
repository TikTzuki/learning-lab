use crate::kafka::broadcast_kafka_topic_handler::MarketData;
use crate::message::MyMessage;
use crate::settings::get_config;
use crate::swarm::metrics::{increase_sw_gossip_received, increase_sw_gossip_sent, increase_sw_tx_message_received};
use crate::swarm::{ticker_task, AppEvent, AppSwarmEvent, ChannelHub, ChannelReceivers, KafkaConsumerEvent, SwarmCommand};
use axum::routing::get;
use futures_util::StreamExt;
use libp2p::gossipsub::{Behaviour as GossipBehaviour, ConfigBuilder as GossipConfigBuilder, Event as GossipEvent, IdentTopic, Message as GossipMessage, MessageAuthenticity, MessageId as GossipMessageId, MessageId, PublishError, SubscriptionError, TopicScoreParams, ValidationMode};
use libp2p::request_response::{
    cbor::Behaviour as RequestResponseBehavior,
    Config as RequestResponseConfig,
    Event as RREvent,
    Message as RRMessage,
};
use libp2p::{
    identify, mdns, noise::Config as NoiseConfig,
    swarm::{NetworkBehaviour, SwarmEvent}, tcp::Config as TcpConfig, yamux::Config as YamuxConfig, PeerId, Stream, StreamProtocol, Swarm, SwarmBuilder};
use libp2p_stream::Behaviour as StreamBehaviour;
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use std::char::ToLowercase;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::io;
use tokio::sync::broadcast::error::SendError;
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use tracing_subscriber::fmt::format;

#[derive(NetworkBehaviour)]
pub struct SwarmBehaviour {
    pub mdns: mdns::tokio::Behaviour,
    pub identify: identify::Behaviour,
    pub request_response: RequestResponseBehavior<String, Option<String>>,
    pub gossipsub: GossipBehaviour,
    pub stream: StreamBehaviour,
}

pub struct SwarmServer {
    swarm: Swarm<SwarmBehaviour>,
    event_tx: broadcast::Sender<AppEvent>,
    peers: HashMap<PeerId, String>,
    pub response_messages: Arc<RwLock<Vec<String>>>,
}

impl SwarmServer {
    fn _new(
        swarm: Swarm<SwarmBehaviour>,
        event_tx: broadcast::Sender<AppEvent>,
    ) -> Self {
        Self {
            swarm,
            event_tx,
            peers: HashMap::new(),
            response_messages: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn new(event_tx: broadcast::Sender<AppEvent>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(TcpConfig::default(),
                      NoiseConfig::new,
                      move || {
                          let mut it = YamuxConfig::default();
                          it.set_max_num_streams(4_000);
                          it
                      },
            )?
            .with_quic()
            .with_behaviour(|key| {
                // rr
                let config = &get_config();
                let protocol_name = config.p2p.protocol_name.as_str();
                let rr_protocol = StreamProtocol::try_from_owned(String::from(protocol_name))
                    .expect("Failed to create StreamProtocol");
                let rr_config = RequestResponseConfig::default()
                    .with_max_concurrent_streams(2_000);

                info!("Swarm peer_id: {}, protocol: {}", key.public().to_peer_id(), protocol_name);

                let message_id_fn = |message: &GossipMessage| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    GossipMessageId::from(s.finish().to_string())
                };

                let gossip_setting = &config.gossip;
                let gossipsub_config = GossipConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(gossip_setting.heartbeat_interval))
                    .validation_mode(ValidationMode::Strict)
                    // .validate_messages() // IMPORTANT! prevent forward
                    .message_id_fn(message_id_fn)
                    .max_transmit_size(gossip_setting.max_transmit_size)
                    .build()
                    .map_err(io::Error::other)?;

                // build a gossipsub network behaviour
                let gossipsub = GossipBehaviour::new(
                    MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                Ok(SwarmBehaviour {
                    mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?,
                    identify: identify::Behaviour::new(identify::Config::new(
                        "/ipfs/0.1.0".into(),
                        key.public(),
                    )),
                    request_response: RequestResponseBehavior::new(
                        [(rr_protocol, libp2p::request_response::ProtocolSupport::Full)],
                        rr_config,
                    ),
                    gossipsub,
                    stream: StreamBehaviour::new(),
                })
            })?
            // .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        // Listen on all interfaces
        // swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        let topic = IdentTopic::new("broadcast");

        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        let mut control = swarm.behaviour_mut().stream.new_control();
        let mut incoming = control.accept(StreamProtocol::new("/my-protocol")).unwrap();

        Ok(Self::_new(
            swarm,
            event_tx,
        ))
    }

    async fn handle_command(&mut self, command: SwarmCommand) {
        match command {
            // rr
            SwarmCommand::BroadcastRRMessage { content } => {
                self.peers.iter().for_each(|(to, v)| {
                    self.swarm.behaviour_mut().request_response
                        .send_request(&to, content.clone());
                });
            }
            SwarmCommand::SendRRMessage { to, content } => {
                // debug!("Sending message to peer {}: {}", to, content);
                // let message = MyMessage::new(
                //     self.swarm.local_peer_id().to_string(),
                //     content.clone(),
                // );
                // self.swarm.behaviour_mut().request_response.send_request(&to, message);
                //
                // // Emit event
                // let _ = self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::MessageSent { to, content }));
            }
            // gossipsub
            SwarmCommand::BroadcastGossipMessage { content } => {
                self.publish("broadcast".to_string(), content.clone()).await;
            }
            SwarmCommand::PublishTopic { topic, content } => {
                self.publish(topic.clone(), content.clone()).await;
            }
            SwarmCommand::SubscribeTopic { topic } => {
                let ident_topic = IdentTopic::new(topic.clone());
                if let Err(e) = self.swarm.behaviour_mut().gossipsub.subscribe(&ident_topic) {
                    error!("SwarmCommand::SubscribeTopic: Subscribe error: {e:?}");
                } else {
                    info!("Subscribed to topic: {}", topic.clone());
                }
            }
            // connection
            SwarmCommand::ConnectToPeer { peer_id, address } => {
                // info!("Connecting to peer {} at {}", peer_id, address);
                // let _ = self.swarm.dial(peer_id)
                //     .expect(format!("Failed to dial peer {}", address).as_str());
            }
            SwarmCommand::DisconnectFromPeer { peer_id } => {
                //     info!("Disconnecting from peer {}", peer_id);
                //     self.peers.remove(&peer_id);
                //     let _ = self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::PeerDisconnected { peer_id }));
            }
        }
    }

    async fn publish(&mut self, topic: String, content: String) {
        let ident_topic = IdentTopic::new(topic.clone());
        match self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(ident_topic, content.as_bytes()) {
            Ok(_) => {
                increase_sw_gossip_sent().await;
            }
            Err(e) => {
                error!("SwarmCommand::BroadcastMessage: Publish error: {e:?}");
            }
        }
    }

    async fn handle_swarm_event(&mut self, event: SwarmEvent<SwarmBehaviourEvent>) {
        match event {
            SwarmEvent::Behaviour(SwarmBehaviourEvent::RequestResponse(event)) => {
                match event {
                    RREvent::Message { message, .. } => match message {
                        RRMessage::Request {
                            request,
                            channel,
                            request_id,
                        } => {
                            match self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::MessageReceived {
                                from: self.swarm.local_peer_id().clone(),
                                content: request,
                            })) {
                                Ok(_) => {
                                    increase_sw_tx_message_received().await;
                                }
                                Err(e) => {
                                    warn!("tx AppSwarmEvent::MessageReceived fail {}", e);
                                }
                            };

                            match self.swarm
                                .behaviour_mut()
                                .request_response
                                .send_response(channel, None) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Failed to send response for request_id {}: {:?}", request_id, e);
                                }
                            }
                        }
                        RRMessage::Response {
                            request_id,
                            response,
                        } => {
                            // let mut count = self.response_messages.write().await;
                            // count.push(response.id.clone());
                            // debug!(
                            //     "Response message #{} {} for request_id: {}",
                            //     count.len(),
                            //     response.id.clone(),
                            //     request_id
                            // );
                        }
                    },
                    RREvent::InboundFailure {
                        peer,
                        request_id,
                        error,
                        ..
                    } => { warn!("RequestResponseEvent::InboundFailure -> PeerID: {peer} | RequestID: {request_id} | Error: {error}") }
                    RREvent::ResponseSent {
                        peer, request_id, ..
                    } => {
                        debug!("RequestResponseEvent::ResponseSent -> PeerID: {peer} | RequestID: {request_id}");
                    }
                    RREvent::OutboundFailure {
                        peer,
                        request_id,
                        error,
                        ..
                    } => { warn!("RequestResponseEvent::OutboundFailure -> PeerID: {peer} | RequestID: {request_id} | Error: {error}") }
                }
            }
            SwarmEvent::Behaviour(SwarmBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
                for (peer_id, multiaddr) in peers {
                    info!("mdns::Event::Discovered: {}", peer_id);

                    self.peers.insert(peer_id, multiaddr.to_string());

                    // Request_response
                    // self.swarm.dial(peer_id).expect("Failed to dial discovered peer");

                    // Gossipsub
                    self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    // let _ = self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::PeerDiscovered {
                    //     peer_id,
                    //     address: multiaddr.to_string(),
                    // }));

                    // Stream
                }
            }
            SwarmEvent::Behaviour(SwarmBehaviourEvent::Mdns(mdns::Event::Expired(peers))) => {
                for (peer_id, _addr) in peers {
                    info!("mdns::Event::Expired: {}", peer_id);
                    self.peers.remove(&peer_id);
                    self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    let _ = self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::PeerDisconnected { peer_id }));
                }
            }
            SwarmEvent::Behaviour(SwarmBehaviourEvent::Gossipsub(GossipEvent::Subscribed { peer_id, topic })) => {
                info!("SwarmEvent::Gossipsub: Subscribed to topic '{}' by peer {}", topic, peer_id);
            }
            SwarmEvent::Behaviour(SwarmBehaviourEvent::Gossipsub(GossipEvent::Message {
                                                                     propagation_source,
                                                                     message_id,
                                                                     message,
                                                                     ..
                                                                 })) => {
                increase_sw_gossip_received().await;
                match String::from_utf8(message.data) {
                    Ok(data) => {
                        match self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::MessageReceived {
                            from: self.swarm.local_peer_id().clone(),
                            content: data,
                        })) {
                            Ok(_) => {
                                increase_sw_tx_message_received().await;
                            }
                            Err(e) => {
                                warn!("tx AppSwarmEvent::MessageReceived fail {}", e);
                            }
                        };
                    }
                    Err(e) => { error!("Failed to decode message data: {}", e); }
                }
            }
            SwarmEvent::Behaviour(SwarmBehaviourEvent::Gossipsub(GossipEvent::SlowPeer { failed_messages, .. })) => {
                warn!("GossipEvent::SlowPeer {:?}", failed_messages);
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                info!("SwarmEvent::ConnectionEstablished: {} {:?}", peer_id, endpoint);
                self.peers.insert(peer_id, endpoint.get_remote_address().to_string());
                // let _ = self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::PeerDiscovered {
                //     peer_id,
                //     address: endpoint.get_remote_address().to_string(),
                // }));
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                info!("SwarmEvent::ConnectionClosed: Connection closed with peer: {} {:?}", peer_id, cause);
                self.peers.remove(&peer_id);
                let _ = self.event_tx.send(AppEvent::Swarm(AppSwarmEvent::PeerDisconnected { peer_id }));
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {}", address);
            }
            _ => {}
        }
    }

    async fn serve(&mut self, mut cmd_rx: mpsc::UnboundedReceiver<SwarmCommand>) {
        loop {
            tokio::select! {
                Some(command) = cmd_rx.recv() => {
                    self.handle_command(command).await;
                }
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event).await;
                }
            }
        }
    }
}

pub async fn run(channel_hub: Arc<ChannelHub>, cmd_rx: mpsc::UnboundedReceiver<SwarmCommand>) {
    let event_tx: broadcast::Sender<AppEvent> = channel_hub.event_tx.clone();
    let mut swarm_server = match SwarmServer::new(event_tx).await {
        Ok(server) => server,
        Err(e) => {
            error!("Failed to create swarm server: {}", e);
            return;
        }
    };

    let serve_task = swarm_server.serve(cmd_rx);

    tokio::select! {
        _ = ticker_task("Swarm") => {
            info!("Swarm ticker task completed");
        }
        _ = serve_task => {
            info!("Swarm server task completed");
        }
    }
}