use crate::{config::Config, controller::requester::RequesterController, error::Error};
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tokio::{select, signal::ctrl_c, task::JoinHandle};
use tracing::{error, info, instrument};
use utils::services::{
    contract_client::Client,
    job::job_handler::JobHandler,
    p2p::{NetworkError, NodeBuilder, NodeClient},
};

#[derive(Debug, Parser)]
pub struct SubmitJobCmd {
    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub code_path: String,

    #[arg(long)]
    pub function_name: String,

    /// A comma seperated list of paramameters to pass to your function
    #[arg(long)]
    pub parameters: Option<String>,
}

impl SubmitJobCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|_| Error::ParsingContractAddress)?;

        let contract_client = Client::<SubstrateConfig, DefaultEnvironment, Keypair>::new(
            &config.artifact_file_path,
            &config.signer,
        )
        .await?;

        let job_handler_service = JobHandler::new(
            &self.code_path,
            self.parameters.clone(),
            &self.function_name,
        )
        .await?;

        let (handle, network_client) = self.join_network(contract_address.to_string()).await?;

        let submit_job_controller = RequesterController::new(
            contract_client,
            contract_address,
            job_handler_service,
            network_client,
        );

        self.start(submit_job_controller, handle).await?;

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
        controller: RequesterController<
            SubstrateConfig,
            Client<'_, SubstrateConfig, DefaultEnvironment, Keypair>,
            JobHandler,
            NodeClient,
        >,
        handle: JoinHandle<Result<(), NetworkError>>,
    ) -> Result<(), Error> {
        let result = select! {
            handle_result = handle => {
                let result = match handle_result {
                    Ok(_) => {
                        error!("Network handler stopped unexpectedly");
                        Err(Error::NetworkHandlerStopped)
                    },
                    Err(err) => {
                        error!("Network handler stopped and returned err: {}", err);
                        Err(Error::from(err))
                    }
                };
                result
            },
            controller_result = controller.run() => {
                if let Err(err) = controller_result {
                    error!("Encountered error while submitting job: {:?}", err);
                    Err(Error::from(err))
                } else {
                    Ok(())
                }
            },
            _ = ctrl_c() => {
                info!("Shutting down...");
                Ok(())
            }
        }?;

        Ok(result)
    }
}
