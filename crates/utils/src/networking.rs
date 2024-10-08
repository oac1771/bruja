use libp2p::{
    futures::prelude::*, mdns::{tokio::Tokio, Behaviour, Config}, swarm::SwarmEvent, PeerId
};
use std::time::Duration;
use tracing::info;

#[allow(unreachable_code)]
pub async fn start() -> Result<(), Error> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key_pair| {
            let peer_id: PeerId = PeerId::from_public_key(&key_pair.public());
            let config = Config::default();
            let behavior: Behaviour<Tokio> = Behaviour::new(config, peer_id).unwrap();
            behavior
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    let addr = "/ip4/0.0.0.0/tcp/0".parse()?;

    swarm.listen_on(addr)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => info!("{event:?}"),
            _ => {}
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    RcgenError {
        #[from]
        source: libp2p::tls::certificate::GenError,
    },

    #[error("{source}")]
    Infallible {
        #[from]
        source: std::convert::Infallible,
    },

    #[error("{source}")]
    TransportError {
        #[from]
        source: libp2p::TransportError<std::io::Error>,
    },

    #[error("{source}")]
    MultiAddrError {
        #[from]
        source: libp2p::multiaddr::Error,
    },
}
