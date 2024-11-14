pub trait SubstrateClient {
    fn get_contract_events(&self) {}
}

struct Client {}

impl SubstrateClient for Client {
    fn get_contract_events(&self) {}
}
