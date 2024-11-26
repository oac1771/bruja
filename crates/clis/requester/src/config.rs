use std::{env::var, str::FromStr};
use subxt_signer::{sr25519::Keypair, SecretUri};

#[derive(Clone)]
pub struct Config {
    pub signer: Keypair,
    pub artifact_file_path: String,
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        let suri = var("SURI").unwrap_or("//Alice".to_string());
        let artifact_file_path = var("ARTIFACT_FILE_PATH")
            .unwrap_or("./target/ink/catalog/catalog.contract".to_string());

        let signer = Keypair::from_uri(&SecretUri::from_str(&suri).unwrap()).unwrap();
        let url = var("URL").unwrap_or("ws://127.0.0.1:9944".to_string());

        Self {
            signer,
            artifact_file_path,
            url,
        }
    }
}
