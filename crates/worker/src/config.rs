use std::{env::var, str::FromStr};
use subxt_signer::{sr25519::Keypair, SecretUri};

pub struct Config {
    pub signer: Keypair,
    pub artifact_file_path: String,
}

impl Config {
    pub fn new(suri: &str, artifact_file_path: String) -> Self {

        let signer = Keypair::from_uri(&SecretUri::from_str(&suri).unwrap()).unwrap();

        Self {
            signer,
            artifact_file_path,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let suri = var("SURI").unwrap_or(String::from("//Alice"));
        let artifact_file_path = var("ARTIFACT_FILE_PATH")
            .unwrap_or(String::from("./target/ink/catalog/catalog.contract"));

        let signer = Keypair::from_uri(&SecretUri::from_str(&suri).unwrap()).unwrap();

        Self {
            signer,
            artifact_file_path,
        }
    }
}