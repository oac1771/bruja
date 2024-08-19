#[cfg(test)]
mod tests {

    use catalog::catalog::{Catalog, CatalogError, CatalogRef};

    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn get_worker_returns_err<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        let mut constructor = CatalogRef::new();
        let contract = client
            .instantiate("catalog", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .unwrap();

        let call_builder = contract.call_builder::<Catalog>();

        let get_worker = call_builder.get_worker();
        let get_res = client.call(&ink_e2e::bob(), &get_worker).dry_run().await?;
        let return_value = get_res.return_value().err().unwrap();

        assert_eq!(return_value, CatalogError::WorkerNotFound);

        Ok(())
    }
}
