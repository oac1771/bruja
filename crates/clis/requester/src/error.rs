#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Contract Client Returned Error: {source}")]
    ContractClientError {
        #[from]
        source: utils::client::ClientError,
    },

    #[error("Subxt client Error: {source}")]
    SubxtError {
        #[from]
        source: subxt::Error,
    },

    #[error("Codec Decode Error: {source}")]
    DecodeError {
        #[from]
        source: codec::Error,
    },

    #[error("Std Error: {source}")]
    StdError {
        #[from]
        source: std::io::Error,
    },

    #[error("WasmTimeError: {source}")]
    WasmTimeError {
        #[from]
        source: anyhow::Error,
    },

    #[error("Parse Error: {source}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("P2P Error: {source}")]
    P2pError {
        #[from]
        source: utils::p2p::Error,
    },

    #[error("Error: {0}")]
    Other(String),
}
