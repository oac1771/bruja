#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: utils::contract_client::ClientError,
    },

    #[error("{source}")]
    Subxt {
        #[from]
        source: subxt::Error,
    },

    #[error("{source}")]
    Decode {
        #[from]
        source: codec::Error,
    },

    #[error("{source}")]
    WasmTime {
        #[from]
        source: anyhow::Error,
    },

    #[error("{source}")]
    P2p {
        #[from]
        source: utils::p2p::Error,
    },

    #[error("Error: {0}")]
    Other(String),
}
