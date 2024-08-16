#[cfg(test)]
mod tests {

    use catalog::catalog::CatalogError;
    use utils::client::Client;

    use anyhow::ensure;
    use futures::{future::BoxFuture, FutureExt};
    use ink_env::DefaultEnvironment;
    use subxt::{utils::AccountId32, SubstrateConfig};
    use subxt_signer::sr25519::{dev::alice, Keypair};

    use subxt::{
        backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
        tx::Signer,
        Config, OnlineClient,
    };

    const FILE_PATH: &str = "../../target/ink/catalog/catalog.contract";

    type TestClient = Client<SubstrateConfig, DefaultEnvironment, Keypair>;

    async fn test_runner<'a, T>(test: T)
    where
        T: FnOnce(TestClient, AccountId32) -> BoxFuture<'a, Result<(), anyhow::Error>>,
    {
        let signer = alice();
        let client = Client::new(FILE_PATH, signer.clone());

        let rpc = RpcClient::from_url("ws://localhost:9944").await.unwrap();
        let foo_client: OnlineClient<SubstrateConfig> =
            OnlineClient::from_rpc_client(rpc.clone()).await.unwrap();
        let rpc: LegacyRpcMethods<SubstrateConfig> = LegacyRpcMethods::new(rpc);
        let account_id: <SubstrateConfig as Config>::AccountId =
            <Keypair as Signer<SubstrateConfig>>::account_id(&signer);

        let best_block = rpc
            .chain_get_block_hash(None)
            .await
            .unwrap()
            .ok_or(subxt::Error::Other("Best block not found".into()))
            .unwrap();
        let account_nonce = foo_client
            .blocks()
            .at(best_block)
            .await
            .unwrap()
            .account_nonce(&account_id)
            .await
            .unwrap();

        println!(">>>>>>> Account Nonce: {}", account_nonce);

        let address = client.instantiate("new").await.unwrap();

        let result = test(client, address).await;

        if let Err(err) = result {
            panic!("{}", err);
        }
    }

    #[tokio::test]
    async fn get_worker_returns_error_if_no_worker_found() {
        test_runner(|client, address| {
            async move {
                let result = client
                    .immutable_call::<Result<u32, CatalogError>>("get_worker", address, vec![])
                    .await?;
                ensure!(result.is_err());
                ensure!(result.unwrap_err() == CatalogError::WorkerNotFound);

                Ok(())
            }
            .boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn set_worker_emits_event_and_sets_correctly() {
        test_runner(|client, address| {
            async move {
                let _result = client
                    .mutable_call("set_worker", address, vec!["10"])
                    .await?;

                Ok(())
            }
            .boxed()
        })
        .await;
    }
}
