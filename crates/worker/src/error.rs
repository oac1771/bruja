use subxt::Error as SubxtError;
use utils::client::ClientError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Contract Client Returned Error: {source}")]
    ContractClientError {
        #[from]
        source: ClientError,
    },

    #[error("Subxt client Error: {source}")]
    SubxtError {
        #[from]
        source: SubxtError,
    },

    #[error("Codec Decode Error: {source}")]
    DecodeError {
        #[from]
        source: codec::Error,
    },

    #[error("WasmTimeError: {source}")]
    WasmTimeError {
        #[from]
        source: anyhow::Error,
    },

    #[error("Error: {0}")]
    Other(String),
}
