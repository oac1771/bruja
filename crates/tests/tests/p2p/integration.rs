#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use tests::test_utils::{BufferWriter, Runner};
    use tokio::task::JoinHandle;
    use tracing::info_span;
    use tracing_subscriber::util::SubscriberInitExt;
    use utils::p2p::{Error, NodeBuilder, NodeClient};

    #[tokio::test]
    async fn peer_discovery_success() {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer = log_buffer.clone();

        let _guard = tracing_subscriber::fmt()
            .json()
            .with_writer(move || BufferWriter {
                buffer: buffer.clone(),
            })
            .set_default();

        let node_runner_1 = NodeRunner::new(log_buffer.clone(), "foo".to_string());
        let node_runner_2 = NodeRunner::new(log_buffer.clone(), "bar".to_string());

        let (_foo, _node_client_1) = node_runner_1.start();
        let (_bar, _node_client_2) = node_runner_2.start();

        node_runner_1.assert_log_entry("foo").await;
    }

    struct NodeRunner {
        log_buffer: Arc<Mutex<Vec<u8>>>,
        name: String,
    }

    impl NodeRunner {
        fn new(log_buffer: Arc<Mutex<Vec<u8>>>, name: String) -> Self {
            Self { log_buffer, name }
        }

        fn start(&self) -> (JoinHandle<Result<(), Error>>, NodeClient) {
            let span = info_span!("{}", self.name).entered();

            let node = NodeBuilder::build().unwrap();
            let (handle, node_client) = node.start().unwrap();

            (handle, node_client)
        }
    }

    impl Runner for NodeRunner {
        fn target() -> String {
            "utils::p2p".to_string()
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }
}
