#[cfg(test)]
mod tests {

    use ink_env::DefaultEnvironment;
    use subxt::SubstrateConfig;
    use subxt_signer::sr25519::{dev::alice, Keypair};
    use utils::client::Client;

    #[tokio::test]
    async fn foo() {
        let artifact_file = "foo".to_string();
        let signer = alice();
        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(artifact_file, signer);

        assert!(true)
    }
}
