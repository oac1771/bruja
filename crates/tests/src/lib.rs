#[cfg(test)]
mod tests {
    use ink_env::DefaultEnvironment;
    use std::sync::{Arc, Mutex};
    use subxt::SubstrateConfig;
    use subxt_signer::sr25519::Keypair;
    use utils::client::Client;

    use worker::{commands::register::RegisterCmd, config::Config};

    struct BufferWriter {
        buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl std::io::Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn foo() {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));

        // Clone the buffer so it can be passed to the tracing subscriber
        let log_buffer_for_tracing = log_buffer.clone();
    
        // Configure the tracing subscriber to write logs into the memory buffer
        tracing_subscriber::fmt()
            .with_writer(move || {
                // Returns a writer to capture logs into the memory buffer
                let buffer_clone = log_buffer_for_tracing.clone();
                BufferWriter { buffer: buffer_clone }
            })
            .init();

        let config = Config::new("//Alice", "../../target/ink/catalog/catalog.contract".to_string());
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer)
                .await
                .unwrap();
        let address = contract_client.instantiate("new").await.unwrap();
        let cmd = RegisterCmd {
            address: address.to_string(),
            val: 10
        };

        let _foo = cmd.handle(config).await.unwrap();

        let logs = log_buffer.lock().unwrap();
        let log_output = String::from_utf8(logs.clone()).expect("Failed to read logs as UTF-8");
        println!("Captured logs:\n{}", log_output);
    }
}
