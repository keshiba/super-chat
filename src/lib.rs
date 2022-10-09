use async_std::io;
use log::{debug, error, info, warn};
use std::error::Error;

use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    futures::{
        prelude::{stream::StreamExt, *},
        select,
    },
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{behaviour, SwarmEvent},
    Multiaddr, NetworkBehaviour, Swarm,
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
pub struct AppBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

pub enum OutEvent {
    Floodsub(FloodsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for OutEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl From<FloodsubEvent> for OutEvent {
    fn from(v: FloodsubEvent) -> Self {
        Self::Floodsub(v)
    }
}

pub async fn init() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();

    info!("Peer Id = {}", local_peer_id.to_string());

    let transport = libp2p::development_transport(local_key).await?;
    let mdns = Mdns::new(MdnsConfig::default()).await?;
    let floodsub = Floodsub::new(local_peer_id);
    let topic = floodsub::Topic::new("chat");

    let mut behaviour = AppBehaviour {
        floodsub: floodsub,
        mdns: mdns,
    };

    behaviour.floodsub.subscribe(topic.clone());
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    if let Some(remote_host) = std::env::args().nth(1) {
        let addr: Multiaddr = remote_host.parse()?;
        swarm.dial(addr)?;
        info!("Connecting to {:?}", remote_host);
    }

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

    loop {
        select! {
            line = stdin.select_next_some() => swarm
                .behaviour_mut()
                .floodsub
                .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()),

            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                },
                SwarmEvent::IncomingConnection { local_addr, send_back_addr } => {
                    info!("{:?} Receiving connection from {:?}", local_addr, send_back_addr);
                },
                SwarmEvent::IncomingConnectionError { send_back_addr, .. } => {
                    info!("Connection failed from {:?}", send_back_addr);
                },
                SwarmEvent::ConnectionEstablished { endpoint, .. } => {
                    info!("Established connection from {:?}", endpoint.get_remote_address());
                },
                SwarmEvent::ConnectionClosed { endpoint, .. } => {
                    info!("Connection closed from {:?}", endpoint.get_remote_address());
                },
                SwarmEvent::Behaviour(OutEvent::Floodsub(floodsub_event)) => handle_floodsub_event(floodsub_event),
                SwarmEvent::Behaviour(OutEvent::Mdns(mdns_event)) => handle_mdns_event(&mut swarm, mdns_event),
                _ => warn!("Unhandled event")
            }
        }
    }
}

pub fn handle_floodsub_event(event: FloodsubEvent) {
    match event {
        FloodsubEvent::Message(message) => {
            info!(
                "{:?}: {:?}",
                message.source,
                String::from_utf8_lossy(&message.data)
            )
        }
        FloodsubEvent::Subscribed { peer_id, topic } => {
            info!("Client {} subscribed to topic {}", peer_id, topic.id());
        }
        FloodsubEvent::Unsubscribed { peer_id, topic } => {
            info!("Client {} unsubscribed from topic {}", peer_id, topic.id());
        }
    }
}

pub fn handle_mdns_event(swarm: &mut Swarm<AppBehaviour>, event: MdnsEvent) {
    match event {
        MdnsEvent::Expired(expired_list) => {
            for (peer_id, _) in expired_list {
                if !swarm.behaviour_mut().mdns.has_node(&peer_id) {
                    swarm
                        .behaviour_mut()
                        .floodsub
                        .remove_node_from_partial_view(&peer_id);
                }
            }
        }
        MdnsEvent::Discovered(discovered_list) => {
            for (peer_id, _) in discovered_list {
                swarm
                    .behaviour_mut()
                    .floodsub
                    .add_node_to_partial_view(peer_id);
            }
        }
    }
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }

    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_accurately() {
        assert_eq!(2, divide(20, 10));
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn panic_when_dividingbyzero() {
        assert_eq!(0, divide(10, 0))
    }
}
