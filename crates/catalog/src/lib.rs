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

            // add event emission

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
}
