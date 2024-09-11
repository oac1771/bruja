#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unexpected_cfgs)]
#[ink::contract]
pub mod catalog {

    use codec::{Decode, Encode};
    use ink::{
        env::{
            hash::{HashOutput, Keccak256},
            hash_bytes,
        },
        prelude::vec::Vec,
        storage::{traits::StorageLayout, Mapping},
    };

    type Keccak256HashOutput = <Keccak256 as HashOutput>::Type;
    type Workers = Mapping<AccountId, u32>;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CatalogError {
        WorkerNotFound,
    }

    #[derive(Debug)]
    #[ink(event)]
    pub struct WorkerRegistered {
        pub who: AccountId,
        pub val: u32,
    }

    #[derive(Debug)]
    #[ink(event)]
    pub struct JobSubmitted {
        pub who: AccountId,
        pub id: Keccak256HashOutput,
    }

    #[derive(Debug, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo), derive(StorageLayout))]
    pub struct Job {
        id: Keccak256HashOutput,
        code: Vec<u8>,
    }

    #[ink(storage)]
    pub struct Catalog {
        workers: Workers,
        jobs: Mapping<AccountId, Job>,
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
                jobs: Mapping::new(),
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
        pub fn register_worker(&mut self, val: u32) {
            let caller = self.env().caller();
            self.workers.insert(caller, &val);

            self.env().emit_event(WorkerRegistered { who: caller, val });
        }

        #[ink(message)]
        pub fn submit_job(&mut self, code: Vec<u8>) {
            let caller = self.env().caller();
            let id = self.hash(&code);
            let job = Job { id, code };

            self.jobs.insert(caller, &job);
            self.env().emit_event(JobSubmitted { who: caller, id });
        }

        fn hash(&self, data: &[u8]) -> Keccak256HashOutput {
            let mut output = Keccak256HashOutput::default();
            hash_bytes::<Keccak256>(data, &mut output);
            output
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

            catalog.register_worker(val);

            let emitted_events = recorded_events().collect::<Vec<EmittedEvent>>();
            let worker_set_event =
                <WorkerRegistered as Decode>::decode(&mut emitted_events[0].data.as_slice())
                    .unwrap();

            assert_eq!(emitted_events.len(), 1);
            assert_eq!(worker_set_event.val, val);
            assert_eq!(worker_set_event.who, who);
        }

        #[ink::test]
        fn submit_job_emits_event() {
            let who = AccountId::from([1; 32]);
            let mut catalog = Catalog::default();
            let code = vec![1, 2, 3, 4];

            catalog.submit_job(code.clone());
            let expected_hash = catalog.hash(&code);

            let emitted_events = recorded_events().collect::<Vec<EmittedEvent>>();
            let job_submitted_event =
                <JobSubmitted as Decode>::decode(&mut emitted_events[0].data.as_slice()).unwrap();

            assert_eq!(job_submitted_event.who, who);
            assert_eq!(job_submitted_event.id, expected_hash);
        }
    }
}
