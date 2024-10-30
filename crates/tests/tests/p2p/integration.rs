#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use tests::test_utils::{BufferWriter, Runner};
    use utils::p2p::{NodeBuilder, Node};
    use tracing_subscriber::util::SubscriberInitExt;

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
    }

    struct NodeRunner {
        log_buffer: Arc<Mutex<Vec<u8>>>,
        node: Node
    }

    impl NodeRunner {
        fn new(log_buffer: Arc<Mutex<Vec<u8>>>) -> Self {
            let node = NodeBuilder::build().unwrap();
            Self { log_buffer, node }
        }
    }

    impl Runner for NodeRunner {
        fn label() -> String {
            "foo::".to_string()
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }
}
