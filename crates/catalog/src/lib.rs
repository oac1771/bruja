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
        prelude::{vec, vec::Vec},
        storage::{traits::StorageLayout, Mapping},
    };

    pub type Keccak256HashOutput = <Keccak256 as HashOutput>::Type;
    type Workers = Mapping<AccountId, u32>;
    type Jobs = Mapping<AccountId, Vec<Keccak256HashOutput>>;
    type Work = Mapping<Keccak256HashOutput, Vec<u8>>;

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
        code: Vec<u8>,
        id: Keccak256HashOutput,
    }

    #[ink(storage)]
    pub struct Catalog {
        workers: Workers,
        jobs: Jobs,
        work: Work
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
                work: Mapping::new()
            }
        }

        #[ink(message)]
        pub fn get_worker(&self) -> u32 {
            let caller = self.env().caller();

            self.workers.get(caller).unwrap_or(0)
        }

        #[ink(message)]
        pub fn register_worker(&mut self, val: u32) {
            let caller = self.env().caller();
            self.workers.insert(caller, &val);

            self.env().emit_event(WorkerRegistered { who: caller, val });
        }

        #[ink(message)]
        pub fn submit_job(&mut self, code: Vec<u8>) {
            let who = self.env().caller();
            let id = self.hash(&code);

            let ids = if let Some(mut ids) = self.jobs.get(who) {
                ids.push(id);
                ids
            } else {
                vec![id]
            };

            self.jobs.insert(who, &ids);
            self.work.insert(id, &code);
            self.env().emit_event(JobSubmitted { who, id });
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
            let jobs = catalog.jobs.get(who).unwrap();

            assert_eq!(job_submitted_event.who, who);
            assert_eq!(job_submitted_event.id, expected_hash);
            assert_eq!(jobs.len(), 1);
        }

        #[ink::test]
        fn submit_job_appends_to_existing_jobs() {
            let who = AccountId::from([1; 32]);
            let mut catalog = Catalog::default();
            let code_1 = vec![1, 2, 3, 4];
            let code_2 = vec![1, 2, 3, 5];

            catalog.submit_job(code_1.clone());
            catalog.submit_job(code_2.clone());

            let expected_hash_1 = catalog.hash(&code_1);
            let expected_hash_2 = catalog.hash(&code_2);

            let emitted_events = recorded_events().collect::<Vec<EmittedEvent>>();

            let job_submitted_event_1 =
                <JobSubmitted as Decode>::decode(&mut emitted_events[0].data.as_slice()).unwrap();
            let job_submitted_event_2 =
                <JobSubmitted as Decode>::decode(&mut emitted_events[1].data.as_slice()).unwrap();
            let jobs = catalog.jobs.get(who).unwrap();

            assert_eq!(job_submitted_event_1.id, expected_hash_1);
            assert_eq!(job_submitted_event_2.id, expected_hash_2);

            assert_eq!(jobs.len(), 2);
            assert_eq!(jobs[0], expected_hash_1);
            assert_eq!(jobs[1], expected_hash_2);
        }
    }
}
