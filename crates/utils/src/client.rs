use std::{fmt::Display, marker::PhantomData};

use contract_extrinsics::{
    ErrorVariant, ExtrinsicOptsBuilder, InstantiateCommandBuilder, InstantiateExec,
};
use ink_env::Environment;
use serde::Serialize;
use sp_core::{Bytes, Decode};
use subxt::{
    config::{Config, DefaultExtrinsicParams, ExtrinsicParams},
    ext::{scale_decode::IntoVisitor, scale_encode::EncodeAsType},
    tx::Signer,
};

pub struct Client<C, E, S> {
    artifact_file: String,
    signer: S,
    _config: PhantomData<C>,
    _env: PhantomData<E>,
}

impl<C: Config, E: Environment, S: Signer<C> + Clone> Client<C, E, S>
where
    C::Hash: From<[u8; 32]> + EncodeAsType + IntoVisitor,
    C::AccountId: Display + IntoVisitor + Decode,
    <<C as Config>::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    E::Balance: Default + EncodeAsType + Serialize,
{
    pub fn new(artifact_file: String, signer: S) -> Self {
        Self {
            artifact_file,
            signer: signer,
            _config: PhantomData::default(),
            _env: PhantomData::default(),
        }
    }

    pub async fn instantiate(
        &self,
        constructor: &str,
    ) -> Result<<C as Config>::AccountId, ClientError> {
        let extrinsic_opts = self.extrinsic_opts_builder().done();
        let salt: Bytes = rand::random::<[u8; 8]>().to_vec().into();

        let instantiate_exec: InstantiateExec<C, E, S> =
            InstantiateCommandBuilder::new(extrinsic_opts)
                .constructor(constructor)
                .salt(Some(salt))
                .done()
                .await?;

        let address = instantiate_exec.instantiate(None).await?.contract_address;

        return Ok(address);
    }

    fn extrinsic_opts_builder(&self) -> ExtrinsicOptsBuilder<C, E, S> {
        ExtrinsicOptsBuilder::new(self.signer.clone()).file(Some(self.artifact_file.clone()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Instantiation Command Builder Error: {source}")]
    InstantiateCommandBuilderError {
        #[from]
        source: anyhow::Error,
    },

    #[error("Instantiation Error: {error}")]
    InstantiateError { error: String },
}

impl From<ErrorVariant> for ClientError {
    fn from(value: ErrorVariant) -> Self {
        let error = match value {
            ErrorVariant::Generic(err) => {
                if let Ok(val) = serde_json::to_string(&err) {
                    val
                } else {
                    "Error serializing GenericError to string".to_string()
                }
            }
            ErrorVariant::Module(err) => err.error,
        };
        ClientError::InstantiateError { error }
    }
}
