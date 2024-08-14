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

    #[derive(Debug)]
    #[ink(event)]
    pub struct WorkerSet {
        who: AccountId,
        val: u32,
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
        pub fn set_worker(&mut self, val: u32) {
            let caller = self.env().caller();
            self.workers.insert(caller, &val);

            self.env().emit_event(WorkerSet { who: caller, val });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::{
            env::test::{recorded_events, EmittedEvent},
            primitives::AccountId,
            scale::Decode,
        };

        #[ink::test]
        fn default_works() {
            let catalog = Catalog::default();
            assert_eq!(catalog.get_worker(), Err(CatalogError::WorkerNotFound));
        }

        #[ink::test]
        fn set_worker_emits_event() {
            let val = 10;
            let who = AccountId::from([1; 32]);
            let mut catalog = Catalog::default();

            catalog.set_worker(val);

            let emitted_events = recorded_events().collect::<Vec<EmittedEvent>>();
            let worker_set_event =
                <WorkerSet as Decode>::decode(&mut emitted_events[0].data.as_slice()).unwrap();

            assert_eq!(emitted_events.len(), 1);
            assert_eq!(worker_set_event.val, val);
            assert_eq!(worker_set_event.who, who);
        }
    }
}
