// use std::net::UdpSocket;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::thread::{sleep, Builder, JoinHandle};
// use std::time::Duration;
// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum StreamerError {
//     #[error("I/O error")]
//     Io(#[from] std::io::Error),
//
//     #[error("receive timeout error")]
//     RecvTimeout(#[from] RecvTimeoutError),
//
//     #[error("send packets error")]
//     Send(#[from] SendError<PacketBatch>),
//
//     #[error(transparent)]
//     SendPktsError(#[from] SendPktsError),
// }
//
// pub type Result<T> = std::result::Result<T, StreamerError>;
// fn recv_loop(
//     socket: &UdpSocket,
//     exit: &AtomicBool,
//     packet_batch_sender: &PacketBatchSender,
//     recycler: &PacketBatchRecycler,
//     stats: &StreamerReceiveStats,
//     coalesce: Duration,
//     use_pinned_memory: bool,
//     in_vote_only_mode: Option<Arc<AtomicBool>>,
// ) -> Result<()> {
//     loop {
//         let mut packet_batch = if use_pinned_memory {
//             PacketBatch::new_with_recycler(recycler, PACKETS_PER_BATCH, stats.name)
//         } else {
//             PacketBatch::with_capacity(PACKETS_PER_BATCH)
//         };
//         loop {
//             // Check for exit signal, even if socket is busy
//             // (for instance the leader transaction socket)
//             if exit.load(Ordering::Relaxed) {
//                 return Ok(());
//             }
//
//             if let Some(ref in_vote_only_mode) = in_vote_only_mode {
//                 if in_vote_only_mode.load(Ordering::Relaxed) {
//                     sleep(Duration::from_millis(1));
//                     continue;
//                 }
//             }
//
//             if let Ok(len) = packet::recv_from(&mut packet_batch, socket, coalesce) {
//                 if len > 0 {
//                     let StreamerReceiveStats {
//                         packets_count,
//                         packet_batches_count,
//                         full_packet_batches_count,
//                         max_channel_len,
//                         ..
//                     } = stats;
//
//                     packets_count.fetch_add(len, Ordering::Relaxed);
//                     packet_batches_count.fetch_add(1, Ordering::Relaxed);
//                     max_channel_len.fetch_max(packet_batch_sender.len(), Ordering::Relaxed);
//                     if len == PACKETS_PER_BATCH {
//                         full_packet_batches_count.fetch_add(1, Ordering::Relaxed);
//                     }
//
//                     packet_batch_sender.send(packet_batch)?;
//                 }
//                 break;
//             }
//         }
//     }
// }
//
// pub fn receiver(
//     thread_name: String,
//     socket: Arc<UdpSocket>,
//     exit: Arc<AtomicBool>,
//     packet_batch_sender: PacketBatchSender,
//     recycler: PacketBatchRecycler,
//     stats: Arc<StreamerReceiveStats>,
//     coalesce: Duration,
//     use_pinned_memory: bool,
//     in_vote_only_mode: Option<Arc<AtomicBool>>,
// ) -> JoinHandle<()> {
//     let res = socket.set_read_timeout(Some(Duration::new(1, 0)));
//     assert!(res.is_ok(), "streamer::receiver set_read_timeout error");
//     Builder::new()
//         .name(thread_name)
//         .spawn(move || {
//             let _ = recv_loop(
//                 &socket,
//                 &exit,
//                 &packet_batch_sender,
//                 &recycler,
//                 &stats,
//                 coalesce,
//                 use_pinned_memory,
//                 in_vote_only_mode,
//             );
//         })
//         .unwrap()
// }
