use crate::{config::Config, controller::requester::RequesterController, error::Error};
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tokio::task::JoinHandle;
use tracing::instrument;
use utils::services::{
    contract_client::Client,
    job::JobHandler,
    p2p::{NetworkClientError, NodeBuilder, NodeClient},
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

        let job_service = JobHandler::new(
            &self.code_path,
            self.parameters.clone(),
            &self.function_name,
        )
        .await?;

        let (handle, network_client) = self.join_network(contract_address.to_string()).await?;

        let submit_job_controller = RequesterController::new(
            contract_client,
            contract_address,
            job_service,
            network_client,
        );

        submit_job_controller.start(handle).await?;

        Ok(())
    }

    async fn join_network(
        &self,
        address: String,
    ) -> Result<(JoinHandle<Result<(), NetworkClientError>>, NodeClient), NetworkClientError> {
        let node = NodeBuilder::build()?;
        let (handle, network_client) = node.start()?;
        network_client.subscribe(&address).await?;

        Ok((handle, network_client))
    }
}
