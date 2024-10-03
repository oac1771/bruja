use clap::Parser;
use futures::prelude::*;
use libp2p::swarm::SwarmEvent;
use libp2p::{ping, Multiaddr};
use std::time::Duration;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
pub struct P2P {
    #[arg(long)]
    peer: Option<String>,
}

impl P2P {
    #[instrument]
    pub async fn handle(&self) {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_async_std()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::tls::Config::new,
                libp2p::yamux::Config::default,
            )
            .unwrap()
            .with_behaviour(|_| ping::Behaviour::default())
            .unwrap()
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

        // Tell the swarm to listen on all interfaces and a random, OS-assigned
        // port.
        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();

        // Dial the peer identified by the multi-address given as the second
        // command-line argument, if any.
        if let Some(addr) = &self.peer {
            let remote: Multiaddr = addr.parse().unwrap();
            swarm.dial(remote).unwrap();
            info!("Dialed {addr}")
        }

        loop {
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
                SwarmEvent::Behaviour(event) => info!("{event:?}"),
                _ => {}
            }
        }

        // Ok(())
    }
}
