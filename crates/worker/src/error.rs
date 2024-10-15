#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    ContractClientError {
        #[from]
        source: utils::client::ClientError,
    },

    #[error("{source}")]
    SubxtError {
        #[from]
        source: subxt::Error,
    },

    #[error("{source}")]
    DecodeError {
        #[from]
        source: codec::Error,
    },

    // #[error("{source}")]
    // SendError {
    //     #[from]
    //     source: tokio::sync::mpsc::error::SendError<T>,
    // },
    #[error("{source}")]
    WasmTimeError {
        #[from]
        source: anyhow::Error,
    },

    #[error("{source}")]
    P2pError {
        #[from]
        source: utils::p2p::Error,
    },

    #[error("Error: {0}")]
    Other(String),
}
