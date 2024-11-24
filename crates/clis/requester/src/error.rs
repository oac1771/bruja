#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    JobHandler {
        #[from]
        source: utils::services::job::job_handler::Error,
    },

    #[error("")]
    Network {
        #[from]
        source: utils::services::p2p::NetworkError,
    },

    #[error("")]
    Join {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("unable to parse contract address from provided string")]
    ParsingContractAddress,

    #[error("")]
    NetworkHandlerStopped,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
