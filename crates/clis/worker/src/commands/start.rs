use crate::{config::Config, controller::worker::WorkerController, error::Error};
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tokio::{select, signal::ctrl_c, task::JoinHandle};
use tracing::{error, info, instrument};
use utils::services::{
    contract_client::Client,
    job::job_runner::WasmJobRunner,
    p2p::{NetworkError, NodeBuilder, NodeClient},
};

#[derive(Debug, Parser)]
pub struct StartCmd {
    #[arg(long)]
    pub address: String,
}

impl StartCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|_| Error::ParsingContractAddress)?;
        let contract_client = Client::<SubstrateConfig, DefaultEnvironment, Keypair>::new(
            &config.artifact_file_path,
            &config.signer,
            &config.url,
        )
        .await?;

        let (handle, network_client) = self.join_network(contract_address.to_string()).await?;

        let job_runner = WasmJobRunner::new();

        let worker_controller: WorkerController<
            SubstrateConfig,
            Keypair,
            Client<'_, SubstrateConfig, DefaultEnvironment, Keypair>,
            NodeClient,
            WasmJobRunner,
        > = WorkerController::new(
            contract_address,
            config.signer.clone(),
            contract_client,
            network_client,
            job_runner,
        );

        self.start(worker_controller, handle).await?;

        Ok(())
    }

    async fn join_network(
        &self,
        address: String,
    ) -> Result<(JoinHandle<Result<(), NetworkError>>, NodeClient), NetworkError> {
        let node = NodeBuilder::build()?;
        let (handle, network_client) = node.start()?;
        network_client.subscribe(&address).await?;

        Ok((handle, network_client))
    }

    async fn start(
        &self,
        controller: WorkerController<
            SubstrateConfig,
            Keypair,
            Client<'_, SubstrateConfig, DefaultEnvironment, Keypair>,
            NodeClient,
            WasmJobRunner,
        >,
        handle: JoinHandle<Result<(), NetworkError>>,
    ) -> Result<(), Error> {
        select! {
            handle_result = handle => {
                match handle_result {
                    Ok(_) => {
                        error!("Network handler stopped unexpectedly");
                        Err(Error::NetworkHandlerStopped)
                    },
                    Err(err) => {
                        error!("Network handler stopped and returned err: {}", err);
                        Err(Error::from(err))
                    }
                }
            },
            controller_result = controller.listen() => {
                match controller_result {
                    Ok(_) => Err(Error::WorkerStoppedUnexpectedly),
                    Err(err) => Err(Error::from(err))
                }

            },
            _ = ctrl_c() => {
                info!("Shutting down...");
                Ok(())
            }
        }?;

        Ok(())
    }
}
