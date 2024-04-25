#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod catalog {

    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Catalog {
        workers: Mapping<AccountId, u32>,
    }

    impl Catalog {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                workers: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn get_worker(&self) -> Option<u32> {
            let caller = self.env().caller();
            let result = self.workers.get(&caller);
            return result;
        }

        #[ink(message)]
        pub fn set_worker(&mut self, val: u32) -> Option<u32> {
            let caller = self.env().caller();
            let result = self.workers.insert(caller, &val);
            return result;
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

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = CatalogRef::default();

            let contract_account_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get_worker = build_message::<CatalogRef>(contract_account_id.clone())
                .call(|catalog| catalog.get_worker());
            let set_worker = build_message::<CatalogRef>(contract_account_id.clone())
                .call(|catalog| catalog.set_worker(42));

            let get_result = client
                .call_dry_run(&ink_e2e::alice(), &get_worker, 0, None)
                .await;
            let set_result = client
                .call_dry_run(&ink_e2e::alice(), &set_worker, 0, None)
                .await;

            assert_eq!(get_result.return_value(), None);
            assert_eq!(set_result.return_value(), None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let worker_value: u32 = 42;
            let constructor = CatalogRef::default();

            let contract_account_id = client
                .instantiate("contract", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<CatalogRef>(contract_account_id.clone())
                .call(|catalog| catalog.get_worker());
            let get_result = client.call(&ink_e2e::bob(), get, 0, None).await.expect("get failed");
            assert_eq!(get_result.return_value(), None);


            let set = build_message::<CatalogRef>(contract_account_id.clone())
                .call(|catalog| catalog.set_worker(worker_value));
            let _set_result = client
                .call(&ink_e2e::bob(), set, 0, None)
                .await
                .expect("set failed");

            let get = build_message::<CatalogRef>(contract_account_id.clone())
                .call(|catalog| catalog.get_worker());
            let get_result = client.call(&ink_e2e::bob(), get, 0, None).await.expect("get failed");
            assert_eq!(get_result.return_value(), Some(worker_value));

            Ok(())
        }
    }
}
