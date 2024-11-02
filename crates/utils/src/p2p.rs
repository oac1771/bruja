use codec::{Decode, Encode};
use libp2p::{
    futures::prelude::*,
    gossipsub::{self, MessageId},
    mdns,
    request_response::{
        self, InboundRequestId, Message as RequestResponseMessage, OutboundRequestId,
        ProtocolSupport, ResponseChannel,
    },
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
    sync::{
        mpsc::{self, Receiver, Sender},
        oneshot,
    },
    task::{spawn, JoinHandle},
    time::{sleep, Duration as TokioDuration},
};
use tracing::{error, info, info_span, Instrument};

pub struct NodeBuilder;

pub struct Node {
    swarm: Swarm<Behavior>,
    gossip_messages: HashMap<MessageId, GossipMessage>,
}

pub struct NodeClient {
    req_tx: Sender<ClientRequest>,
    inbound_req_rx: Receiver<InboundP2pRequest>,
    pending_inbound_req: HashMap<InboundRequestId, ResponseChannel<P2pResponse>>,
}

impl NodeBuilder {
    pub fn build() -> Result<Node, Error> {
        let swarm = libp2p::SwarmBuilder::with_new_identity()
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
        let (inbound_req_tx, inbound_req_rx) = mpsc::channel::<InboundP2pRequest>(100);

        let addr = "/ip4/0.0.0.0/tcp/0".parse()?;
        self.swarm.listen_on(addr)?;

        let handle = spawn(
            async move {
                match self.run(req_rx, &inbound_req_tx).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            .instrument(info_span!("")),
        );

        let node_client = NodeClient::new(req_tx, inbound_req_rx);

        Ok((handle, node_client))
    }

    async fn run(
        &mut self,
        mut req_rx: Receiver<ClientRequest>,
        inbound_req_tx: &Sender<InboundP2pRequest>,
    ) -> Result<(), Error> {
        loop {
            select! {
                Some(request) = req_rx.recv() => self.handle_client_request(request),
                event = self.swarm.select_next_some() => self.handle_event(event, inbound_req_tx).await
            }
        }
    }

    fn handle_client_request(&mut self, request: ClientRequest) {
        match request {
            ClientRequest::Publish { topic, msg, sender } => {
                let tpc = gossipsub::IdentTopic::new(&topic);
                let result = if let Err(err) = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(tpc, msg.encode())
                {
                    error!("Publishing Error: {}", err);
                    Err(Error::from(err))
                } else {
                    info!("Successfully published message to {} topic", topic);
                    Ok(ClientResponse::Publish)
                };
                Self::send_client_response(result, sender);
            }
            ClientRequest::Subscribe { topic, sender } => {
                let topic = gossipsub::IdentTopic::new(topic);

                let result =
                    if let Err(err) = self.swarm.behaviour_mut().gossipsub.subscribe(&topic) {
                        error!("Subscription Error: {}", err);
                        Err(Error::from(err))
                    } else {
                        info!("Subscribed to topic: {}", topic);
                        Ok(ClientResponse::Subscribe)
                    };
                Self::send_client_response(result, sender);
            }
            ClientRequest::ReadGossipMessages { sender } => {
                let msgs = self
                    .gossip_messages
                    .values()
                    .map(|data| data.clone())
                    .collect::<Vec<GossipMessage>>();
                let resp = ClientResponse::GossipMessages { msgs };
                Self::send_client_response(Ok(resp), sender);
            }
            ClientRequest::SendRequest {
                payload,
                peer_id,
                sender,
            } => {
                let request_id = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&peer_id, P2pRequest(payload));
                let resp = ClientResponse::RequestId { request_id };
                Self::send_client_response(Ok(resp), sender);
            }
            ClientRequest::SendResponse {
                payload,
                sender,
                channel,
            } => {
                let result = if let Ok(_) = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_response(channel, P2pResponse(payload))
                {
                    info!("Response successfully sent");
                    Ok(ClientResponse::ResponseSent)
                } else {
                    error!("Subscribed to topic");
                    Err(Error::SendResponseError)
                };
                Self::send_client_response(result, sender);
            }
            ClientRequest::GetLocalPeerId { sender } => {
                let peer_id = self.swarm.local_peer_id();
                let resp = ClientResponse::PeerId {
                    peer_id: peer_id.clone(),
                };
                Self::send_client_response(Ok(resp), sender);
            }
        };
    }

    fn send_client_response(
        result: Result<ClientResponse, Error>,
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
    ) {
        if let Err(_) = sender.send(result) {
            error!("Error sending response to client. The receiver has been dropped");
        }
    }

    async fn handle_event(
        &mut self,
        event: SwarmEvent<BehaviorEvent>,
        inbound_req_tx: &Sender<InboundP2pRequest>,
    ) {
        match event {
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            })) => {
                info!("Received gossip message: with id: {id} from peer: {peer_id}");
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
            SwarmEvent::Behaviour(BehaviorEvent::RequestResponse(
                request_response::Event::Message { peer, message },
            )) => {
                info!("Received request response message from peer: {}", peer);
                match message {
                    RequestResponseMessage::Request {
                        request,
                        request_id,
                        channel,
                    } => {
                        let req = InboundP2pRequest {
                            request,
                            channel,
                            request_id,
                        };
                        inbound_req_tx.send(req).await.unwrap();
                        info!("Request relayed to client");
                    }
                    _ => {} // RequestResponseMessage::Response { request_id, response } => todo!(),
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
    fn new(req_tx: Sender<ClientRequest>, inbound_req_rx: Receiver<InboundP2pRequest>) -> Self {
        let pending_inbound_req: HashMap<InboundRequestId, ResponseChannel<P2pResponse>> =
            HashMap::new();

        Self {
            req_tx,
            inbound_req_rx,
            pending_inbound_req,
        }
    }

    pub async fn publish(&self, topic: &str, msg: Message) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();
        let req = ClientRequest::Publish {
            topic: topic.to_string(),
            msg,
            sender,
        };
        self.send_client_request(req).await?;
        self.recv_node_response(receiver).await?;

        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), Error> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();
        let req = ClientRequest::Subscribe {
            topic: topic.to_string(),
            sender,
        };
        self.send_client_request(req).await?;
        self.recv_node_response(receiver).await?;

        Ok(())
    }

    pub async fn get_gossip_messages(&mut self) -> Result<Vec<GossipMessage>, Error> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();

        let req = ClientRequest::ReadGossipMessages { sender };
        self.send_client_request(req).await?;

        if let ClientResponse::GossipMessages { msgs } = self.recv_node_response(receiver).await? {
            return Ok(msgs);
        }
        return Err(Error::UnexpectedClientResponse);
    }

    pub async fn send_request<P: Encode>(
        &mut self,
        peer_id: PeerId,
        payload: P,
    ) -> Result<OutboundRequestId, Error> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();

        let req = ClientRequest::SendRequest {
            peer_id,
            payload: payload.encode(),
            sender,
        };
        self.send_client_request(req).await?;

        if let ClientResponse::RequestId { request_id } = self.recv_node_response(receiver).await? {
            return Ok(request_id);
        }
        return Err(Error::UnexpectedClientResponse);
    }

    pub async fn send_response<P: Encode>(
        &mut self,
        id: InboundRequestId,
        payload: P,
    ) -> Result<(), Error> {
        let channel = self.pending_inbound_req.remove(&id).unwrap();
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();
        let req = ClientRequest::SendResponse {
            sender,
            payload: payload.encode(),
            channel,
        };

        self.send_client_request(req).await?;
        self.recv_node_response(receiver).await?;

        Ok(())
    }

    pub async fn get_local_peer_id(&mut self) -> Result<PeerId, Error> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();

        let req = ClientRequest::GetLocalPeerId { sender };
        self.send_client_request(req).await?;

        if let ClientResponse::PeerId { peer_id } = self.recv_node_response(receiver).await? {
            return Ok(peer_id);
        }
        return Err(Error::UnexpectedClientResponse);
    }

    pub async fn read_inbound_requests(&mut self) -> Result<(P2pRequest, InboundRequestId), Error> {
        while let Some(req) = self.inbound_req_rx.recv().await {
            self.pending_inbound_req.insert(req.request_id, req.channel);
            return Ok((req.request, req.request_id));
        }
        return Err(Error::Other {
            err: "foo".to_string(),
        });
    }

    async fn send_client_request(&self, req: ClientRequest) -> Result<(), Error> {
        self.req_tx
            .send(req)
            .await
            .map_err(|err| Error::SendRequestError {
                err: err.to_string(),
            })?;

        Ok(())
    }

    async fn recv_node_response(
        &self,
        receiver: oneshot::Receiver<Result<ClientResponse, Error>>,
    ) -> Result<ClientResponse, Error> {
        let result = select! {
            _ = sleep(TokioDuration::from_secs(10)) => {
                Err(Error::OneShotTimeOutError)
            },
            msgs = receiver => {
                Ok(msgs?)
            }
        }??;

        Ok(result)
    }
}

pub enum ClientRequest {
    Publish {
        topic: String,
        msg: Message,
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
    },
    Subscribe {
        topic: String,
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
    },
    ReadGossipMessages {
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
    },
    SendRequest {
        peer_id: PeerId,
        payload: Vec<u8>,
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
    },
    SendResponse {
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
        payload: Vec<u8>,
        channel: ResponseChannel<P2pResponse>,
    },
    GetLocalPeerId {
        sender: oneshot::Sender<Result<ClientResponse, Error>>,
    },
}

pub enum ClientResponse {
    Publish,
    Subscribe,
    ResponseSent,
    GossipMessages { msgs: Vec<GossipMessage> },
    RequestId { request_id: OutboundRequestId },
    PeerId { peer_id: PeerId },
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

#[derive(Encode, Decode, Debug, Clone)]
pub enum Message {
    JobAcceptance { job_id: Vec<u8> },
}

pub struct InboundP2pRequest {
    request: P2pRequest,
    channel: ResponseChannel<P2pResponse>,
    request_id: InboundRequestId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct P2pRequest(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct P2pResponse(pub Vec<u8>);

#[derive(NetworkBehaviour)]
struct Behavior {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    request_response: request_response::cbor::Behaviour<P2pRequest, P2pResponse>,
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

    #[error("{source}")]
    PublishError {
        #[from]
        source: libp2p::gossipsub::PublishError,
    },

    #[error("{source}")]
    TokioRecvError {
        #[from]
        source: tokio::sync::oneshot::error::RecvError,
    },

    #[error("Timed out waiting for response from node")]
    OneShotTimeOutError,

    #[error("")]
    UnexpectedClientResponse,

    #[error("{err}")]
    SendRequestError { err: String },

    #[error("")]
    SendResponseError,

    #[error("{err}")]
    Other { err: String },
}
