#[cfg(test)]
mod tests {

    use catalog::catalog::CatalogError;
    use utils::client::Client;

    use futures::{future::BoxFuture, FutureExt};
    use ink_env::DefaultEnvironment;
    use subxt::{utils::AccountId32, SubstrateConfig};
    use subxt_signer::sr25519::{dev::alice, Keypair};

    const FILE_PATH: &str = "../../target/ink/catalog/catalog.contract";

    type TestClient = Client<SubstrateConfig, DefaultEnvironment, Keypair>;

    async fn test_runner<'a, T>(test: T)
    where
        T: FnOnce(TestClient, AccountId32) -> BoxFuture<'a, Result<(), anyhow::Error>>,
    {
        let signer = alice();
        let client = Client::new(FILE_PATH, signer);

        let address = client.instantiate("new").await.unwrap();

        let result = test(client, address).await;

        if let Err(err) = result {
            panic!("{}", err);
        }
    }

    #[tokio::test]
    async fn foo() {
        test_runner(|client, address| {
            async move {
                let _result = client
                    .immutable_call::<Result<u32, CatalogError>>("get_worker", address, vec![])
                    .await
                    .unwrap();
                Ok(())
            }
            .boxed()
        })
        .await;

        assert!(true)
    }
}
