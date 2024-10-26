#[cfg(test)]
mod tests {

    use catalog::catalog::{Catalog, CatalogRef, JobRequest, JobRequestSubmitted};

    use codec::Decode;
    use ink::env::DefaultEnvironment;
    use ink_e2e::{alice, events::ContractEmitted, ContractsBackend};

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

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
        let params = vec![vec![1, 2, 3, 4]];
        let resources = vec![];

        let job_request = JobRequest::new(code, params, resources);

        let mut call_builder = contract.call_builder::<Catalog>();

        let submit_job = call_builder.submit_job_request(job_request);
        let response = client.call(&alice, &submit_job).submit().await.unwrap();

        let contract_emmitted_event = response
            .events
            .find_first::<ContractEmitted<DefaultEnvironment>>()
            .unwrap()
            .unwrap();

        let job_submitted_event =
            <JobRequestSubmitted as Decode>::decode(&mut contract_emmitted_event.data.as_slice())
                .unwrap();

        assert_eq!(job_submitted_event.who, alice.public_key().0.into());

        Ok(())
    }
}
