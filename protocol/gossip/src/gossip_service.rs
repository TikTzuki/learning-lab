// use std::net::UdpSocket;
// use std::sync::Arc;
// use std::time::Duration;
// use tokio::sync::mpsc::unbounded_channel;
// use log::trace;
// 
// pub fn new(
//     gossip_socket: UdpSocket,
// ){
//     let (request_sender, request_receiver) = unbounded_channel();
//     let gossip_socket = Arc::new(gossip_socket);
//     trace!(
//             "GossipService: id: {}, listening on: {:?}",
//             &cluster_info.id(),
//             gossip_socket.local_addr().unwrap()
//         );
//     let socket_addr_space = *cluster_info.socket_addr_space();
//     let t_receiver = streamer::receiver(
//         "solRcvrGossip".to_string(),
//         gossip_socket.clone(),
//         exit.clone(),
//         request_sender,
//         Recycler::default(),
//         Arc::new(StreamerReceiveStats::new("gossip_receiver")),
//         Duration::from_millis(1), // coalesce
//         false,
//         None,
//     );
// }