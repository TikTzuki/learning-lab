// use crate::v2::{ticker_task, AppEvent, AppSwarmEvent, ChannelHub, WebSocketCommand};
// use log::info;
// use std::sync::Arc;
// use tokio::sync::broadcast;
//
// struct WebSocketDistributor {
//     channel_hub: Arc<ChannelHub>,
//     event_rx: broadcast::Receiver<AppEvent>,
// }
//
// impl WebSocketDistributor {
//     pub async fn serve(&mut self) {
//         while let Ok(event) = self.event_rx.recv().await {
//             match event {
//                 AppEvent::Swarm(AppSwarmEvent::MessageReceived { from, content, .. }) => {
//                     info!("WebSocketDistributor received message from {}: {}", from, content);
//                     let _ = self.channel_hub.ws_cmd_tx.send(
//                         WebSocketCommand::BroadcastToAll {
//                             content,
//                         }
//                     ).await;
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
//
// pub async fn run(
//     channel_hub: Arc<ChannelHub>,
// ) {
//     let event_rx = channel_hub.event_tx.subscribe();
//     let mut distributor = WebSocketDistributor { channel_hub, event_rx };
//     tokio::select! {
//         _ = ticker_task("WebSocket Distributor") => {
//             info!("WebSocket distributor ticker task completed");
//         }
//         _ =
//         distributor.serve() => {
//             info!("WebSocket distributor task completed");
//         }
//     }
// }