use async_stream::stream;
use libp2p::{
    futures::prelude::*,
    gossipsub, mdns,
    request_response::{
        self, InboundRequestId, Message as RequestResponseMessage, OutboundRequestId,
        ProtocolSupport, ResponseChannel,
    },
    swarm::{DialError, NetworkBehaviour, SwarmEvent},
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
        oneshot, Mutex,
    },
    task::{spawn, yield_now, JoinHandle},
    time::{sleep, Duration as TokioDuration},
};
use tracing::{error, info, info_span, Instrument};

pub trait NetworkClient {
    type Err;
    // make all return types associated types

    fn publish_message(
        &self,
        topic: &str,
        msg: Vec<u8>,
    ) -> impl Future<Output = Result<(), Self::Err>>;

    fn send_request(
        &self,
        peer_id: PeerId,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<OutboundRequestId, Self::Err>> + Send;

    fn send_response(
        &self,
        id: InboundRequestId,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send;

    fn get_gossip_nodes(
        &self,
        topic: &str,
    ) -> impl Future<Output = Result<Vec<PeerId>, Self::Err>> + Send;

    fn get_local_peer_id(&self) -> impl Future<Output = Result<PeerId, Self::Err>> + Send;

    fn gossip_msg_stream(&self) -> impl Future<Output = impl Stream<Item = GossipMessage>> + Send;

    fn req_stream(
        &self,
    ) -> impl Future<Output = impl Stream<Item = (InboundRequestId, P2pRequest)>> + Send;

    fn resp_stream(&self) -> impl Future<Output = impl Stream<Item = InboundP2pResponse>> + Send;
}

pub struct NodeBuilder;

pub struct Node {
    swarm: Swarm<Behavior>,
}

pub struct NodeClient {
    req_tx: Sender<ClientRequest>,
    inbound_req_rx: Mutex<Receiver<InboundP2pRequest>>,
    inbound_resp_rx: Mutex<Receiver<InboundP2pResponse>>,
    gossip_msg_rx: Mutex<Receiver<GossipMessage>>,
    pending_inbound_req: Mutex<HashMap<InboundRequestId, ResponseChannel<P2pResponse>>>,
}

impl NodeBuilder {
    pub fn build() -> Result<Node, NetworkClientError> {
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

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
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
    pub fn start(
        mut self,
    ) -> Result<(JoinHandle<Result<(), NetworkClientError>>, NodeClient), NetworkClientError> {
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
                    Err(err) => Err(NetworkClientError::from(err)),
                }
            }
            .instrument(info_span!("")),
        );

        let node_client = NodeClient::new(
            req_tx,
            Mutex::new(inbound_req_rx),
            Mutex::new(inbound_resp_rx),
            Mutex::new(gossip_msg_rx),
        );

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
                let result = if self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_response(channel, P2pResponse(payload))
                    .is_ok()
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
                let resp = ClientResponse::PeerId { peer_id: *peer_id };
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
                    .map(|(p, _)| *p)
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
        if sender.send(result).is_err() {
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
            SwarmEvent::Behaviour(BehaviorEvent::RequestResponse(
                request_response::Event::OutboundFailure { peer, error, .. },
            )) => {
                error!("Outbound request to peer {} failed: {}", peer, error);
            }
            SwarmEvent::Behaviour(BehaviorEvent::RequestResponse(
                request_response::Event::InboundFailure { peer, error, .. },
            )) => {
                error!("Inbound request from peer {} failed: {}", peer, error);
            }
            _ => {}
        }
        yield_now().await;
    }
}

impl NetworkClient for NodeClient {
    type Err = NetworkClientError;

    async fn publish_message(&self, topic: &str, msg: Vec<u8>) -> Result<(), Self::Err> {
        let payload = ClientRequestPayload::Publish {
            topic: topic.to_string(),
            msg,
        };
        self.send_client_request(payload).await?;

        Ok(())
    }

    async fn get_local_peer_id(&self) -> Result<PeerId, Self::Err> {
        let payload = ClientRequestPayload::GetLocalPeerId;

        if let ClientResponse::PeerId { peer_id } = self.send_client_request(payload).await? {
            return Ok(peer_id);
        }
        Err(Error::UnexpectedClientResponse.into())
    }

    async fn send_request(
        &self,
        peer_id: PeerId,
        payload: Vec<u8>,
    ) -> Result<OutboundRequestId, Self::Err> {
        let payload = ClientRequestPayload::SendRequest { payload, peer_id };

        if let ClientResponse::RequestId { request_id } = self.send_client_request(payload).await? {
            return Ok(request_id);
        }
        Err(Error::UnexpectedClientResponse.into())
    }

    async fn send_response(&self, id: InboundRequestId, payload: Vec<u8>) -> Result<(), Self::Err> {
        if let Some(channel) = self.pending_inbound_req.lock().await.remove(&id) {
            let payload = ClientRequestPayload::SendResponse { payload, channel };
            self.send_client_request(payload).await?;
            return Ok(());
        }
        Err(Error::InboundRequestIdNotFound.into())
    }

    async fn get_gossip_nodes(&self, topic: &str) -> Result<Vec<PeerId>, Self::Err> {
        let payload = ClientRequestPayload::GetGossipNodes {
            topic: topic.to_string(),
        };
        if let ClientResponse::GossipNodes { gossip_nodes } =
            self.send_client_request(payload).await?
        {
            return Ok(gossip_nodes);
        }
        Err(Error::UnexpectedClientResponse.into())
    }

    async fn req_stream(&self) -> impl Stream<Item = (InboundRequestId, P2pRequest)> {
        let stream = stream! {
            while let Some(req) = self.inbound_req_rx.lock().await.recv().await {
                self.pending_inbound_req.lock().await.insert(req.id, req.channel);
                yield (req.id, req.request)
            }
        };

        stream
    }

    async fn resp_stream(&self) -> impl Stream<Item = InboundP2pResponse> {
        let stream = stream! {
            while let Some(resp) = self.inbound_resp_rx.lock().await.recv().await {
                yield resp
            }
        };

        stream
    }

    async fn gossip_msg_stream(&self) -> impl Stream<Item = GossipMessage> {
        let stream = stream! {

            while let Some(msg) = self.gossip_msg_rx.lock().await.recv().await {
                yield msg
            }
        };

        stream
    }
}

impl NodeClient {
    fn new(
        req_tx: Sender<ClientRequest>,
        inbound_req_rx: Mutex<Receiver<InboundP2pRequest>>,
        inbound_resp_rx: Mutex<Receiver<InboundP2pResponse>>,
        gossip_msg_rx: Mutex<Receiver<GossipMessage>>,
    ) -> Self {
        let pending_inbound_req: Mutex<HashMap<InboundRequestId, ResponseChannel<P2pResponse>>> =
            Mutex::new(HashMap::new());

        Self {
            req_tx,
            inbound_req_rx,
            inbound_resp_rx,
            gossip_msg_rx,
            pending_inbound_req,
        }
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), Error> {
        let payload = ClientRequestPayload::Subscribe {
            topic: topic.trim().to_string(),
        };
        self.send_client_request(payload).await?;

        Ok(())
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
            .map_err(|err| Error::SendClientRequest {
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
            _ = sleep(TokioDuration::from_secs(5)) => {
                Err(Error::TimedOutWaitingForNodeResponse)
            },
            msg = receiver => {
                match msg {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into())
                }
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
    PeerDialed,
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

    pub fn message(&self) -> &[u8] {
        self.message.as_slice()
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
pub enum NetworkClientError {
    #[error("")]
    NetworkError {
        #[from]
        source: Error,
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
    RcgenError {
        #[from]
        source: libp2p::tls::certificate::GenError,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    Infallible {
        #[from]
        source: std::convert::Infallible,
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

    #[error("")]
    NodeResponse {
        #[from]
        source: oneshot::error::RecvError,
    },

    #[error("")]
    TimedOutWaitingForNodeResponse,

    #[error("")]
    UnexpectedClientResponse,

    #[error("")]
    SendResponseError,

    #[error("")]
    DialPeerError { err: DialError },

    #[error("")]
    InboundRequestIdNotFound,

    #[error("{err}")]
    SendClientRequest { err: String },

    #[error("{err}")]
    Other { err: String },
}
