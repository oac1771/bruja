#[cfg(test)]
mod tests {

    use catalog::catalog::{Catalog, CatalogError, CatalogRef, JobSubmitted};

    use codec::Decode;
    use ink::env::DefaultEnvironment;
    use ink_e2e::{alice, events::ContractEmitted, ContractsBackend};

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn submit_job_emits_event<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        let mut constructor = CatalogRef::new();
        let alice = alice();
        let contract = client
            .instantiate("catalog", &alice, &mut constructor)
            .submit()
            .await
            .unwrap();
        let code = vec![1, 2, 3, 4];

        let mut call_builder = contract.call_builder::<Catalog>();

        let submit_job = call_builder.submit_job(code);
        let response = client.call(&alice, &submit_job).submit().await.unwrap();

        let contract_emmitted_event = response
            .events
            .find_first::<ContractEmitted<DefaultEnvironment>>()
            .unwrap()
            .unwrap();

        let job_submitted_event =
            <JobSubmitted as Decode>::decode(&mut contract_emmitted_event.data.as_slice()).unwrap();

        assert_eq!(job_submitted_event.who, alice.public_key().0.into());

        Ok(())
    }
}
