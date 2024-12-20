#[cfg(feature = "integration_tests")]
mod tests {
    use ink_env::{DefaultEnvironment, Environment};
    use integration_tests::utils::{Log, Runner};
    use requester::{commands::submit_job::SubmitJobCmd, config::Config as ConfigR};
    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
    };
    use subxt::{
        config::Config,
        error::{Error as SubxtError, RpcError},
        ext::scale_value::{Composite, Value, ValueDef},
        tx::TxClient,
        utils::{AccountId32, MultiAddress},
        OnlineClient, SubstrateConfig,
    };
    use subxt_signer::{sr25519::Keypair, SecretUri};
    use tokio::time::{error::Elapsed, sleep, timeout, Duration, Instant};
    use utils::{
        chain,
        services::contract_client::{Client, Error},
    };
    use worker::{commands::start::StartCmd, config::Config as ConfigW};

    const CONTRACT_FILE_PATH: &'static str = "../../target/ink/catalog/catalog.contract";
    const CLIENT_WAIT_TIMEOUT: u64 = 60;
    const ACCOUNT_FUNDER: &'static str = "//Charlie";
    const CONTRACT_INSTANTIATOR: &'static str = "//Bob";
    const URL: &'static str = "ws://127.0.0.1:9944";

    #[test_macro::test]
    async fn submit_job(log_buffer: Arc<Mutex<Vec<u8>>>) {
        let contract_address = instantiate_contract().await;
        let worker_key_pair = Keypair::from_seed(rand::random::<[u8; 32]>()).unwrap();
        let requester_key_pair = Keypair::from_seed(rand::random::<[u8; 32]>()).unwrap();

        let fund_amount = 1_000_000_000_000;
        let value = 100_000;

        let worker_account_id = worker_key_pair.public_key().to_account_id();
        let requester_account_id = requester_key_pair.public_key().to_account_id();

        fund_account(worker_account_id.clone(), fund_amount).await;
        fund_account(requester_account_id.clone(), fund_amount).await;

        let requester_runner = RequesterRunner::new(
            contract_address.clone(),
            requester_key_pair,
            log_buffer.clone(),
        );
        let worker_runner = WorkerRunner::new(
            contract_address.clone(),
            worker_key_pair,
            log_buffer.clone(),
        );

        worker_runner.start().await;
        requester_runner
            .submit_job(
                "tests/requester_worker/work_bg.wasm",
                "add",
                Some(String::from("10,10")),
                value,
            )
            .await;

        worker_runner
            .assert_info_log_entry("Starting Worker Controller")
            .await;
        requester_runner
            .assert_info_log_entry("Job Request Submitted!")
            .await;
        worker_runner
            .assert_info_log_entry("Published job acceptance")
            .await;
        requester_runner
            .assert_info_log_contains("Job acceptance received from peer: ")
            .await;
        requester_runner
            .assert_info_log_contains("Job sent to peer: ")
            .await;
        worker_runner.assert_info_log_entry("Job received!").await;
        worker_runner
            .assert_info_log_entry("Job acknowledgement sent")
            .await;
        requester_runner
            .assert_info_log_contains("Job has been accepted by a worker")
            .await;
        worker_runner.assert_info_log_entry("Results sent").await;
        requester_runner
            .assert_info_log_contains("Received results")
            .await;
        requester_runner
            .assert_info_log_contains("Result acknowledgement sent")
            .await;
        worker_runner
            .assert_info_log_entry("Result acknowledged by requester")
            .await;

        requester_runner.assert_info_log_entry("Paid Worker!").await;
        requester_runner.assert_info_log_contains("Results: ").await;

        let worker_balance = get_balance(&worker_account_id).await;
        let requester_balance = get_balance(&requester_account_id).await;

        assert_eq!(worker_balance - fund_amount, value);
        assert!(requester_balance < fund_amount);
    }

    async fn instantiate_contract() -> AccountId32 {
        let signer =
            Keypair::from_uri(&SecretUri::from_str(CONTRACT_INSTANTIATOR).unwrap()).unwrap();
        let contract_client = get_contract_client(&signer).await;
        let tx_client = contract_client.online_client().await.unwrap().tx();

        let address = loop {
            match contract_client.instantiate("new", 0).await {
                Ok(addr) => {
                    break addr;
                }
                Err(Error::Subxt {
                    source: SubxtError::Rpc(RpcError::ClientError(client_err)),
                }) => {
                    if client_err.to_string().contains("Priority is too low:") {
                        wait_for_account_nonce(&tx_client, &signer.public_key().to_account_id())
                            .await
                            .unwrap();
                    }
                }
                Err(err) => {
                    panic!("Error while submitting extrinsic: {:?}", err)
                }
            }
        };

        address
    }

    async fn get_contract_client(
        signer: &Keypair,
    ) -> Client<SubstrateConfig, DefaultEnvironment, Keypair> {
        let start_time = Instant::now();

        let contract_client =
            loop {
                match Client::<SubstrateConfig, DefaultEnvironment, Keypair>::new(
                    CONTRACT_FILE_PATH,
                    signer,
                    URL,
                )
                .await
                {
                    Ok(client) => break client,
                    Err(Error::Subxt {
                        source: SubxtError::Rpc(RpcError::ClientError(_)),
                    }) => {
                        if let None = Instant::now().checked_duration_since(start_time).and_then(
                            |elapsed_time| {
                                Duration::from_secs(CLIENT_WAIT_TIMEOUT).checked_sub(elapsed_time)
                            },
                        ) {
                            panic!("Timedout waiting for client to be ready");
                        }
                        println!("waiting for rpc node...");
                        sleep(Duration::from_secs(3)).await;
                    }
                    Err(err) => panic!("Error while instantiating client: {}", err),
                }
            };

        contract_client
    }

    async fn fund_account(
        dest: impl Into<MultiAddress<AccountId32, ()>>,
        value: <DefaultEnvironment as Environment>::Balance,
    ) {
        let signer = Keypair::from_uri(&SecretUri::from_str(ACCOUNT_FUNDER).unwrap()).unwrap();
        let transfer_tx = chain::tx()
            .balances()
            .transfer_keep_alive(dest.into(), value);
        let chain_client = OnlineClient::<SubstrateConfig>::new().await.unwrap();

        loop {
            let signed_extrinsic = chain_client
                .tx()
                .create_signed(&transfer_tx, &signer, Default::default())
                .await
                .unwrap();

            match signed_extrinsic.submit_and_watch().await {
                Ok(tx_progress) => {
                    tx_progress.wait_for_finalized_success().await.unwrap();
                    break;
                }
                Err(SubxtError::Rpc(RpcError::ClientError(client_err))) => {
                    if client_err.to_string().contains("Priority is too low:") {
                        wait_for_account_nonce(
                            &chain_client.tx(),
                            &signer.public_key().to_account_id(),
                        )
                        .await
                        .unwrap();
                    }
                }
                Err(err) => {
                    panic!("Error while submitting extrinsic: {:?}", err)
                }
            };
        }
    }

    async fn wait_for_account_nonce(
        tx_client: &TxClient<SubstrateConfig, OnlineClient<SubstrateConfig>>,
        account_id: &<SubstrateConfig as Config>::AccountId,
    ) -> Result<(), Elapsed> {
        let nonce_check = async {
            let start_account_nonce = tx_client.account_nonce(account_id).await.unwrap();
            let mut account_nonce = start_account_nonce.clone();

            while account_nonce.checked_sub(start_account_nonce).unwrap() == 0 {
                sleep(Duration::from_secs(1)).await;
                account_nonce = tx_client.account_nonce(account_id).await.unwrap();
            }
        };

        timeout(Duration::from_secs(10), nonce_check).await?;
        Ok(())
    }

    async fn get_balance(account: &<SubstrateConfig as Config>::AccountId) -> u128 {
        let signer = Keypair::from_uri(&SecretUri::from_str(ACCOUNT_FUNDER).unwrap()).unwrap();
        let contract_client = get_contract_client(&signer).await;
        let account_addr =
            subxt::dynamic::storage("System", "Account", vec![Value::from_bytes(&account)]);

        let latest_block = contract_client
            .online_client()
            .await
            .unwrap()
            .blocks()
            .at_latest()
            .await
            .unwrap();
        let account = contract_client
            .online_client()
            .await
            .unwrap()
            .storage()
            .at(latest_block.hash())
            .fetch(&account_addr)
            .await
            .unwrap()
            .unwrap()
            .to_value()
            .unwrap();

        let acc_data = get_composite_field_value(&account, "data").unwrap();
        let balance = get_composite_field_value(&acc_data, "free").unwrap();

        balance.as_u128().unwrap()
    }

    fn get_composite_field_value<'a, T>(
        value: &'a Value<T>,
        field_name: &str,
    ) -> Option<&'a Value<T>> {
        if let ValueDef::Composite(Composite::Named(fields)) = &value.value {
            let (_, field) = fields.iter().find(|(name, _)| name == field_name).unwrap();
            return Some(field);
        }
        None
    }

    struct WorkerRunner {
        config: ConfigW,
        contract_address: AccountId32,
        log_buffer: Arc<Mutex<Vec<u8>>>,
    }

    struct RequesterRunner {
        config: ConfigR,
        contract_address: AccountId32,
        log_buffer: Arc<Mutex<Vec<u8>>>,
    }

    impl WorkerRunner {
        fn new(
            contract_address: AccountId32,
            signer: Keypair,
            log_buffer: Arc<Mutex<Vec<u8>>>,
        ) -> Self {
            let config = ConfigW {
                signer,
                artifact_file_path: CONTRACT_FILE_PATH.to_string(),
                url: URL.to_string(),
            };

            Self {
                config,
                contract_address,
                log_buffer,
            }
        }

        async fn start(&self) {
            let start_cmd = StartCmd {
                address: self.contract_address.to_string(),
            };
            let config = self.config.clone();

            let _join_handle = tokio::spawn(async move { start_cmd.handle(config).await.unwrap() });
        }
    }

    impl Runner for WorkerRunner {
        fn log_filter(&self, log: &Log) -> bool {
            log.target().contains("worker::")
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }

    impl RequesterRunner {
        fn new(
            contract_address: AccountId32,
            signer: Keypair,
            log_buffer: Arc<Mutex<Vec<u8>>>,
        ) -> Self {
            let config = ConfigR {
                signer,
                artifact_file_path: CONTRACT_FILE_PATH.to_string(),
                url: URL.to_string(),
            };
            Self {
                config,
                contract_address,
                log_buffer,
            }
        }

        async fn submit_job(
            &self,
            path: &str,
            func_name: &str,
            parameters: Option<String>,
            value: u128,
        ) {
            let submit_job_cmd = SubmitJobCmd {
                address: self.contract_address.to_string(),
                value,
                code_path: path.to_string(),
                function_name: func_name.to_string(),
                parameters,
            };
            let config = self.config.clone();

            let _join_handle =
                tokio::spawn(async move { submit_job_cmd.handle(config).await.unwrap() });
        }
    }

    impl Runner for RequesterRunner {
        fn log_filter(&self, log: &Log) -> bool {
            log.target().contains("requester::")
        }

        fn log_buffer(&self) -> Arc<Mutex<Vec<u8>>> {
            self.log_buffer.clone()
        }
    }
}
