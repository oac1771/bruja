use libp2p::{
    futures::prelude::*,
    gossipsub, mdns,
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
    future::Future,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{
    io, select,
    sync::{
        mpsc::{self, Receiver, Sender},
        oneshot,
    },
    task::{spawn, yield_now, JoinHandle},
    time::{sleep, Duration as TokioDuration},
};
use tracing::{error, info, info_span, Instrument};

pub struct NodeBuilder;

pub struct Node {
    swarm: Swarm<Behavior>,
}

pub struct NodeClient {
    req_tx: Sender<ClientRequest>,
    inbound_req_rx: Receiver<InboundP2pRequest>,
    inbound_resp_rx: Receiver<InboundP2pResponse>,
    gossip_msg_rx: Receiver<GossipMessage>,
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
            .with_quic()
            .with_behaviour(|key| {
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns_config = mdns::Config {
                    ttl: Duration::from_secs(6 * 60),
                    query_interval: Duration::from_secs(1),
                    enable_ipv6: false,
                };

                let mdns = mdns::tokio::Behaviour::new(mdns_config, key.public().to_peer_id())?;
                let request_response = request_response::cbor::Behaviour::new(
                    [(StreamProtocol::new("/exchange/1"), ProtocolSupport::Full)],
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

        Ok(Node { swarm })
    }
}

impl Node {
    pub fn start(mut self) -> Result<(JoinHandle<Result<(), Error>>, NodeClient), Error> {
        let (req_tx, req_rx) = mpsc::channel::<ClientRequest>(100);
        let (inbound_req_tx, inbound_req_rx) = mpsc::channel::<InboundP2pRequest>(100);
        let (inbound_resp_tx, inbound_resp_rx) = mpsc::channel::<InboundP2pResponse>(100);
        let (gossip_msg_tx, gossip_msg_rx) = mpsc::channel::<GossipMessage>(100);

        self.swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        let handle = spawn(
            async move {
                match self
                    .run(req_rx, &inbound_req_tx, &inbound_resp_tx, &gossip_msg_tx)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            .instrument(info_span!("")),
        );

        let node_client = NodeClient::new(req_tx, inbound_req_rx, inbound_resp_rx, gossip_msg_rx);

        Ok((handle, node_client))
    }

    async fn run(
        &mut self,
        mut req_rx: Receiver<ClientRequest>,
        inbound_req_tx: &Sender<InboundP2pRequest>,
        inbound_resp_tx: &Sender<InboundP2pResponse>,
        gossip_msg_tx: &Sender<GossipMessage>,
    ) -> Result<(), Error> {
        loop {
            select! {
                Some(request) = req_rx.recv() => self.handle_client_request(request),
                event = self.swarm.select_next_some() => self.handle_event(event, inbound_req_tx, inbound_resp_tx, gossip_msg_tx).await
            }
        }
    }

    fn handle_client_request(&mut self, request: ClientRequest) {
        let sender = request.sender;
        match request.payload {
            ClientRequestPayload::Publish { topic, msg } => {
                let tpc = gossipsub::IdentTopic::new(&topic);
                let result =
                    if let Err(err) = self.swarm.behaviour_mut().gossipsub.publish(tpc, msg) {
                        error!("Publishing Error: {}", err);
                        Err(Error::from(err))
                    } else {
                        info!("Successfully published message to {} topic", topic);
                        Ok(ClientResponse::Publish)
                    };
                Self::send_client_response(result, sender);
            }
            ClientRequestPayload::Subscribe { topic } => {
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
            ClientRequestPayload::SendRequest { payload, peer_id } => {
                let request_id = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&peer_id, P2pRequest(payload));
                let resp = ClientResponse::RequestId { request_id };
                Self::send_client_response(Ok(resp), sender);
            }
            ClientRequestPayload::SendResponse { payload, channel } => {
                let result = if let Ok(_) = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_response(channel, P2pResponse(payload))
                {
                    info!("Response successfully sent");
                    Ok(ClientResponse::ResponseSent)
                } else {
                    error!("Send Response Error");
                    Err(Error::SendResponseError)
                };
                Self::send_client_response(result, sender);
            }
            ClientRequestPayload::GetLocalPeerId => {
                let peer_id = self.swarm.local_peer_id();
                let resp = ClientResponse::PeerId {
                    peer_id: peer_id.clone(),
                };
                Self::send_client_response(Ok(resp), sender);
            }
            ClientRequestPayload::GetGossipNodes { topic } => {
                let hash = gossipsub::IdentTopic::new(topic).hash();
                let gossip_nodes = self
                    .swarm
                    .behaviour()
                    .gossipsub
                    .all_peers()
                    .filter(|(_, t)| t.contains(&&hash))
                    .map(|(p, _)| p.clone())
                    .collect::<Vec<PeerId>>();

                let resp = ClientResponse::GossipNodes { gossip_nodes };
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
        inbound_resp_tx: &Sender<InboundP2pResponse>,
        gossip_msg_tx: &Sender<GossipMessage>,
    ) {
        match event {
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message,
                ..
            })) => {
                let gsp_msg = GossipMessage {
                    peer_id,
                    message: message.data,
                };
                match gossip_msg_tx.send(gsp_msg).await {
                    Ok(_) => {
                        info!("Gossip message relayed to client");
                    }
                    Err(err) => error!("Error relaying gossip message to client: {}", err),
                }
            }
            SwarmEvent::Behaviour(BehaviorEvent::RequestResponse(
                request_response::Event::Message { peer, message },
            )) => match message {
                RequestResponseMessage::Request {
                    request,
                    request_id: id,
                    channel,
                } => {
                    info!("Received request from peer: {}", peer);
                    let req = InboundP2pRequest {
                        request,
                        channel,
                        id,
                    };
                    match inbound_req_tx.send(req).await {
                        Ok(_) => {
                            info!("Inbound request relayed to client");
                        }
                        Err(err) => error!("Error relaying inbound request to client: {}", err),
                    }
                }
                RequestResponseMessage::Response {
                    request_id: id,
                    response,
                } => {
                    info!("Received response from peer: {}", peer);
                    let resp = InboundP2pResponse { response, id };
                    match inbound_resp_tx.send(resp).await {
                        Ok(_) => {
                            info!("Inbound response relayed to client");
                        }
                        Err(err) => {
                            error!("Error relaying inbound response to client: {}", err)
                        }
                    }
                }
            },
            SwarmEvent::Behaviour(BehaviorEvent::Gossipsub(gossipsub::Event::Subscribed {
                peer_id: _peer_id,
                topic,
            })) => info!("A remote subscribed to a topic: {topic}"),
            SwarmEvent::Behaviour(BehaviorEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, _) in list {
                    self.swarm
                        .behaviour_mut()
                        .gossipsub
                        .add_explicit_peer(&peer_id);
                    info!("mDNS discovered a new peer: {peer_id}");
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
        yield_now().await;
    }
}

impl NodeClient {
    fn new(
        req_tx: Sender<ClientRequest>,
        inbound_req_rx: Receiver<InboundP2pRequest>,
        inbound_resp_rx: Receiver<InboundP2pResponse>,
        gossip_msg_rx: Receiver<GossipMessage>,
    ) -> Self {
        let pending_inbound_req: HashMap<InboundRequestId, ResponseChannel<P2pResponse>> =
            HashMap::new();

        Self {
            req_tx,
            inbound_req_rx,
            inbound_resp_rx,
            gossip_msg_rx,
            pending_inbound_req,
        }
    }

    pub async fn get_gossip_nodes(&self, topic: &str) -> Result<Vec<PeerId>, Error> {
        let payload = ClientRequestPayload::GetGossipNodes {
            topic: topic.to_string(),
        };
        if let ClientResponse::GossipNodes { gossip_nodes } =
            self.send_client_request(payload).await?
        {
            return Ok(gossip_nodes);
        }
        return Err(Error::UnexpectedClientResponse);
    }

    pub async fn publish(&self, topic: &str, msg: Vec<u8>) -> Result<(), Error> {
        let payload = ClientRequestPayload::Publish {
            topic: topic.to_string(),
            msg,
        };
        self.send_client_request(payload).await?;

        Ok(())
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), Error> {
        let payload = ClientRequestPayload::Subscribe {
            topic: topic.trim().to_string(),
        };
        self.send_client_request(payload).await?;

        Ok(())
    }

    pub async fn send_request(
        &mut self,
        peer_id: PeerId,
        payload: Vec<u8>,
    ) -> Result<OutboundRequestId, Error> {
        let payload = ClientRequestPayload::SendRequest { payload, peer_id };

        if let ClientResponse::RequestId { request_id } = self.send_client_request(payload).await? {
            return Ok(request_id);
        }
        return Err(Error::UnexpectedClientResponse);
    }

    pub async fn send_response(
        &mut self,
        id: InboundRequestId,
        payload: Vec<u8>,
    ) -> Result<(), Error> {
        if let Some(channel) = self.pending_inbound_req.remove(&id) {
            let payload = ClientRequestPayload::SendResponse { payload, channel };
            self.send_client_request(payload).await?;
            return Ok(());
        }
        return Err(Error::InboundRequestIdNotFound);
    }

    pub async fn get_local_peer_id(&mut self) -> Result<PeerId, Error> {
        let payload = ClientRequestPayload::GetLocalPeerId;

        if let ClientResponse::PeerId { peer_id } = self.send_client_request(payload).await? {
            return Ok(peer_id);
        }
        return Err(Error::UnexpectedClientResponse);
    }

    pub async fn recv_inbound_req(&mut self) -> Option<(InboundRequestId, P2pRequest)> {
        if let Some(req) = self.inbound_req_rx.recv().await {
            self.pending_inbound_req.insert(req.id, req.channel);
            return Some((req.id, req.request));
        }
        return None;
    }

    pub fn recv_inbound_resp(&mut self) -> impl Future<Output = Option<InboundP2pResponse>> + '_ {
        self.inbound_resp_rx.recv()
    }

    pub fn foo_recv(
        &mut self,
    ) -> Result<InboundP2pResponse, tokio::sync::mpsc::error::TryRecvError> {
        self.inbound_resp_rx.try_recv()
    }

    pub fn recv_gossip_msg(&mut self) -> impl Future<Output = Option<GossipMessage>> + '_ {
        self.gossip_msg_rx.recv()
    }

    async fn send_client_request(
        &self,
        payload: ClientRequestPayload,
    ) -> Result<ClientResponse, Error> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, Error>>();
        let req = ClientRequest { payload, sender };

        self.req_tx
            .send(req)
            .await
            .map_err(|err| Error::SendRequestError {
                err: err.to_string(),
            })?;

        let resp = self.recv_node_response(receiver).await?;

        Ok(resp)
    }

    async fn recv_node_response(
        &self,
        receiver: oneshot::Receiver<Result<ClientResponse, Error>>,
    ) -> Result<ClientResponse, Error> {
        let result = select! {
            _ = sleep(TokioDuration::from_secs(10)) => {
                Err(Error::TimeOutError {err: "Timedout waiting for response from node".to_string()})
            },
            msgs = receiver => {
                Ok(msgs?)
            }
        }??;

        Ok(result)
    }
}
struct ClientRequest {
    payload: ClientRequestPayload,
    sender: oneshot::Sender<Result<ClientResponse, Error>>,
}
pub enum ClientRequestPayload {
    Publish {
        topic: String,
        msg: Vec<u8>,
    },
    Subscribe {
        topic: String,
    },
    SendRequest {
        peer_id: PeerId,
        payload: Vec<u8>,
    },
    SendResponse {
        channel: ResponseChannel<P2pResponse>,
        payload: Vec<u8>,
    },
    GetLocalPeerId,
    GetGossipNodes {
        topic: String,
    },
}

pub enum ClientResponse {
    Publish,
    Subscribe,
    ResponseSent,
    GossipMessages { msgs: Vec<GossipMessage> },
    RequestId { request_id: OutboundRequestId },
    PeerId { peer_id: PeerId },
    GossipNodes { gossip_nodes: Vec<PeerId> },
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GossipMessage {
    peer_id: PeerId,
    message: Vec<u8>,
}

impl GossipMessage {
    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub fn message(self) -> Vec<u8> {
        self.message
    }
}

pub struct InboundP2pRequest {
    request: P2pRequest,
    channel: ResponseChannel<P2pResponse>,
    id: InboundRequestId,
}

#[derive(Debug)]
pub struct InboundP2pResponse {
    response: P2pResponse,
    id: OutboundRequestId,
}

impl InboundP2pResponse {
    pub fn response(&self) -> &P2pResponse {
        &self.response
    }

    pub fn id(&self) -> &OutboundRequestId {
        &self.id
    }
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
    RecvError {
        #[from]
        source: tokio::sync::oneshot::error::RecvError,
    },

    #[error("{err}")]
    TimeOutError { err: String },

    #[error("")]
    UnexpectedClientResponse,

    #[error("")]
    SendResponseError,

    #[error("")]
    InboundRequestIdNotFound,

    #[error("{err}")]
    SendRequestError { err: String },

    #[error("{err}")]
    Other { err: String },
}
