#[cfg(test)]
mod tests {

    use ink_env::DefaultEnvironment;
    use subxt::SubstrateConfig;
    use subxt_signer::sr25519::{dev::alice, Keypair};
    use utils::client::Client;

    #[tokio::test]
    async fn foo() {
        let artifact_file = "../../target/ink/catalog/catalog.contract";
        let signer = alice();
        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(artifact_file, signer);

        let address = client.instantiate("new").await.unwrap();

        assert!(true)
    }
}
