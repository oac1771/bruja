#[cfg(feature = "integration_tests")]
mod tests {
    use codec::{Decode, Encode};
    use rand::{
        distributions::Alphanumeric,
        {thread_rng, Rng},
    };
    use std::sync::{Arc, Mutex};
    use tests::test_utils::{Log, Runner};
    use tokio::task::JoinHandle;
    use tracing::instrument;
    use utils::p2p::{Error, Message, NodeBuilder, NodeClient};

    struct NodeRunner<'a> {
        log_buffer: Arc<Mutex<Vec<u8>>>,
        name: &'a str,
    }

    impl<'a> NodeRunner<'a> {
        fn new(log_buffer: Arc<Mutex<Vec<u8>>>, name: &'a str) -> Self {
            Self { log_buffer, name }
        }

        #[instrument(skip(self), fields(label = %self.name))]
        fn start(&self) -> (JoinHandle<Result<(), Error>>, NodeClient) {
            let node = NodeBuilder::build().unwrap();
            let (handle, node_client) = node.start().unwrap();

            (handle, node_client)
        }
    }

    impl<'a> Runner for NodeRunner<'a> {
        fn label(&self) -> &str {
            self.name
        }

        fn log_filter(&self, log: &Log) -> bool {
            log.spans()
                .into_iter()
                .any(|val| val.to_string().contains(self.name))
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    struct JobPayload {
        pub job: Vec<u8>,
    }

    #[test_macro::test]
    async fn mdns_peer_discovery_success(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        let peer_id_1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id_2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_2))
            .await;
        node_2
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_1))
            .await;
    }

    #[test_macro::test]
    async fn publish_gossip_message(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let topic: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(|val| char::from(val))
            .collect();
        let expected_job_id = vec![1, 2, 3, 4, 5];

        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        let peer_id_1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id_2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_2))
            .await;
        node_2
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_1))
            .await;

        client_1.subscribe(&topic).await.unwrap();
        client_2.subscribe(&topic).await.unwrap();

        node_1
            .assert_info_log_entry(&format!("Subscribed to topic: {}", topic))
            .await;
        node_2
            .assert_info_log_entry(&format!("Subscribed to topic: {}", topic))
            .await;

        let msg = Message::JobAcceptance {
            job_id: expected_job_id.clone(),
        };
        client_1.publish(&topic, msg).await.unwrap();
        node_1
            .assert_info_log_entry(&format!(
                "Successfully published message to {} topic",
                topic
            ))
            .await;

        let msgs = client_2.get_gossip_messages().await.unwrap();

        assert!(msgs.len() > 0);
        msgs.into_iter().for_each(|msg| {
            let Message::JobAcceptance { job_id } = msg.message();
            assert_eq!(job_id, expected_job_id.clone())
        });
    }

    #[test_macro::test]
    async fn publish_gossip_message_fails_with_insuficient_peers_when_there_are_no_peers(
        log_buffer: Arc<Mutex<Vec<u8>>>,
    ) {
        let topic: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(|val| char::from(val))
            .collect();
        let msg = Message::JobAcceptance { job_id: vec![] };

        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");

        let (_, client_1) = node_1.start();
        client_1.subscribe(&topic).await.unwrap();
        if let Err(Error::PublishError { source }) = client_1.publish(&topic, msg).await {
            if let libp2p::gossipsub::PublishError::InsufficientPeers = source {
                node_1
                    .assert_error_log_entry(&format!("Publishing Error: {}", source))
                    .await;
            }
        } else {
            panic!("Publish command did not error")
        }
    }

    #[test_macro::test]
    async fn send_request_to_node(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        let peer_id1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id2))
            .await;
        node_2
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id1))
            .await;

        let expected_payload = JobPayload { job: vec![1, 2, 3] };
        let expected_id = client_1
            .send_request(peer_id2, expected_payload.clone())
            .await
            .unwrap();

        node_2
            .assert_info_log_entry("Request relayed to client")
            .await;

        let (req, id) = client_2.read_inbound_requests().await.unwrap();
        let payload = <JobPayload as Decode>::decode(&mut req.0.as_slice()).unwrap();

        assert_eq!(payload, expected_payload);
        assert_eq!(id.to_string(), expected_id.to_string());
    }

    #[test_macro::test]
    async fn send_response_to_node(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");
        let expected_payload = JobPayload { job: vec![1, 2, 3] };

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        let peer_id1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id2))
            .await;
        node_2
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id1))
            .await;

        let expected_id = client_1
            .send_request(peer_id2, expected_payload.clone())
            .await
            .unwrap();
        node_2
            .assert_info_log_entry("Request relayed to client")
            .await;

        let (req, id) = client_2.read_inbound_requests().await.unwrap();
        let payload = <JobPayload as Decode>::decode(&mut req.0.as_slice()).unwrap();

        client_2.send_response(id, payload).await.unwrap();
        node_1
            .assert_info_log_entry("Response relayed to client")
            .await;

        let (resp, result_id) = client_1.read_inbound_responses().await.unwrap();
        let result_payload = <JobPayload as Decode>::decode(&mut resp.0.as_slice()).unwrap();

        assert_eq!(result_id.to_string(), expected_id.to_string());
        assert_eq!(result_payload, expected_payload);
    }
}