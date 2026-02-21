use crate::message::{Message, MessageResponse};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{ping, request_response};

#[derive(NetworkBehaviour)]
pub(crate) struct Behaviour {
    ping: ping::Behaviour,
}

impl Behaviour {
    pub(crate) fn new(
        ping: ping::Behaviour,
    ) -> Self {
        Self {
            ping,
        }
    }
}
