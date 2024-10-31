#[cfg(test)]
mod tests {
    use ink_env::DefaultEnvironment;
    use requester::{commands::submit_job::SubmitJobCmd, config::Config as ConfigR};
    use std::sync::{Arc, Mutex};
    use subxt::{utils::AccountId32, SubstrateConfig};
    use subxt_signer::sr25519::Keypair;
    use tests::test_utils::{Log, Runner};
    use tracing::{Instrument, Span};
    use utils::client::Client;
    use worker::{
        commands::{register::RegisterCmd, start::StartCmd},
        config::Config as ConfigW,
    };

    const ARTIFACT_FILE_PATH: &'static str = "../../target/ink/catalog/catalog.contract";

    #[test_macro::test]
    async fn register_worker(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let address = instantiate_contract("//Alice").await;

        let worker_runner = WorkerRunner::new(address, "//Alice", log_buffer.clone());
        worker_runner.register(10).await;
        worker_runner
            .assert_info_log_entry("Successfully registered worker!")
            .await;
    }

    #[test_macro::test]
    async fn submit_job(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let address = instantiate_contract("//Bob").await;

        let requester_runner = RequesterRunner::new(address.clone(), "//Bob", log_buffer.clone());
        let worker_runner = WorkerRunner::new(address.clone(), "//Bob", log_buffer.clone());

        worker_runner.start().await;
        requester_runner
            .submit_job(
                "tests/requester_worker/work_bg.wasm",
                "foo",
                Some(String::from("10")),
            )
            .await;

        worker_runner.assert_info_log_entry("Starting worker").await;
        requester_runner
            .assert_info_log_entry("Job Request Submitted!")
            .await;
        worker_runner
            .assert_info_log_entry("Found JobRequest Event")
            .await;
        worker_runner
            .assert_info_log_entry("Published job acceptance")
            .await;
        requester_runner
            .assert_info_log_entry("Messages received!")
            .await;
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

    struct WorkerRunner {
        config: ConfigW,
        address: AccountId32,
        log_buffer: Arc<Mutex<Vec<u8>>>,
    }

    struct RequesterRunner {
        config: ConfigR,
        address: AccountId32,
        log_buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl WorkerRunner {
        fn new(address: AccountId32, suri: &str, log_buffer: Arc<Mutex<Vec<u8>>>) -> Self {
            let config = ConfigW::new(suri, ARTIFACT_FILE_PATH.to_string());
            Self {
                config,
                address,
                log_buffer,
            }
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
        fn label(&self) -> &str {
            "worker::"
        }

        fn log_filter(&self, log: &Log) -> bool {
            log.target().contains(self.label())
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }

    impl RequesterRunner {
        fn new(address: AccountId32, suri: &str, log_buffer: Arc<Mutex<Vec<u8>>>) -> Self {
            let config = ConfigR::new(suri, ARTIFACT_FILE_PATH.to_string());
            Self {
                config,
                address,
                log_buffer,
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
    }

    impl Runner for RequesterRunner {
        fn label(&self) -> &str {
            "requester::"
        }

        fn log_filter(&self, log: &Log) -> bool {
            log.target().contains(self.label())
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }
}
