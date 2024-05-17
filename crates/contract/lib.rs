#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod catalog {

    use ink::storage::Mapping;

    #[derive(Default)]
    #[ink(storage)]
    pub struct Catalog {
        workers: Mapping<AccountId, u32>,
    }

    impl Catalog {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn get_worker(&self) -> Option<u32> {
            let caller = self.env().caller();
            self.workers.get(caller)
        }

        #[ink(message)]
        pub fn set_worker(&mut self, val: u32) -> Option<u32> {
            let caller = self.env().caller();
            self.workers.insert(caller, &val)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let mut catalog = Catalog::default();
            assert_eq!(catalog.get_worker(), None);
            assert_eq!(catalog.set_worker(10), None);
        }
    }

    // #[cfg(all(test, feature = "e2e-tests"))]
    #[cfg(test)]
    mod e2e_tests {
        // use super::*;
        use subxt::{utils::{AccountId32, MultiAddress}, Error, OnlineClient, SubstrateConfig};
        use subxt_signer::sr25519;
        use rand::Rng;

        #[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
        pub mod chain {}

        use chain::{contracts::events::Instantiated, runtime_types::sp_weights::weight_v2::Weight};

        const PROOF_SIZE: u64 = u64::MAX / 2;

        const CONTRACT: &str = r#"
            (module
                (import "env" "memory" (memory 1 1))
                (func (export "deploy"))
                (func (export "call"))
            )
        "#;

        async fn deploy_contract() -> AccountId32 {
            let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");
            let alice = sr25519::dev::alice();
            let salt: u8 = rand::thread_rng().gen();
        
            let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();
        
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
        
            let signed_extrinsic = client
                .tx()
                .create_signed(&instantiate_tx, &alice, Default::default())
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

        #[tokio::test]
        async fn default_works() {
            let contract_id = deploy_contract().await;

            assert_ne!(contract_id.0.len(), 0)
        }
    }
}
