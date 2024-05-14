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
        use super::*;
        use subxt::{OnlineClient, SubstrateConfig};

        #[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
        pub mod chain {}

        #[tokio::test]
        async fn default_works() {
            let _foo = chain::tx();

            let bar = CatalogRef::new();

            let _client = OnlineClient::<SubstrateConfig>::new().await.unwrap();

            assert!(false);
        }
    }
}
