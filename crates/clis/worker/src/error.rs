#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: utils::contract_client::ContractClientError,
    },

    #[error("unable to parse contract address from provided string")]
    ParsingContractAddress,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
