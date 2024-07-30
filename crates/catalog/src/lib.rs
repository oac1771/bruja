#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
pub mod catalog {

    use codec::{Decode, Encode};
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CatalogError {
        WorkerNotFound,
    }

    #[ink(storage)]
    pub struct Catalog {
        workers: Mapping<AccountId, u32>,
    }

    impl Default for Catalog {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Catalog {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                workers: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn get_worker(&self) -> Result<u32, CatalogError> {
            let caller = self.env().caller();
            let result = self
                .workers
                .get(caller)
                .ok_or(CatalogError::WorkerNotFound)?;

            Ok(result)
        }

        #[ink(message)]
        pub fn set_worker(&mut self, val: u32) -> bool {
            let caller = self.env().caller();
            self.workers.insert(caller, &val);

            // so it can be checked by 3rd party

            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let catalog = Catalog::default();
            assert_eq!(catalog.get_worker(), Err(CatalogError::WorkerNotFound));
        }
    }

    // #[cfg(all(test, feature = "e2e-tests"))]
    #[cfg(test)]
    mod e2e_tests {
        use rand::Rng;
        use subxt::{utils::AccountId32, Error, OnlineClient, SubstrateConfig};
        use subxt_signer::sr25519::{dev, Keypair};

        #[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
        pub mod chain {}

        use chain::{
            contracts::events::Instantiated, runtime_types::sp_weights::weight_v2::Weight,
        };

        const PROOF_SIZE: u64 = u64::MAX / 2;

        const CONTRACT: &str = r#"
            (module
                (import "env" "memory" (memory 1 1))
                (func (export "deploy"))
                (func (export "call"))
            )
        "#;

        struct TestCtx {
            client: OnlineClient<SubstrateConfig>,
            signer: Keypair,
        }

        impl TestCtx {
            async fn new(signer: Keypair) -> Self {
                Self {
                    client: OnlineClient::<SubstrateConfig>::new().await.unwrap(),
                    signer,
                }
            }

            async fn deploy_contract(&self) -> AccountId32 {
                let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

                let salt: u8 = rand::thread_rng().gen();

                let instantiate_tx = chain::tx().contracts().instantiate_with_code(
                    0,
                    Weight {
                        ref_time: 500_000_000_000,
                        proof_size: PROOF_SIZE,
                    },
                    None,
                    code,
                    vec![],
                    vec![salt],
                );

                let signed_extrinsic = self
                    .client
                    .tx()
                    .create_signed(&instantiate_tx, &self.signer, Default::default())
                    .await
                    .unwrap();

                let events = signed_extrinsic
                    .submit_and_watch()
                    .await
                    .unwrap()
                    .wait_for_finalized_success()
                    .await
                    .unwrap();

                let instantiated = events
                    .find_first::<Instantiated>()
                    .unwrap()
                    .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))
                    .unwrap();

                instantiated.contract
            }
        }

        #[tokio::test]
        async fn default_works() {
            let contract_id = TestCtx::new(dev::alice()).await.deploy_contract().await;

            assert_ne!(contract_id.0.len(), 0)
        }

        #[tokio::test]
        async fn default_works_2() {
            let contract_id = TestCtx::new(dev::bob()).await.deploy_contract().await;

            assert_ne!(contract_id.0.len(), 0)
        }
    }
}
