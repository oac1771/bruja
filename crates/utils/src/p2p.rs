use codec::{Decode, Encode};
use libp2p::{
    futures::prelude::*,
    gossipsub::{self, MessageId},
    mdns,
    request_response::{self, OutboundRequestId, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId, StreamProtocol, Swarm,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{
    io, select,
    sync::mpsc::{self, Receiver, Sender},
    task::{spawn, JoinHandle},
    time::{sleep, Duration as TokioDuration, Instant},
};
use tracing::{error, info, info_span, Instrument};

pub struct NodeBuilder;

pub struct Node {
    swarm: Swarm<Behavior>,
    gossip_messages: HashMap<MessageId, GossipMessage>,
}

pub struct NodeClient {
    req_tx: Sender<ClientRequest>,
    resp_rx: Receiver<ClientResponse>,
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

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                let request_response = request_response::cbor::Behaviour::new(
                    [(
                        StreamProtocol::new("/file-exchange/1"),
                        ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                );

                Ok(Behavior {
                    gossipsub,
                    mdns,
                    request_response,
                })
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

        let gossip_messages: HashMap<MessageId, GossipMessage> = HashMap::new();

        Ok(Node {
            swarm,
            gossip_messages,
        })
    }
}

impl Node {
    pub fn start(mut self) -> Result<(JoinHandle<Result<(), Error>>, NodeClient), Error> {
        let (req_tx, req_rx) = mpsc::channel::<ClientRequest>(100);
        let (resp_tx, resp_rx) = mpsc::channel::<ClientResponse>(100);

        let handle = spawn(
            async move {
                match self.run(req_rx, resp_tx).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            .instrument(info_span!("")),
        );

        let node_client = NodeClient { req_tx, resp_rx };

        Ok((handle, node_client))
    }

    async fn run(
        &mut self,
        mut req_rx: Receiver<ClientRequest>,
        resp_tx: Sender<ClientResponse>,
    ) -> Result<(), Error> {
        loop {
            select! {
                Some(request) = req_rx.recv() => self.handle_client_request(request, &resp_tx).await,
                event = self.swarm.select_next_some() => self.handle_event(event)
            }
        }
    }

    async fn handle_client_request(
        &mut self,
        request: ClientRequest,
        resp_tx: &Sender<ClientResponse>,
    ) {
        match request {
            ClientRequest::Publish { topic, msg } => {
                let topic = gossipsub::IdentTopic::new(topic);
                match self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(topic, msg.encode())
                {
                    Ok(msg_id) => {
                        info!("Message sent with ID: {}", msg_id);
                    }
                    Err(err) => {
                        error!("Publish Error: {}", err);
                    }
                }
            }
            ClientRequest::Subscribe { topic } => {
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
            ClientRequest::ReadGossipMessages => {
                let msgs = self
                    .gossip_messages
                    .values()
                    .map(|data| data.clone())
                    .collect::<Vec<GossipMessage>>();
                if let Err(err) = resp_tx.send(ClientResponse::Messages { msgs }).await {
                    error!("Error Sending Messages: {}", err);
                }
            }
            ClientRequest::SendRequest { payload, peer_id } => {
                let request_id = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&peer_id, Request(payload.encode()));
                if let Err(err) = resp_tx.send(ClientResponse::RequestId { request_id }).await {
                    error!("Error sending request_id: {}", err);
                }
            }
        };
    }

    fn handle_event(&mut self, event: SwarmEvent<BehaviorEvent>) {
        match event {
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            })) => {
                info!("Received message: with id: {id} from peer: {peer_id}");
                match <Message as Decode>::decode(&mut message.data.as_slice()) {
                    Ok(msg) => {
                        let gsp_msg = GossipMessage {
                            peer_id,
                            message: msg,
                        };
                        self.gossip_messages.insert(id, gsp_msg);
                    }
                    Err(err) => {
                        error!("Error Decoding Message: {}", err);
                    }
                }
            }
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Subscribed {
                peer_id: _peer_id,
                topic,
            })) => info!("A remote subscribed to a topic: {topic}",),
            SwarmEvent::Behaviour(BehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _) in list {
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
    pub async fn publish(&self, topic: &str, msg: Message) -> Result<(), Error> {
        let req = ClientRequest::Publish {
            topic: topic.to_string(),
            msg,
        };

        self.send(req).await?;

        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), Error> {
        let req = ClientRequest::Subscribe {
            topic: topic.to_string(),
        };
        self.send(req).await?;

        Ok(())
    }

    pub async fn read_gossip_messages(&mut self) -> Result<Vec<GossipMessage>, Error> {
        let req = ClientRequest::ReadGossipMessages;
        self.send(req).await?;

        while let Some(ClientResponse::Messages { msgs }) = self.resp_rx.recv().await {
            return Ok(msgs);
        }

        Err(Error::Other {
            err: "Error receiving response".to_string(),
        })
    }

    pub async fn wait_for_gossip_messages(&mut self) -> Result<Vec<GossipMessage>, Error> {
        let start = Instant::now();

        while start.elapsed() < TokioDuration::from_secs(10) {
            match self.read_gossip_messages().await {
                Ok(msgs) => {
                    if msgs.len() > 0 {
                        return Ok(msgs);
                    }
                }
                Err(err) => {
                    error!("Error reading messages: {}", err);
                }
            }
            sleep(TokioDuration::from_secs(1)).await;
        }

        Err(Error::Other {
            err: "Timed out waiting for messages".to_string(),
        })
    }

    pub async fn send_request(
        &mut self,
        peer_id: PeerId,
        payload: Payload,
    ) -> Result<OutboundRequestId, Error> {
        let req = ClientRequest::SendRequest { peer_id, payload };
        self.send(req).await?;

        while let Some(ClientResponse::RequestId { request_id }) = self.resp_rx.recv().await {
            return Ok(request_id);
        }

        Err(Error::Other {
            err: "Error receiving response".to_string(),
        })
    }

    async fn send(&self, req: ClientRequest) -> Result<(), Error> {
        self.req_tx
            .send(req)
            .await
            .map_err(|err| Error::SendError {
                err: err.to_string(),
            })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ClientRequest {
    Publish { topic: String, msg: Message },
    Subscribe { topic: String },
    ReadGossipMessages,
    SendRequest { peer_id: PeerId, payload: Payload },
}

#[derive(Debug, Encode)]
pub enum Payload {
    Job,
}

pub enum ClientResponse {
    Messages { msgs: Vec<GossipMessage> },
    RequestId { request_id: OutboundRequestId },
}

#[derive(Encode, Decode, Debug, Clone)]
pub enum Message {
    JobAcceptance { job_id: Vec<u8> },
}

#[derive(Clone)]
pub struct GossipMessage {
    peer_id: PeerId,
    message: Message,
}

impl GossipMessage {
    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub fn message(self) -> Message {
        self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Request(Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Response(Vec<u8>);

#[derive(NetworkBehaviour)]
struct Behavior {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    request_response: request_response::cbor::Behaviour<Request, Response>,
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
