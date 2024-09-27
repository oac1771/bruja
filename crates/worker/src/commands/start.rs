use crate::{config::Config, error::Error};
use catalog::catalog::JobSubmitted;
use utils::{chain::contracts::events::ContractEmitted, client::Client};

use clap::Parser;
use codec::Decode;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{blocks::Block, utils::AccountId32, OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use wasmtime::{Caller, Engine, Linker, Module, Store};

enum WatchedEvents {
    Job(JobSubmitted),
    DecodeErr,
}

#[derive(Debug, Parser)]
pub struct StartCmd {
    #[arg(long)]
    address: String,
}

impl StartCmd {
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        println!("Starting worker...");

        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer).await?;
        let client = contract_client.online_client().await?;

        let mut blocks_sub = client.blocks().subscribe_finalized().await?;

        while let Some(block) = blocks_sub.next().await {
            if let Err(error) = self.process_block(block, &contract_client).await {
                println!("Error Processing Block Data: {}", error);
            }
        }

        Ok(())
    }

    async fn process_block(
        &self,
        block: Result<Block<SubstrateConfig, OnlineClient<SubstrateConfig>>, subxt::Error>,
        contract_client: &Client<'_, SubstrateConfig, DefaultEnvironment, Keypair>,
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

            self.handle_events(events, contract_client).await?;
        }

        Ok(())
    }

    async fn handle_events(
        &self,
        events: impl Iterator<Item = ContractEmitted>,
        contract_client: &Client<'_, SubstrateConfig, DefaultEnvironment, Keypair>,
    ) -> Result<(), Error> {
        for event in events {
            match self.determine_event(&event) {
                WatchedEvents::Job(job_event) => {
                    println!("Found Job Event");
                    let job = contract_client
                        .read_storage::<Vec<u8>>(event.contract, "work", &job_event.id)
                        .await?;
                    self.start_job(job).await?;
                }
                WatchedEvents::DecodeErr => {}
            }
        }

        Ok(())
    }

    fn determine_event(&self, event: &ContractEmitted) -> WatchedEvents {
        if let Ok(event) = <JobSubmitted as Decode>::decode(&mut event.data.as_slice()) {
            WatchedEvents::Job(event)
        } else {
            WatchedEvents::DecodeErr
        }
    }

    async fn start_job(&self, job: Vec<u8>) -> Result<(), Error> {
        let engine = Engine::default();
        let module =
            Module::new(&engine, job).map_err(|err| Error::WasmTimeError { source: err })?;
        let mut store: Store<u32> = Store::new(&engine, 4);
        let mut linker = Linker::new(&engine);
        
        linker
            .func_wrap(
                "host",
                "host_func",
                |caller: Caller<'_, u32>, param: i32| {
                    println!("Got {} from WebAssembly", param);
                    println!("my host state is: {}", caller.data());
                },
            )
            .map_err(|err| Error::WasmTimeError { source: err })?;
        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(|err| Error::WasmTimeError { source: err })?;
        let hello = instance
            .get_typed_func::<(), ()>(&mut store, "hello")
            .map_err(|err| Error::WasmTimeError { source: err })?;

        hello
            .call(&mut store, ())
            .map_err(|err| Error::WasmTimeError { source: err })?;

        Ok(())
    }
}
