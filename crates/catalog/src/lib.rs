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
        storage::Mapping,
    };

    pub type HashId = <Keccak256 as HashOutput>::Type;
    type Workers = Mapping<AccountId, u32>;
    type Requests = Mapping<AccountId, Vec<HashId>>;

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
    pub struct JobRequestSubmitted {
        pub who: AccountId,
        pub id: HashId,
    }

    #[derive(Debug, Encode, Decode, PartialEq, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct JobRequest {
        id: HashId,
    }

    impl JobRequest {
        pub fn new(code: &[u8], params: &Vec<Vec<u8>>) -> Self {
            code.encode_to(&mut params.encode());
            let id = hash(code);

            Self { id }
        }

        pub fn id(&self) -> HashId {
            self.id
        }
    }

    #[ink(storage)]
    pub struct Catalog {
        workers: Workers,
        requests: Requests,
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
                requests: Mapping::new(),
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
        pub fn submit_job_request(&mut self, job_request: JobRequest) {
            let who = self.env().caller();
            let id = job_request.id();

            let ids = if let Some(mut ids) = self.requests.get(who) {
                ids.push(id);
                ids
            } else {
                vec![id]
            };

            self.requests.insert(who, &ids);
            self.env().emit_event(JobRequestSubmitted { who, id });
        }
    }

    fn hash(data: &[u8]) -> HashId {
        let mut output = HashId::default();
        hash_bytes::<Keccak256>(data, &mut output);
        output
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::{
            env::test::{recorded_events, EmittedEvent},
            primitives::AccountId,
            scale::Decode,
        };

        impl JobRequest {
            fn test(code: Vec<u8>) -> Self {
                let params: Vec<Vec<u8>> = vec![];

                Self::new(code.as_slice(), &params)
            }
        }

        #[ink::test]
        fn register_worker_emits_event() {
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
        fn submit_job_request_emits_event() {
            let who = AccountId::from([1; 32]);
            let mut catalog = Catalog::default();
            let code = vec![1, 2, 3, 4];

            let job_request = JobRequest::test(code.clone());

            catalog.submit_job_request(job_request.clone());

            let emitted_events = recorded_events().collect::<Vec<EmittedEvent>>();
            let job_submitted_event =
                <JobRequestSubmitted as Decode>::decode(&mut emitted_events[0].data.as_slice())
                    .unwrap();
            let job_ids = catalog.requests.get(who).unwrap();

            assert_eq!(job_submitted_event.who, who);
            assert_eq!(job_submitted_event.id, job_request.id());
            assert_eq!(job_ids[0], job_request.id());
        }

        #[ink::test]
        fn submit_job_request_appends_to_existing_jobs() {
            let who = AccountId::from([1; 32]);
            let mut catalog = Catalog::default();

            let code_1 = vec![1, 2, 3, 4];
            let code_2 = vec![1, 2, 3, 5];

            let job_1_request = JobRequest::test(code_1.clone());
            let job_2_request = JobRequest::test(code_2.clone());

            catalog.submit_job_request(job_1_request.clone());
            catalog.submit_job_request(job_2_request.clone());

            let emitted_events = recorded_events().collect::<Vec<EmittedEvent>>();

            let job_submitted_event_1 =
                <JobRequestSubmitted as Decode>::decode(&mut emitted_events[0].data.as_slice())
                    .unwrap();
            let job_submitted_event_2 =
                <JobRequestSubmitted as Decode>::decode(&mut emitted_events[1].data.as_slice())
                    .unwrap();
            let jobs = catalog.requests.get(who).unwrap();

            assert_eq!(job_submitted_event_1.id, job_1_request.id());
            assert_eq!(job_submitted_event_2.id, job_2_request.id());

            assert_eq!(jobs.len(), 2);
            assert_eq!(jobs[0], job_1_request.id());
            assert_eq!(jobs[1], job_2_request.id());
        }
    }
}
