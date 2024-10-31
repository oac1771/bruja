#[cfg(test)]
mod tests {
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
    async fn mdns_peer_discovery_success(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let node_1 = NodeRunner::new(log_buffer.clone(), "node_1");
        let node_2 = NodeRunner::new(log_buffer.clone(), "node_2");

        let (_, mut client_1) = node_1.start();
        let (_, mut client_2) = node_2.start();

        let peer_id_1 = client_1.get_local_peer_id().await.unwrap();
        let peer_id_2 = client_2.get_local_peer_id().await.unwrap();

        node_1
            .assert_log_entry(format!("mDNS discovered a new peer: {}", peer_id_2).as_str())
            .await;
        node_2
            .assert_log_entry(format!("mDNS discovered a new peer: {}", peer_id_1).as_str())
            .await;
    }
}
