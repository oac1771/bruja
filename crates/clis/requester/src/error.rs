#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: utils::services::contract_client::ContractClientError,
    },

    #[error("{source}")]
    Client {
        #[from]
        source: utils::services::contract_client::Error,
    },

    #[error("{source}")]
    RequesterController {
        #[from]
        source: crate::controller::requester::RequesterControllerError,
    },

    #[error("{source}")]
    JobBuilderService {
        #[from]
        source: utils::services::job::job_builder::JobBuilderServiceError,
    },

    #[error("")]
    NetworkClient {
        #[from]
        source: utils::services::p2p::NetworkClientError,
    },

    #[error("unable to parse contract address from provided string")]
    ParsingContractAddress,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
