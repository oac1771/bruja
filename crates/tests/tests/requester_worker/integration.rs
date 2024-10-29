#[cfg(test)]
mod tests {
    use ink_env::DefaultEnvironment;
    use requester::{commands::submit_job::SubmitJobCmd, config::Config as ConfigR};
    use serde::Deserialize;
    use serde_json::Deserializer;
    use std::{
        io::{Cursor, Write},
        sync::{Arc, Mutex},
    };
    use subxt::{utils::AccountId32, SubstrateConfig};
    use subxt_signer::sr25519::Keypair;
    use tokio::{
        select,
        time::{sleep, Duration},
    };
    use tracing::{Instrument, Span};
    use tracing_subscriber::util::SubscriberInitExt;
    use utils::client::Client;
    use worker::{
        commands::{register::RegisterCmd, start::StartCmd},
        config::Config as ConfigW,
    };

    const ARTIFACT_FILE_PATH: &'static str = "../../target/ink/catalog/catalog.contract";

    #[tokio::test]
    async fn register_worker() {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer = log_buffer.clone();

        let _guard = tracing_subscriber::fmt()
            .json()
            .with_writer(move || BufferWriter {
                buffer: buffer.clone(),
            })
            .set_default();
        let address = instantiate_contract("//Alice").await;

        let worker_runner = WorkerRunner::new(address, "//Alice");
        worker_runner.register(10).await;
        worker_runner
            .assert_log_entry("Successfully registered worker!", log_buffer.clone())
            .await;
    }

    #[tokio::test]
    async fn submit_job() {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer = log_buffer.clone();

        let _guard = tracing_subscriber::fmt()
            .json()
            .with_writer(move || BufferWriter {
                buffer: buffer.clone(),
            })
            .set_default();

        let address = instantiate_contract("//Bob").await;

        let requester_runner = RequesterRunner::new(address.clone(), "//Bob");
        let worker_runner = WorkerRunner::new(address.clone(), "//Bob");

        worker_runner.start().await;
        requester_runner
            .submit_job(
                "tests/requester_worker/work_bg.wasm",
                "foo",
                Some(String::from("10")),
            )
            .await;

        // Log assertions
        worker_runner
            .assert_log_entry("Starting worker", log_buffer.clone())
            .await;
        requester_runner
            .assert_log_entry("Job Request Submitted!", log_buffer.clone())
            .await;
        worker_runner
            .assert_log_entry("Found JobRequest Event", log_buffer.clone())
            .await;
        worker_runner
            .assert_log_entry("Published job acceptance", log_buffer.clone())
            .await;
        requester_runner
            .assert_log_entry("Messages received!", log_buffer.clone())
            .await;
    }

    struct BufferWriter {
        buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    async fn instantiate_contract(suri: &str) -> AccountId32 {
        let config = ConfigW::new(suri, ARTIFACT_FILE_PATH.to_string());
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer)
                .await
                .unwrap();
        let address = contract_client.instantiate("new").await.unwrap();

        address
    }

    #[derive(Deserialize, Debug)]
    struct Log {
        fields: Fields,
        target: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Fields {
        #[allow(unused)]
        LocalPeerId {
            local_peer_id: String,
        },
        Message {
            message: String,
        },
    }

    struct WorkerRunner {
        config: ConfigW,
        address: AccountId32,
    }

    struct RequesterRunner {
        config: ConfigR,
        address: AccountId32,
    }

    impl WorkerRunner {
        fn new(address: AccountId32, suri: &str) -> Self {
            let config = ConfigW::new(suri, ARTIFACT_FILE_PATH.to_string());
            Self { config, address }
        }

        async fn register(&self, val: u32) {
            let register_cmd = RegisterCmd {
                address: self.address.to_string(),
                val,
            };

            register_cmd.handle(self.config.clone()).await.unwrap();
        }

        async fn start(&self) {
            let start_cmd = StartCmd {
                address: self.address.to_string(),
            };
            let config = self.config.clone();

            let span = Span::current();

            let _join_handle = tokio::spawn(async move { start_cmd.handle(config).await.unwrap() })
                .instrument(span);
        }
    }

    impl Runner for WorkerRunner {
        fn label() -> String {
            "worker::".to_string()
        }
    }

    impl RequesterRunner {
        fn new(address: AccountId32, suri: &str) -> Self {
            let config = ConfigR::new(suri, ARTIFACT_FILE_PATH.to_string());
            Self { config, address }
        }

        async fn submit_job(&self, path: &str, func_name: &str, params: Option<String>) {
            let submit_job_cmd = SubmitJobCmd {
                address: self.address.to_string(),
                path: path.to_string(),
                func_name: func_name.to_string(),
                params: params,
            };
            let config = self.config.clone();

            let span = Span::current();

            let _join_handle =
                tokio::spawn(async move { submit_job_cmd.handle(config).await.unwrap() })
                    .instrument(span);
        }
    }

    impl Runner for RequesterRunner {
        fn label() -> String {
            "requester::".to_string()
        }
    }

    trait Runner {
        fn label() -> String;

        async fn assert_log_entry(&self, entry: &str, log_buffer: Arc<Mutex<Vec<u8>>>) {
            select! {
                _ = sleep(Duration::from_secs(10)) => {
                    let buffer = log_buffer.lock().unwrap();
                    let output = String::from_utf8(buffer.clone()).unwrap();
                    panic!("Failed to find log entry: {}\nLogs: {}", entry.to_string(), output)
                },
                _ = self.parse_logs(entry, log_buffer.clone()) => {}
            }
        }

        async fn parse_logs(&self, entry: &str, log_buffer: Arc<Mutex<Vec<u8>>>) {
            let mut logs: Vec<Log> = vec![];

            while logs.len() == 0 {
                let buffer = log_buffer.lock().unwrap();
                let log_output = String::from_utf8(buffer.clone()).unwrap();
                std::mem::drop(buffer);

                let cursor = Cursor::new(log_output);

                logs = Deserializer::from_reader(cursor.clone())
                    .into_iter::<Log>()
                    .map(|log| log.unwrap())
                    .filter(|log| log.target.contains(&Self::label()))
                    .filter(|log| match &log.fields {
                        Fields::Message { message } => message == entry,
                        _ => false,
                    })
                    .collect::<Vec<Log>>();

                let _ = sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
