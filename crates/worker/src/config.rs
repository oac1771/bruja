use std::{env::var, str::FromStr};
use subxt::utils::AccountId32;
use subxt_signer::{sr25519::Keypair, SecretUri};

pub struct Config {
    pub signer: Keypair,
    pub artifact_file_path: String,
    pub contract_address: AccountId32,
}

impl Config {
    pub fn new() -> Self {
        let suri = var("SURI").unwrap_or(String::from("//Alice"));
        let artifact_file_path = var("ARTIFACT_FILE_PATH")
            .unwrap_or(String::from("./target/ink/catalog/catalog.contract"));
        let contract_address = AccountId32::from_str(&var("ADDRESS").unwrap()).unwrap();

        let signer = Keypair::from_uri(&SecretUri::from_str(&suri).unwrap()).unwrap();

        Self {
            signer,
            artifact_file_path,
            contract_address,
        }
    }
}
