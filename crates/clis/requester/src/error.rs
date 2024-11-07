#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: utils::client::ClientError,
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
    Std {
        #[from]
        source: std::io::Error,
    },

    #[error("{source}")]
    WasmTime {
        #[from]
        source: anyhow::Error,
    },

    #[error("{source}")]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("{source}")]
    P2p {
        #[from]
        source: utils::p2p::Error,
    },

    #[error("Error: {0}")]
    Other(String),
}
