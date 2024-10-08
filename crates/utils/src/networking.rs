use libp2p::{
    futures::prelude::*,
    gossipsub, mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
};
use std::{time::Duration, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};
use tracing::info;
use tokio::io;

pub async fn start() -> Result<(), Error> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|key| {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(Behavior { gossipsub, mdns })
        }).map_err(|err| Error::Other { err: err.to_string() })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    let topic = gossipsub::IdentTopic::new("test-net");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    let addr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(addr)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => info!("{event:?}"),
            _ => {}
        }
    }

}

#[derive(NetworkBehaviour)]
struct Behavior {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
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

    #[error("{source}")]
    SubscriptionError {
        #[from]
        source: libp2p::gossipsub::SubscriptionError,
    },

    #[error("Error: {err}")]
    Other {err: String}
}
