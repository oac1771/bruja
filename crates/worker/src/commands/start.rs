use crate::error::Error;
use catalog::catalog::{JobSubmitted, WorkerRegistered};
use utils::chain::contracts::events::ContractEmitted;

use clap::Parser;
use codec::Decode;
use std::str::FromStr;
use subxt::{
    blocks::Block, storage::StorageClient, utils::AccountId32, OnlineClient, SubstrateConfig,
};

enum WatchedEvents {
    Job(JobSubmitted),
    Registration(WorkerRegistered),
    DecodeErr,
}

#[derive(Debug, Parser)]
pub struct StartCmd {
    #[arg(long)]
    address: String,
}

impl StartCmd {
    pub async fn handle(&self) -> Result<(), Error> {
        let client = OnlineClient::<SubstrateConfig>::new().await?;
        let storage_client = client.storage();
        let mut blocks_sub = client.blocks().subscribe_finalized().await?;

        while let Some(block) = blocks_sub.next().await {
            if let Err(error) = self.process_block(block, &storage_client).await {
                println!("Error Processing Block: {}", error);
            }
        }

        Ok(())
    }

    async fn process_block(
        &self,
        block: Result<Block<SubstrateConfig, OnlineClient<SubstrateConfig>>, subxt::Error>,
        storage_client: &StorageClient<SubstrateConfig, OnlineClient<SubstrateConfig>>,
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

            self.handle_events(events, storage_client)?;
        }

        Ok(())
    }

    fn handle_events(
        &self,
        events: impl Iterator<Item = ContractEmitted>,
        _storage_client: &StorageClient<SubstrateConfig, OnlineClient<SubstrateConfig>>,
    ) -> Result<(), Error> {
        for event in events {
            match self.determine_event(event) {
                WatchedEvents::Job(job) => {
                    println!("Job Event!");
                    println!("Id: {:?}", job.id);
                    println!("Key: {:?}", job.key);
                }
                WatchedEvents::Registration(registration) => {
                    println!("Registration Event!: {:?}", registration.who)
                }
                WatchedEvents::DecodeErr => {}
            }
        }

        Ok(())
    }

    fn determine_event(&self, event: ContractEmitted) -> WatchedEvents {
        if let Ok(event) = <JobSubmitted as Decode>::decode(&mut event.data.as_slice()) {
            WatchedEvents::Job(event)
        } else if let Ok(event) = <WorkerRegistered as Decode>::decode(&mut event.data.as_slice()) {
            WatchedEvents::Registration(event)
        } else {
            WatchedEvents::DecodeErr
        }
    }
}
