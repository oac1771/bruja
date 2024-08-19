use utils::client::ClientError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Contract Client Returned Error: {source}")]
    ContractClientError {
        #[from]
        source: ClientError,
    },
}
