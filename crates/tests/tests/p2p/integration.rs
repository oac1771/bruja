#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use tests::test_utils::{BufferWriter, Runner};
    use tracing_subscriber::util::SubscriberInitExt;
    use utils::p2p::{Node, NodeBuilder};

    #[tokio::test]
    async fn foo() {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer = log_buffer.clone();

        let _guard = tracing_subscriber::fmt()
            .json()
            .with_writer(move || BufferWriter {
                buffer: buffer.clone(),
            })
            .set_default();

        let node_runner_1 = NodeRunner::new(log_buffer.clone());
        let ndoe_runner_2 = NodeRunner::new(log_buffer.clone());

        let _ = node_runner_1.start();

        node_runner_1.assert_log_entry("foo").await;
    }

    struct NodeRunner {
        log_buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl NodeRunner {
        fn new(log_buffer: Arc<Mutex<Vec<u8>>>) -> Self {
            Self { log_buffer }
        }

        fn start(&self) {
            let node = NodeBuilder::build().unwrap();

            let (handle, node_client) = node.start().unwrap();
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
