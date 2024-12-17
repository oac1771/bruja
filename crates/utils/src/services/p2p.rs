use async_stream::stream;
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
    fmt::{self, Debug, Display, Formatter},
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
    type Id;
    type NetworkId: NetworkIdT + Display;
    type GossipMessage: GossipMessageT<NetworkId = Self::NetworkId>;
    type Request: RequestT<Id = Self::Id>;
    type Response: ResponseT<Id = Self::Id>;

    fn publish_message(
        &self,
        topic: &str,
        msg: Vec<u8>,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send;

    fn send_request(
        &self,
        network_id: Self::NetworkId,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<Self::Id, Self::Err>> + Send;

    fn send_response(
        &self,
        id: Self::Id,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send;

    fn get_gossip_nodes(
        &self,
        topic: &str,
    ) -> impl Future<Output = Result<impl Iterator<Item = Self::NetworkId>, Self::Err>> + Send;

    fn get_local_network_id(
        &self,
    ) -> impl Future<Output = Result<Self::NetworkId, Self::Err>> + Send;

    fn gossip_msg_stream(
        &self,
    ) -> impl Future<Output = impl Stream<Item = Self::GossipMessage>> + Send;

    fn req_stream(&self) -> impl Future<Output = impl Stream<Item = Self::Request>> + Send;

    fn resp_stream(&self) -> impl Future<Output = impl Stream<Item = Self::Response>> + Send;
}

pub struct NodeBuilder;

pub struct Node {
    swarm: Swarm<Behavior>,
    pending_inbound_req: HashMap<u64, ResponseChannel<P2pResponse>>,
}

pub struct NodeClient {
    req_tx: Sender<ClientRequest>,
    inbound_req_rx: Mutex<Receiver<InboundP2pRequest>>,
    inbound_resp_rx: Mutex<Receiver<InboundP2pResponse>>,
    gossip_msg_rx: Mutex<Receiver<GossipMessage>>,
}

impl NodeBuilder {
    pub fn build() -> Result<Node, NetworkError> {
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
            .map_err(|err| NetworkError::Behavior {
                err: err.to_string(),
            })?
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

        let pending_inbound_req: HashMap<u64, ResponseChannel<P2pResponse>> = HashMap::new();

        Ok(Node {
            swarm,
            pending_inbound_req,
        })
    }
}

impl Node {
    pub fn start(
        mut self,
    ) -> Result<(JoinHandle<Result<(), NetworkError>>, NodeClient), NetworkError> {
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
    ) -> Result<(), NetworkError> {
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
                        Err(NetworkError::from(err))
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
                        Err(NetworkError::from(err))
                    } else {
                        info!("Subscribed to topic: {}", topic);
                        Ok(ClientResponse::Subscribe)
                    };
                Self::send_client_response(result, sender);
            }
            ClientRequestPayload::SendRequest {
                payload,
                network_id,
            } => {
                let request_id = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&network_id.inner(), P2pRequest(payload));

                let resp = ClientResponse::RequestId { request_id };
                Self::send_client_response(Ok(resp), sender);
            }
            ClientRequestPayload::SendResponse { payload, id } => {
                let result = if let Some(channel) = self.pending_inbound_req.remove(&id) {
                    if self
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
                        Err(NetworkError::SendResponseError)
                    }
                } else {
                    Err(NetworkError::ChannelNotFoundForGivenRequestId)
                };

                Self::send_client_response(result, sender);
            }
            ClientRequestPayload::GetLocalPeerId => {
                let peer_id = self.swarm.local_peer_id();
                let network_id = NetworkId::new(*peer_id);
                let resp = ClientResponse::NetworkId { network_id };
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
                    .map(|(p, _)| NetworkId::new(*p))
                    .collect::<Vec<NetworkId>>();

                let resp = ClientResponse::GossipNodes { gossip_nodes };
                Self::send_client_response(Ok(resp), sender);
            }
        };
    }

    fn send_client_response(
        result: Result<ClientResponse, NetworkError>,
        sender: oneshot::Sender<Result<ClientResponse, NetworkError>>,
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
                let network_id = NetworkId::new(peer_id);
                let gsp_msg = GossipMessage {
                    network_id,
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
                    let req = InboundP2pRequest { request, id };
                    let req_id = req.id();
                    match inbound_req_tx.send(req).await {
                        Ok(_) => {
                            self.pending_inbound_req.insert(req_id, channel);
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
    type Id = u64;
    type NetworkId = NetworkId;
    type GossipMessage = GossipMessage;
    type Request = InboundP2pRequest;
    type Response = InboundP2pResponse;

    async fn publish_message(&self, topic: &str, msg: Vec<u8>) -> Result<(), Self::Err> {
        let payload = ClientRequestPayload::Publish {
            topic: topic.to_string(),
            msg,
        };
        self.send_client_request(payload).await?;

        Ok(())
    }

    async fn get_local_network_id(&self) -> Result<Self::NetworkId, Self::Err> {
        let payload = ClientRequestPayload::GetLocalPeerId;

        if let ClientResponse::NetworkId { network_id } = self.send_client_request(payload).await? {
            return Ok(network_id);
        }
        Err(NetworkError::UnexpectedClientResponse.into())
    }

    async fn send_request(
        &self,
        network_id: Self::NetworkId,
        payload: Vec<u8>,
    ) -> Result<Self::Id, Self::Err> {
        let payload = ClientRequestPayload::SendRequest {
            payload,
            network_id,
        };

        if let ClientResponse::RequestId { request_id } = self.send_client_request(payload).await? {
            let id = hash_id(request_id);

            return Ok(id);
        }
        Err(NetworkError::UnexpectedClientResponse.into())
    }

    async fn send_response(&self, id: Self::Id, payload: Vec<u8>) -> Result<(), Self::Err> {
        let payload = ClientRequestPayload::SendResponse { payload, id };
        self.send_client_request(payload).await?;
        Ok(())
    }

    async fn get_gossip_nodes(
        &self,
        topic: &str,
    ) -> Result<impl Iterator<Item = Self::NetworkId>, Self::Err> {
        let payload = ClientRequestPayload::GetGossipNodes {
            topic: topic.to_string(),
        };
        if let ClientResponse::GossipNodes { gossip_nodes } =
            self.send_client_request(payload).await?
        {
            return Ok(gossip_nodes.into_iter());
        }
        Err(NetworkError::UnexpectedClientResponse.into())
    }

    async fn req_stream(&self) -> impl Stream<Item = Self::Request> {
        let stream = stream! {
            while let Some(req) = self.inbound_req_rx.lock().await.recv().await {
                yield req
            }
        };

        stream
    }

    async fn resp_stream(&self) -> impl Stream<Item = Self::Response> {
        let stream = stream! {
            while let Some(resp) = self.inbound_resp_rx.lock().await.recv().await {
                yield resp
            }
        };

        stream
    }

    async fn gossip_msg_stream(&self) -> impl Stream<Item = Self::GossipMessage> {
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
        Self {
            req_tx,
            inbound_req_rx,
            inbound_resp_rx,
            gossip_msg_rx,
        }
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), NetworkError> {
        let payload = ClientRequestPayload::Subscribe {
            topic: topic.trim().to_string(),
        };
        self.send_client_request(payload).await?;

        Ok(())
    }

    async fn send_client_request(
        &self,
        payload: ClientRequestPayload,
    ) -> Result<ClientResponse, NetworkError> {
        let (sender, receiver) = oneshot::channel::<Result<ClientResponse, NetworkError>>();
        let req = ClientRequest { payload, sender };

        self.req_tx
            .send(req)
            .await
            .map_err(|err| NetworkError::SendClientRequest {
                err: err.to_string(),
            })?;

        let resp = self.recv_node_response(receiver).await?;

        Ok(resp)
    }

    async fn recv_node_response(
        &self,
        receiver: oneshot::Receiver<Result<ClientResponse, NetworkError>>,
    ) -> Result<ClientResponse, NetworkError> {
        let result = select! {
            _ = sleep(TokioDuration::from_secs(5)) => {
                Err(NetworkError::TimedOutWaitingForNodeResponse)
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
    sender: oneshot::Sender<Result<ClientResponse, NetworkError>>,
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
        network_id: NetworkId,
        payload: Vec<u8>,
    },
    SendResponse {
        payload: Vec<u8>,
        id: u64,
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
    NetworkId { network_id: NetworkId },
    GossipNodes { gossip_nodes: Vec<NetworkId> },
}

pub trait NetworkIdT: Copy {
    fn to_vec(self) -> Vec<u8>;
    fn from_bytes(v: &[u8]) -> Self;
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct NetworkId(PeerId);

impl NetworkIdT for NetworkId {
    fn to_vec(self) -> Vec<u8> {
        self.0.to_bytes()
    }

    fn from_bytes(v: &[u8]) -> Self {
        let peer_id = PeerId::from_bytes(v).unwrap();
        Self(peer_id)
    }
}

impl NetworkId {
    fn new(peer_id: PeerId) -> Self {
        Self(peer_id)
    }

    fn inner(self) -> PeerId {
        self.0
    }
}

impl Debug for NetworkId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NetworkId")
            .field(&self.0.to_base58())
            .finish()
    }
}

impl Display for NetworkId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0.to_base58(), f)
    }
}

pub trait GossipMessageT {
    type NetworkId: NetworkIdT;
    fn message_ref(&self) -> &[u8];
    fn network_id(&self) -> Self::NetworkId;
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct GossipMessage {
    network_id: NetworkId,
    message: Vec<u8>,
}

impl GossipMessageT for GossipMessage {
    type NetworkId = NetworkId;

    fn network_id(&self) -> Self::NetworkId {
        self.network_id
    }

    fn message_ref(&self) -> &[u8] {
        self.message.as_slice()
    }
}

pub trait RequestT {
    type Id;

    fn body_ref(&self) -> &[u8];
    fn id(&self) -> Self::Id;
}

pub struct InboundP2pRequest {
    request: P2pRequest,
    id: InboundRequestId,
}

impl RequestT for InboundP2pRequest {
    type Id = u64;

    fn body_ref(&self) -> &[u8] {
        self.request.0.as_slice()
    }

    fn id(&self) -> Self::Id {
        hash_id(self.id)
    }
}

fn hash_id(val: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

pub trait ResponseT {
    type Id;

    fn id(&self) -> Self::Id;
    fn body_ref(&self) -> &[u8];
}

#[derive(Debug)]
pub struct InboundP2pResponse {
    response: P2pResponse,
    id: OutboundRequestId,
}

impl ResponseT for InboundP2pResponse {
    type Id = u64;

    fn body_ref(&self) -> &[u8] {
        self.response.0.as_slice()
    }

    fn id(&self) -> Self::Id {
        hash_id(self.id)
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
    Network {
        #[from]
        source: NetworkError,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("{source}")]
    Rcgen {
        #[from]
        source: libp2p::tls::certificate::GenError,
    },

    #[error("{source}")]
    Transport {
        #[from]
        source: libp2p::TransportError<std::io::Error>,
    },

    #[error("{source}")]
    MultiAddr {
        #[from]
        source: libp2p::multiaddr::Error,
    },

    #[error("{source}")]
    Subscription {
        #[from]
        source: libp2p::gossipsub::SubscriptionError,
    },

    #[error("{source}")]
    Publish {
        #[from]
        source: libp2p::gossipsub::PublishError,
    },

    #[error("Unable to receive response from node")]
    NodeResponse {
        #[from]
        source: oneshot::error::RecvError,
    },

    #[error("{err}")]
    Behavior { err: String },

    #[error("")]
    TimedOutWaitingForNodeResponse,

    #[error("")]
    UnexpectedClientResponse,

    #[error("")]
    SendResponseError,

    #[error("")]
    ChannelNotFoundForGivenRequestId,

    #[error("{err}")]
    SendClientRequest { err: String },
}

#[cfg(test)]
pub mod test {
    use stream::iter;

    use super::*;
    use std::{any::Any, collections::HashMap, marker::Send, sync::Mutex};

    use crate::services::test::Expectation;

    pub struct MockNodeClient {
        expectations: Mutex<HashMap<String, Box<dyn Any>>>,
    }

    impl MockNodeClient {
        fn _expect_publish_message(&mut self) -> &mut Expectation<Result<(), NetworkClientError>> {
            self._expectation::<Result<(), NetworkClientError>>("publish_message")
        }

        fn _expect_send_request(&mut self) -> &mut Expectation<Result<u64, NetworkClientError>> {
            self._expectation::<Result<u64, NetworkClientError>>("send_request")
        }

        fn _expect_send_response(&mut self) -> &mut Expectation<Result<(), NetworkClientError>> {
            self._expectation::<Result<(), NetworkClientError>>("send_response")
        }

        fn _expect_get_gossip_nodes(
            &mut self,
        ) -> &mut Expectation<Result<Vec<NetworkId>, NetworkClientError>> {
            self._expectation::<Result<Vec<NetworkId>, NetworkClientError>>("get_gossip_nodes")
        }

        fn _expect_get_local_network_id(
            &mut self,
        ) -> &mut Expectation<Result<NetworkId, NetworkClientError>> {
            self._expectation::<Result<NetworkId, NetworkClientError>>("get_local_network_id")
        }

        fn _expect_gossip_msg_stream(&mut self) -> &mut Expectation<Vec<GossipMessage>> {
            self._expectation::<Vec<GossipMessage>>("gossip_msg_stream")
        }

        fn _expect_req_stream(&mut self) -> &mut Expectation<Vec<InboundP2pRequest>> {
            self._expectation::<Vec<InboundP2pRequest>>("req_stream")
        }

        fn _expect_resp_stream(&mut self) -> &mut Expectation<Vec<InboundP2pResponse>> {
            self._expectation::<Vec<InboundP2pResponse>>("resp_stream")
        }

        fn _expectation<T: 'static>(&mut self, entry: &str) -> &mut Expectation<T> {
            self.expectations
                .get_mut()
                .unwrap()
                .entry(entry.to_string())
                .or_insert_with(|| {
                    let expectation: Expectation<T> = Expectation::new();
                    Box::new(Some(expectation))
                })
                .downcast_mut::<Expectation<T>>()
                .unwrap()
        }

        fn into_expectation<T: 'static>(&self, entry: &str) -> Box<dyn Fn() -> T + Send + Sync> {
            self.expectations
                .lock()
                .unwrap()
                .remove(entry)
                .unwrap()
                .downcast::<Expectation<T>>()
                .unwrap()
                .func()
                .unwrap()
        }
    }

    impl NetworkClient for MockNodeClient {
        type Err = NetworkClientError;
        type Id = u64;
        type NetworkId = NetworkId;
        type GossipMessage = GossipMessage;
        type Request = InboundP2pRequest;
        type Response = InboundP2pResponse;

        fn publish_message(
            &self,
            _topic: &str,
            _msg: Vec<u8>,
        ) -> impl Future<Output = Result<(), Self::Err>> + Send {
            let func = self.into_expectation::<Result<(), NetworkClientError>>("publish_message");

            async move { func.as_ref()() }
        }

        fn send_request(
            &self,
            _network_id: Self::NetworkId,
            _payload: Vec<u8>,
        ) -> impl Future<Output = Result<Self::Id, Self::Err>> + Send {
            let func = self.into_expectation::<Result<u64, NetworkClientError>>("send_request");

            async move { func.as_ref()() }
        }

        fn send_response(
            &self,
            _id: Self::Id,
            _payload: Vec<u8>,
        ) -> impl Future<Output = Result<(), Self::Err>> + Send {
            let func = self.into_expectation::<Result<(), NetworkClientError>>("send_response");

            async move { func.as_ref()() }
        }

        fn get_gossip_nodes(
            &self,
            _topic: &str,
        ) -> impl Future<Output = Result<impl Iterator<Item = Self::NetworkId>, Self::Err>> + Send
        {
            let func = self
                .into_expectation::<Result<Vec<NetworkId>, NetworkClientError>>("get_gossip_nodes");
            let res = func.as_ref()().and_then(|x| Ok(x.into_iter()));

            async { res }
        }

        fn get_local_network_id(
            &self,
        ) -> impl Future<Output = Result<Self::NetworkId, Self::Err>> + Send {
            let func = self
                .into_expectation::<Result<NetworkId, NetworkClientError>>("get_local_network_id");

            async move { func.as_ref()() }
        }

        fn gossip_msg_stream(
            &self,
        ) -> impl Future<Output = impl Stream<Item = Self::GossipMessage>> + Send {
            let func = self.into_expectation::<Vec<GossipMessage>>("gossip_msg_stream");
            let stream = iter(func.as_ref()().into_iter());

            async { stream }
        }

        fn req_stream(&self) -> impl Future<Output = impl Stream<Item = Self::Request>> + Send {
            let func = self.into_expectation::<Vec<InboundP2pRequest>>("req_stream");
            let stream = iter(func.as_ref()().into_iter());

            async { stream }
        }

        fn resp_stream(&self) -> impl Future<Output = impl Stream<Item = Self::Response>> + Send {
            let func = self.into_expectation::<Vec<InboundP2pResponse>>("resp_stream");
            let stream = iter(func.as_ref()().into_iter());

            async { stream }
        }
    }
}
