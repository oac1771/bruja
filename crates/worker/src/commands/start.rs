use crate::{config::Config, error::Error};
use catalog::catalog::JobRequestSubmitted;
use clap::Parser;
use codec::Decode;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{blocks::Block, utils::AccountId32, OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tokio::{select, signal};
use tracing::{error, info, instrument};
use utils::{
    chain::contracts::events::ContractEmitted,
    client::Client,
    p2p::{Message, NodeBuilder, NodeClient},
};

enum WatchedEvents {
    JobRequest(JobRequestSubmitted),
    DecodeErr,
}

#[derive(Debug, Parser)]
pub struct StartCmd {
    #[arg(long)]
    pub address: String,
}

impl StartCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        info!("Starting worker");

        let node = NodeBuilder::build()?;
        let (handle, node_client) = node.start()?;
        node_client.subscribe(&self.address).await?;

        select! {
            _ = handle => {},
            _ = self.listen_blocks(config, node_client) => {},
            _ = signal::ctrl_c() => {
                info!("Shutting down...")
            }
        };

        Ok(())
    }

    async fn listen_blocks(&self, config: Config, node_client: NodeClient) -> Result<(), Error> {
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer).await?;
        let client = contract_client.online_client().await?;

        let mut blocks_sub = client.blocks().subscribe_finalized().await?;

        while let Some(block) = blocks_sub.next().await {
            if let Err(error) = self.process_block(block, &node_client).await {
                error!("Error Processing Block Data: {}", error);
            }
        }

        Ok(())
    }

    async fn process_block(
        &self,
        block: Result<Block<SubstrateConfig, OnlineClient<SubstrateConfig>>, subxt::Error>,
        node_client: &NodeClient,
    ) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|err| Error::Other(err.to_string()))?;
        let block = block?;
        let extrinsics = block.extrinsics().await?;

        for ext in extrinsics.iter() {
            let ext = ext?;
            let ext_events = ext.events().await?;

            let events = ext_events
                .find::<ContractEmitted>()
                .filter_map(|ev| ev.ok())
                .filter(|ev| ev.contract == contract_address);

            self.handle_events(events, node_client).await?;
        }

        Ok(())
    }

    async fn handle_events(
        &self,
        events: impl Iterator<Item = ContractEmitted>,
        node_client: &NodeClient,
    ) -> Result<(), Error> {
        for event in events {
            match self.determine_event(&event) {
                WatchedEvents::JobRequest(job_request) => {
                    info!("Found JobRequest Event");
                    self.handle_job_request(job_request, node_client).await?;
                }
                WatchedEvents::DecodeErr => error!("Error decoding event: {:?}", event.data),
            }
        }

        Ok(())
    }

    async fn handle_job_request(
        &self,
        job_request: JobRequestSubmitted,
        node_client: &NodeClient,
    ) -> Result<(), Error> {
        // add logic here to decide if worker wants to accept this job
        self.accept_job(job_request, node_client).await?;

        Ok(())
    }

    async fn accept_job(
        &self,
        job_request: JobRequestSubmitted,
        node_client: &NodeClient,
    ) -> Result<(), Error> {
        let job_acceptance = Message::JobAcceptance {
            job_id: job_request.id.to_vec(),
        };
        node_client.publish(&self.address, job_acceptance).await?;
        info!("Published job acceptance");
        Ok(())
    }

    fn determine_event(&self, event: &ContractEmitted) -> WatchedEvents {
        if let Ok(event) = <JobRequestSubmitted as Decode>::decode(&mut event.data.as_slice()) {
            WatchedEvents::JobRequest(event)
        } else {
            WatchedEvents::DecodeErr
        }
    }
}
