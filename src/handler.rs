use futures::future::BoxFuture;
use futures::prelude::*;
use libp2p_core::upgrade::ReadyUpgrade;
use libp2p_identity::PeerId;
use libp2p_swarm::handler::{ConnectionEvent, FullyNegotiatedInbound, FullyNegotiatedOutbound};
use libp2p_swarm::{
    ConnectionHandler, ConnectionHandlerEvent, KeepAlive, Stream, StreamProtocol, SubstreamProtocol,
};
use std::{
    io,
    task::{Context, Poll},
};
use void::Void;

type InOutBoundFuture = BoxFuture<'static, Result<Stream, io::Error>>;

const PROTOCOL_NAME: StreamProtocol = StreamProtocol::new("/ipfs/ping/1.0.0");

pub struct Handler {
    // _interval: Delay,
    outbound: Option<InOutBoundFuture>,
    inbound: Option<InOutBoundFuture>,
    // _peer: PeerId,
}

impl Handler {
    pub fn new(_peer: PeerId) -> Self {
        Handler {
            // peer,
            // interval: Delay::new(Duration::new(0, 0)),
            outbound: None,
            inbound: None,
        }
    }
}

impl ConnectionHandler for Handler {
    type FromBehaviour = Void;
    type ToBehaviour = Void;
    type Error = Void;
    type InboundProtocol = ReadyUpgrade<StreamProtocol>;
    type OutboundProtocol = ReadyUpgrade<StreamProtocol>;
    type OutboundOpenInfo = ();
    type InboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<ReadyUpgrade<StreamProtocol>, ()> {
        SubstreamProtocol::new(ReadyUpgrade::new(PROTOCOL_NAME), ())
    }

    fn on_behaviour_event(&mut self, _: Void) {}

    fn connection_keep_alive(&self) -> KeepAlive {
        KeepAlive::Yes
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
    ) -> Poll<ConnectionHandlerEvent<ReadyUpgrade<StreamProtocol>, (), Void, Self::Error>> {
        Poll::Pending
    }

    fn on_connection_event(
        &mut self,
        event: ConnectionEvent<
            Self::InboundProtocol,
            Self::OutboundProtocol,
            Self::InboundOpenInfo,
            Self::OutboundOpenInfo,
        >,
    ) {
        match event {
            ConnectionEvent::FullyNegotiatedInbound(FullyNegotiatedInbound {
                protocol: stream,
                ..
            }) => {
                self.inbound = Some(async { Ok(stream) }.boxed());
            }
            ConnectionEvent::FullyNegotiatedOutbound(FullyNegotiatedOutbound {
                protocol: stream,
                ..
            }) => {
                self.outbound = Some(async { Ok(stream) }.boxed());
            }
            ConnectionEvent::DialUpgradeError(_)
            | ConnectionEvent::AddressChange(_)
            | ConnectionEvent::ListenUpgradeError(_)
            | ConnectionEvent::LocalProtocolsChange(_)
            | ConnectionEvent::RemoteProtocolsChange(_) => {}
        }
    }
}
