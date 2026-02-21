mod stream;
mod gossip_service;

use {
    std::{
        collections::HashSet,
        net::{SocketAddr, TcpListener, UdpSocket},
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc, RwLock,
        },
        thread::{self, sleep, JoinHandle},
        time::{Duration, Instant},
    }
};

fn main() {}
