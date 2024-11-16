#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: utils::services::contract_client::ContractClientError,
    },

    #[error("{source}")]
    SubmitJobController {
        #[from]
        source: crate::controller::requester::SubmitJobControllerError,
    },

    #[error("{source}")]
    JobService {
        #[from]
        source: utils::services::job::JobServiceError,
    },

    #[error("unable to parse contract address from provided string")]
    ParsingContractAddress,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
