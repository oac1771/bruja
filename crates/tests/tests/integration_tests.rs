#[cfg(test)]
mod tests {
    use futures::{future::BoxFuture, FutureExt};
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
    use worker::{commands::register::RegisterCmd, config::Config as ConfigW};

    const ARTIFACT_FILE_PATH: &'static str = "../../target/ink/catalog/catalog.contract";

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
    struct Fields {
        message: String,
    }

    struct WorkerRunner {
        log_buffer: Arc<Mutex<Vec<u8>>>,
        config: ConfigW,
        address: AccountId32,
    }

    struct RequesterRunner {
        log_buffer: Arc<Mutex<Vec<u8>>>,
        config: ConfigR,
        address: AccountId32,
    }

    impl WorkerRunner {
        fn new(log_buffer: Arc<Mutex<Vec<u8>>>, address: AccountId32, suri: &str) -> Self {
            let config = ConfigW::new(suri, ARTIFACT_FILE_PATH.to_string());
            Self {
                log_buffer,
                config,
                address,
            }
        }

        async fn register(&self, val: u32) {
            let register_cmd = RegisterCmd {
                address: self.address.to_string(),
                val,
            };

            register_cmd.handle(&self.config).await.unwrap();
            let logs = self.parse_logs("Successfully registered worker!");
            assert!(logs.len() > 0);
        }

        fn parse_logs(&self, msg: &str) -> Vec<Log> {
            let logs = self.log_buffer.lock().unwrap();
            let log_output = String::from_utf8(logs.clone()).unwrap();
            let cursor = Cursor::new(log_output);

            Deserializer::from_reader(cursor)
                .into_iter::<Log>()
                .filter_map(|log| log.ok())
                .filter(|log| log.target.contains("worker::") && log.fields.message == msg)
                .collect::<Vec<Log>>()
        }
    }

    impl RequesterRunner {
        fn new(log_buffer: Arc<Mutex<Vec<u8>>>, address: AccountId32, suri: &str) -> Self {
            let config = ConfigR::new(suri, ARTIFACT_FILE_PATH.to_string());
            Self {
                log_buffer,
                config,
                address,
            }
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

        async fn assert_log_entry(&self, entry: &str) {
            select! {
                _ = sleep(Duration::from_secs(10)) => panic!("Failed to find entry: {}", entry.to_string()),
                _ = self.parse_logs(entry) => println!("")
            }
        }

        async fn parse_logs(&self, entry: &str) {
            let mut logs: Vec<Log> = vec![];

            while logs.len() == 0 {
                let log_buffer = self.log_buffer.lock().unwrap();
                let log_output = String::from_utf8(log_buffer.clone()).unwrap();
                std::mem::drop(log_buffer);

                let cursor = Cursor::new(log_output);

                logs = Deserializer::from_reader(cursor.clone())
                    .into_iter::<Log>()
                    .filter_map(|log| log.ok())
                    .filter(|log| log.target.contains("requester::") && log.fields.message == entry)
                    .collect::<Vec<Log>>();

                let _ = sleep(Duration::from_millis(100)).await;
            }
        }
    }

    async fn test<'a, T>(test: T)
    where
        T: FnOnce(Arc<Mutex<Vec<u8>>>) -> BoxFuture<'a, Result<(), anyhow::Error>>,
    {
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer = log_buffer.clone();

        let _guard = tracing_subscriber::fmt()
            .json()
            .with_writer(move || BufferWriter {
                buffer: buffer.clone(),
            })
            .set_default();

        let result = test(log_buffer).await;

        if let Err(err) = result {
            panic!("{}", err);
        }
    }

    #[tokio::test]
    async fn register_worker() {
        test(|log_buffer| {
            async move {
                let address = instantiate_contract("//Alice").await;

                let worker_runner = WorkerRunner::new(log_buffer.clone(), address, "//Alice");
                worker_runner.register(10).await;

                Ok(())
            }
            .boxed()
        })
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

        let requester_runner = RequesterRunner::new(log_buffer.clone(), address, "//Bob");
        requester_runner
            .submit_job("tests/work_bg.wasm", "foo", Some(String::from("10")))
            .await;
        requester_runner
            .assert_log_entry("Job Request Submitted!")
            .await;
    }
}
