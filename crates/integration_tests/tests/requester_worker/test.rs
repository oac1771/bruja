#[cfg(feature = "integration_tests")]
mod tests {
    use ink_env::DefaultEnvironment;
    use requester::{commands::submit_job::SubmitJobCmd, config::Config as ConfigR};
    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
    };
    use subxt::{error::Error, utils::AccountId32, SubstrateConfig};
    use subxt_signer::{sr25519::Keypair, SecretUri};
    use tests::test_utils::{Log, Runner};
    use tokio::time::{sleep, Duration, Instant};
    use utils::client::{Client, ClientError};
    use worker::{
        commands::{register::RegisterCmd, start::StartCmd},
        config::Config as ConfigW,
    };

    const CONTRACT_FILE_PATH: &'static str = "../../target/ink/catalog/catalog.contract";
    const CLIENT_WAIT_TIMEOUT: u64 = 10;

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

        worker_runner.assert_info_log_entry("Starting Worker").await;
        requester_runner
            .assert_info_log_entry("Job Request Submitted!")
            .await;
        worker_runner
            .assert_info_log_entry("Found JobRequest Event")
            .await;
        worker_runner
            .assert_info_log_entry("Published job acceptance")
            .await;
    }

    async fn instantiate_contract(suri: &str) -> AccountId32 {
        let signer = Keypair::from_uri(&SecretUri::from_str(suri).unwrap()).unwrap();
        let contract_client = get_client(&signer).await;
        let address = contract_client.instantiate("new").await.unwrap();

        address
    }

    async fn get_client(signer: &Keypair) -> Client<SubstrateConfig, DefaultEnvironment, Keypair> {
        let contract_client = match Client::<SubstrateConfig, DefaultEnvironment, Keypair>::new(
            CONTRACT_FILE_PATH,
            signer,
        )
        .await
        {
            Ok(client) => client,
            Err(mut client_err) => {
                let client = {
                    let start = Instant::now();
                    loop {
                        if Instant::now().checked_duration_since(start).unwrap()
                            < Duration::from_secs(CLIENT_WAIT_TIMEOUT)
                        {
                            if let ClientError::Subxt {
                                source: Error::Rpc(_),
                            } = client_err
                            {
                                println!("Waiting for rpc node to be ready...");
                                sleep(Duration::from_secs(1)).await;

                                match Client::<SubstrateConfig, DefaultEnvironment, Keypair>::new(
                                    CONTRACT_FILE_PATH,
                                    signer,
                                )
                                .await
                                {
                                    Ok(c) => {
                                        println!("Instantiating client");
                                        break c;
                                    }
                                    Err(err) => client_err = err,
                                }
                            }
                        } else {
                            panic!("Timedout waiting for client instantiation")
                        }
                    }
                };
                client
            }
        };

        contract_client
    }

    // add method here to fund accounts from sudo account

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
            let config = ConfigW::new(suri, CONTRACT_FILE_PATH.to_string());
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

            let _join_handle = tokio::spawn(async move { start_cmd.handle(config).await.unwrap() });
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
            let config = ConfigR::new(suri, CONTRACT_FILE_PATH.to_string());
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

            let _join_handle =
                tokio::spawn(async move { submit_job_cmd.handle(config).await.unwrap() });
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
