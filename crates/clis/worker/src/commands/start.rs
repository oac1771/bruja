use crate::{config::Config, controller::worker::WorkerController, error::Error};
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tracing::instrument;
use utils::services::contract_client::Client;

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
        )
        .await?;

        let worker_controller = WorkerController::new(contract_client, contract_address);

        worker_controller.start().await;

        Ok(())
    }

    // async fn join_network(&self) -> Result<(JoinHandle<Result<(), P2pError>>, NodeClient), Error> {
    //     let node = NodeBuilder::build()?;
    //     let (node_handle, node_client) = node.start()?;
    //     node_client.subscribe(&self.address).await?;

    //     Ok((node_handle, node_client))
    // }
}

//     async fn start(&self, node_handle: JoinHandle<Result<(), P2pError>>) -> Result<(), Error> {
//         info!("Starting Worker");

//         select! {
//             _ = node_handle => {},
//             result = self.listen_blocks() => {
//                 if let Err(err) = result {
//                     error!("Encountered Error: {}", err);
//                 }
//             },
//             _ = signal::ctrl_c() => {
//                 info!("Shutting down...")
//             }
//         };

//         Ok(())
//     }

//     async fn listen_blocks(&self) -> Result<(), Error> {
//         let contract_client: ContractClient<SubstrateConfig, DefaultEnvironment, Keypair> =
//             ContractClient::new(&self.config.artifact_file_path, &self.config.signer).await?;
//         let client = contract_client.online_client().await?;

//         let mut blocks_sub = client.blocks().subscribe_finalized().await?;

//         while let Some(block) = blocks_sub.next().await {
//             if let Err(error) = self.process_block(block).await {
//                 error!("Error Processing Block Data: {}", error);
//             }
//         }

//         Ok(())
//     }

//     async fn process_block(
//         &self,
//         block: Result<Block<SubstrateConfig, OnlineClient<SubstrateConfig>>, subxt::Error>,
//     ) -> Result<(), Error> {
//         let block = block?;
//         let extrinsics = block.extrinsics().await?;

//         for ext in extrinsics.iter() {
//             if let Err(err) = self.handle_extrinsics(ext).await {
//                 error!("Error processing Extrinsic: {}", err);
//             }
//         }
//         Ok(())
//     }

// async fn handle_extrinsics(
//     &self,
//     ext: Result<ExtrinsicDetails<SubstrateConfig, OnlineClient<SubstrateConfig>>, subxt::Error>,
// ) -> Result<(), Error> {
//     let ext = ext?;
//     let ext_events = ext.events().await?;

//     let events = ext_events
//         .find::<ContractEmitted>()
//         .filter_map(|ev| ev.ok())
//         .filter(|ev| ev.contract == self.contract_address);

//     self.handle_events(events).await?;
//     Ok(())
// }

//     async fn handle_events(
//         &self,
//         events: impl Iterator<Item = ContractEmitted>,
//     ) -> Result<(), Error> {
//         for event in events {
//             match self.determine_event(&event)? {
//                 WatchedEvents::JobRequest(job_request) => {
//                     info!("Found JobRequest Event");
//                     self.handle_job_request(job_request).await?;
//                 }
//             };
//         }

//         Ok(())
//     }

//     async fn handle_job_request(&self, job_request: JobRequestSubmitted) -> Result<(), Error> {
//         self.accept_job(job_request).await?;

//         Ok(())
//     }

//     async fn accept_job(&self, job_request: JobRequestSubmitted) -> Result<(), Error> {
//         let job_id = job_request.id.to_vec();
//         let msg = Gossip::JobAcceptance { job_id };

//         self.publish_message(msg.encode()).await?;
//         Ok(())
//     }

//     fn determine_event(&self, event: &ContractEmitted) -> Result<WatchedEvents, Error> {
//         if let Ok(event) = <JobRequestSubmitted as Decode>::decode(&mut event.data.as_slice()) {
//             Ok(WatchedEvents::JobRequest(event))
//         } else {
//             Err(Error::Decode {
//                 data: event.data.to_vec(),
//             })
//         }
//     }

//     async fn publish_message(&self, msg: Vec<u8>) -> Result<(), Error> {
//         let address = self.contract_address.to_string();

//         self.wait_for_gossip_peers(&address).await?;
//         self.node_client.publish(&address, msg).await?;

//         info!("Published job acceptance");

//         Ok(())
//     }

//     async fn wait_for_gossip_peers(&self, address: &str) -> Result<(), Error> {
//         let mut gossip_nodes = Vec::new();

//         while gossip_nodes.is_empty() {
//             info!("Waiting for gossip peers");
//             gossip_nodes = self.node_client.get_gossip_nodes(address).await?;
//             sleep(Duration::from_millis(500)).await;
//         }
//         info!("Connected to gossip peers");
//         Ok(())
//     }
// }
