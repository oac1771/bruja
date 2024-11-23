#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    WorkerController {
        #[from]
        source: crate::controller::worker::WorkerControllerError,
    },

    #[error("{source}")]
    Client {
        #[from]
        source: utils::services::contract_client::Error,
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

    #[error("")]
    WorkerStoppedUnexpectedly,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
