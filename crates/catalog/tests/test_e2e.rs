#[cfg(test)]
mod tests {

    use catalog::catalog::CatalogError;
    use ink::primitives::MessageResult;
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
        let result = client
            .immutable_call::<MessageResult<Result<u32, CatalogError>>>(
                "get_worker",
                address,
                vec![],
            )
            .await
            .unwrap();

        assert!(true)
    }
}
