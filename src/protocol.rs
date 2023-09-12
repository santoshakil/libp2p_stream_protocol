// use futures::prelude::*;
// use instant::Duration;
// use libp2p_swarm::StreamProtocol;
// use rand::{distributions, thread_rng, Rng};
// use std::{io, time::Instant};

// pub const PROTOCOL_NAME: StreamProtocol = StreamProtocol::new("/ipfs/ping/1.0.0");

// #[derive(Default, Debug, Copy, Clone)]
// pub(crate) struct Ping;
// const PING_SIZE: usize = 32;

// pub(crate) async fn send_ping<S>(mut stream: S) -> io::Result<(S, Duration)>
// where
//     S: AsyncRead + AsyncWrite + Unpin,
// {
//     let payload: [u8; PING_SIZE] = thread_rng().sample(distributions::Standard);
//     stream.write_all(&payload).await?;
//     stream.flush().await?;
//     let started = Instant::now();
//     let mut recv_payload = [0u8; PING_SIZE];
//     stream.read_exact(&mut recv_payload).await?;
//     if recv_payload == payload {
//         Ok((stream, started.elapsed()))
//     } else {
//         Err(io::Error::new(
//             io::ErrorKind::InvalidData,
//             "Ping payload mismatch",
//         ))
//     }
// }

// pub(crate) async fn recv_ping<S>(mut stream: S) -> io::Result<S>
// where
//     S: AsyncRead + AsyncWrite + Unpin,
// {
//     let mut payload = [0u8; PING_SIZE];
//     stream.read_exact(&mut payload).await?;
//     stream.write_all(&payload).await?;
//     stream.flush().await?;
//     Ok(stream)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use futures::StreamExt;
//     use libp2p_core::{
//         multiaddr::multiaddr,
//         transport::{memory::MemoryTransport, ListenerId, Transport},
//     };
//     use rand::{thread_rng, Rng};

//     #[test]
//     fn ping_pong() {
//         let mem_addr = multiaddr![Memory(thread_rng().gen::<u64>())];
//         let mut transport = MemoryTransport::new().boxed();
//         transport.listen_on(ListenerId::next(), mem_addr).unwrap();

//         let listener_addr = transport
//             .select_next_some()
//             .now_or_never()
//             .and_then(|ev| ev.into_new_address())
//             .expect("MemoryTransport not listening on an address!");

//         async_std::task::spawn(async move {
//             let transport_event = transport.next().await.unwrap();
//             let (listener_upgrade, _) = transport_event.into_incoming().unwrap();
//             let conn = listener_upgrade.await.unwrap();
//             recv_ping(conn).await.unwrap();
//         });

//         async_std::task::block_on(async move {
//             let c = MemoryTransport::new()
//                 .dial(listener_addr)
//                 .unwrap()
//                 .await
//                 .unwrap();
//             let (_, rtt) = send_ping(c).await.unwrap();
//             assert!(rtt > Duration::from_secs(0));
//         });
//     }
// }
