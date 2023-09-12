#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod handler;
mod protocol;

use handler::Handler;
use libp2p_core::{Endpoint, Multiaddr};
use libp2p_identity::PeerId;
use libp2p_swarm::{
    behaviour::FromSwarm, ConnectionDenied, ConnectionId, NetworkBehaviour, PollParameters,
    THandler, THandlerInEvent, THandlerOutEvent, ToSwarm,
};
use std::{
    collections::VecDeque,
    task::{Context, Poll},
};

pub struct Behaviour {
    events: VecDeque<Event>,
}

#[derive(Debug)]
pub struct Event {
    pub peer: PeerId,
    pub connection: ConnectionId,
}

impl Behaviour {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    // pub fn send_ping_to_all(&mut self) {
    //     self.events.push_front(Event {
    //         peer,
    //         connection: ConnectionId::from(0),
    //     })
    // }
}

impl Default for Behaviour {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = Handler;
    type ToSwarm = Event;

    fn handle_established_inbound_connection(
        &mut self,
        _: ConnectionId,
        peer: PeerId,
        _: &Multiaddr,
        _: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(Handler::new(peer))
    }

    fn handle_established_outbound_connection(
        &mut self,
        _: ConnectionId,
        peer: PeerId,
        _: &Multiaddr,
        _: Endpoint,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(Handler::new(peer))
    }

    fn on_connection_handler_event(
        &mut self,
        peer: PeerId,
        connection: ConnectionId,
        _result: THandlerOutEvent<Self>,
    ) {
        self.events.push_front(Event { peer, connection })
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
        _: &mut impl PollParameters,
    ) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        if let Some(e) = self.events.pop_back() {
            Poll::Ready(ToSwarm::GenerateEvent(e))
        } else {
            Poll::Pending
        }
    }

    fn on_swarm_event(&mut self, event: FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::ConnectionEstablished(_)
            | FromSwarm::ConnectionClosed(_)
            | FromSwarm::AddressChange(_)
            | FromSwarm::DialFailure(_)
            | FromSwarm::ListenFailure(_)
            | FromSwarm::NewListener(_)
            | FromSwarm::NewListenAddr(_)
            | FromSwarm::ExpiredListenAddr(_)
            | FromSwarm::ListenerError(_)
            | FromSwarm::ListenerClosed(_)
            | FromSwarm::NewExternalAddrCandidate(_)
            | FromSwarm::ExternalAddrExpired(_)
            | FromSwarm::ExternalAddrConfirmed(_) => {}
        }
    }
}
