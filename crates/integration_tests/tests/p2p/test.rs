#[cfg(feature = "integration_tests")]
mod tests {
    use rand::{
        distributions::Alphanumeric,
        {thread_rng, Rng},
    };
    use std::sync::{Arc, Mutex};
    use tests::test_utils::{Log, Runner};
    use tokio::task::JoinHandle;
    use tracing::instrument;
    use utils::p2p::{Error, NodeBuilder, NodeClient};

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

    #[test_macro::test]
    async fn mdns_and_gossip_discovery_success(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let topic: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(|val| char::from(val))
            .collect();

        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        client_1.subscribe(&topic).await.unwrap();
        client_2.subscribe(&topic).await.unwrap();

        let peer_id_1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id_2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_2))
            .await;
        node_2
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_1))
            .await;

        node_1
            .assert_info_log_entry(&format!("A remote subscribed to a topic: {}", topic))
            .await;
        node_2
            .assert_info_log_entry(&format!("A remote subscribed to a topic: {}", topic))
            .await;

        let gossip_nodes_1 = client_1.get_gossip_nodes(&topic).await.unwrap();
        let gossip_nodes_2 = client_2.get_gossip_nodes(&topic).await.unwrap();

        assert_eq!(gossip_nodes_1[0], peer_id_2);
        assert_eq!(gossip_nodes_2[0], peer_id_1);
    }

    #[test_macro::test]
    async fn publish_gossip_message(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let topic: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(|val| char::from(val))
            .collect();
        let msg = vec![1, 2, 3, 4, 5];

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

        client_1.publish(&topic, msg.clone()).await.unwrap();
        node_1
            .assert_info_log_entry(&format!(
                "Successfully published message to {} topic",
                topic
            ))
            .await;
        node_2
            .assert_info_log_entry("Gossip message relayed to client")
            .await;
    }

    #[test_macro::test]
    async fn read_gossip_message_stream(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let topic: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(|val| char::from(val))
            .collect();
        let expected_msg = vec![1, 2, 3, 4, 5];
        let noise_msg = vec![5, 4, 3, 2, 1];

        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        client_1.subscribe(&topic).await.unwrap();
        client_2.subscribe(&topic).await.unwrap();

        let peer_id_1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id_2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_2))
            .await;
        node_2
            .assert_info_log_entry(&format!("mDNS discovered a new peer: {}", peer_id_1))
            .await;

        client_1.publish(&topic, noise_msg.clone()).await.unwrap();
        client_1
            .publish(&topic, expected_msg.clone())
            .await
            .unwrap();

        node_2
            .assert_info_log_entry("Gossip message relayed to client")
            .await;
        node_2
            .assert_info_log_entry("Gossip message relayed to client")
            .await;

        let result = loop {
            if let Some(msg) = client_2.recv_gossip_msg().await {
                if msg.message() == expected_msg {
                    break true;
                }
            } else {
                panic!("No gossip messages received")
            }
        };

        assert!(result);
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
        let msg = vec![1, 2, 3, 4];

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

        client_1
            .send_request(peer_id2, vec![1, 2, 3])
            .await
            .unwrap();

        node_2
            .assert_info_log_entry("Inbound request relayed to client")
            .await;
    }

    #[test_macro::test]
    async fn send_response_to_node(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");
        let expected_payload = vec![1, 2, 3, 4];
        let noise_payload = vec![4, 3, 2, 1];

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
            .send_request(peer_id2, expected_payload)
            .await
            .unwrap();
        client_1
            .send_request(peer_id2, noise_payload)
            .await
            .unwrap();
        node_2
            .assert_info_log_entry(&format!("Received request from peer: {}", peer_id1))
            .await;
        node_2
            .assert_info_log_entry("Inbound request relayed to client")
            .await;

        let id = loop {
            if let Some((req_id, _)) = client_2.recv_inbound_req().await {
                if req_id.to_string() == expected_id.to_string() {
                    break req_id;
                }
            } else {
                panic!("No requests received")
            }
        };

        let expected_response = vec![0, 0, 0];

        client_2
            .send_response(id.clone(), expected_response.clone())
            .await
            .unwrap();

        node_2
            .assert_info_log_entry("Response successfully sent")
            .await;

        node_1
            .assert_info_log_entry(&format!("Received response from peer: {}", peer_id2))
            .await;
        node_1
            .assert_info_log_entry("Inbound response relayed to client")
            .await;

        let result_response = loop {
            match client_1.foo_recv() {
                Ok(resp) => {
                    if resp.id().to_string() == id.to_string() {
                        break resp.response().clone();
                    }
                }
                Err(err) => panic!("Error: {}", err),
            };
            // if let Some(resp) = client_1.recv_inbound_resp().await {
            //     if resp.id().to_string() == id.to_string() {
            //         break resp.response().clone();
            //     }
            // } else {
            //     panic!("No responses received");
            // }
        };
        assert_eq!(result_response.0, expected_response);
    }
}
