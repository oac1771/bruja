use libp2p::{
    futures::prelude::*,
    gossipsub, mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    Swarm,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{
    io, select,
    sync::mpsc::{self, Receiver, Sender},
    task::{spawn, JoinHandle},
};
use tracing::{error, info};

pub struct NodeBuilder;

pub struct Node {
    swarm: Swarm<Behavior>,
}

pub struct NodeClient {
    cmd_tx: Sender<Command>,
}

impl NodeBuilder {
    pub fn build() -> Result<Node, Error> {
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

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                    .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                    .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

                // build a gossipsub network behaviour
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(Behavior { gossipsub, mdns })
            })
            .map_err(|err| Error::Other {
                err: err.to_string(),
            })?
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

        let addr = "/ip4/0.0.0.0/tcp/0".parse()?;
        swarm.listen_on(addr)?;

        Ok(Node { swarm })
    }
}

impl Node {
    pub async fn run(mut self) -> Result<(JoinHandle<Result<(), Error>>, NodeClient), Error> {
        let (cmd_tx, cmd_rx) = mpsc::channel::<Command>(100);

        let handle = spawn(async move {
            match self.start(cmd_rx).await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        });

        let node_client = NodeClient { cmd_tx };

        Ok((handle, node_client))
    }

    async fn start(&mut self, mut cmd_rx: Receiver<Command>) -> Result<(), Error> {
        loop {
            select! {
                Some(cmd) = cmd_rx.recv() => self.handle_cmd(cmd).await,
                event = self.swarm.select_next_some() => self.handle_event(event).await
            }
        }
    }

    async fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Publish { topic, val } => {
                let topic = gossipsub::IdentTopic::new(topic);
                match self.swarm.behaviour_mut().gossipsub.publish(topic, val) {
                    Ok(msg_id) => {
                        info!("Message sent with ID: {}", msg_id);
                    }
                    Err(err) => {
                        error!("Publish Error: {}", err);
                    }
                }
            }
            Command::Subscribe { topic } => {
                let topic = gossipsub::IdentTopic::new(topic);
                match self.swarm.behaviour_mut().gossipsub.subscribe(&topic) {
                    Ok(_) => {
                        info!("Subscribed to topic: {}", topic);
                    }
                    Err(err) => {
                        error!("Subscription Error: {}", err);
                    }
                }
            }
        };
    }

    async fn handle_event(&mut self, event: SwarmEvent<BehaviorEvent>) {
        match event {
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            })) => info!(
                "Got message: '{}' with id: {id} from peer: {peer_id}",
                String::from_utf8_lossy(&message.data),
            ),
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Subscribed {
                peer_id: _peer_id,
                topic,
            })) => info!("A remote subscribed to a topic: {topic}",),
            SwarmEvent::Behaviour(BehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _multiaddr) in list {
                    info!("mDNS discovered a new peer: {peer_id}");
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::Behaviour(BehaviorEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, _multiaddr) in list {
                    info!("mDNS discover peer has expired: {peer_id}");
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                }
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Local node is listening on {address}");
            }
            _ => {}
        }
    }
}

impl NodeClient {
    pub async fn publish(&self, topic: &str, val: Vec<u8>) -> Result<(), Error> {
        let cmd = Command::Publish {
            topic: topic.to_string(),
            val,
        };
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|err| Error::SendError {
                err: err.to_string(),
            })?;

        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), Error> {
        let cmd = Command::Subscribe {
            topic: topic.to_string(),
        };
        self.cmd_tx
            .send(cmd)
            .await
            .map_err(|err| Error::SendError {
                err: err.to_string(),
            })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Command {
    Publish { topic: String, val: Vec<u8> },
    Subscribe { topic: String },
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

    #[error("Channel Send Error: {err}")]
    SendError { err: String },

    #[error("Error: {err}")]
    Other { err: String },
}
