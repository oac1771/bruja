#[cfg(test)]
mod tests {

    use catalog::catalog::CatalogRef;

    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        let mut constructor = CatalogRef::new();
        let contract_acc_id = client
            .instantiate("catalog", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .unwrap()
            .account_id;
        println!(">> {:?}", contract_acc_id);
        Ok(())
    }
}
