use anyhow::Context;
use std::{fmt::Display, marker::PhantomData};

use contract_extrinsics::{
    CallCommandBuilder, CallExec, ErrorVariant, ExtrinsicOptsBuilder, InstantiateCommandBuilder,
    InstantiateExec,
};
use contract_transcode::Value;
use ink_env::Environment;
use serde::Serialize;
use sp_core::{Bytes, Decode};
use sp_runtime::DispatchError;
use subxt::{
    blocks::ExtrinsicEvents,
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
    C::AccountId: Display + IntoVisitor + Decode + EncodeAsType,
    <<C as Config>::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    E::Balance: Default + EncodeAsType + Serialize,
{
    pub fn new(artifact_file: &str, signer: S) -> Self {
        Self {
            artifact_file: artifact_file.to_string(),
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
                .await
                .context("Failed at InstantiateCommandBuilder::done()")?;

        let address = instantiate_exec.instantiate(None).await?.contract_address;

        return Ok(address);
    }

    async fn _immutable_call(
        &self,
        message: &str,
        address: <C as Config>::AccountId,
        args: Vec<String>,
    ) -> Result<Value, ClientError> {
        let extrinsic_opts = self.extrinsic_opts_builder().done();
        let call_exec: CallExec<C, E, S> =
            CallCommandBuilder::new(address, &message, extrinsic_opts)
                .args(args)
                .done()
                .await
                .context("Failed at CallCommandBuilder::done()")?;

        if self._is_mutable(&call_exec, message)? {
            return Err(ClientError::MessageMutabilityError {
                message: format!("{} is not immutable message", message),
            });
        }

        let call_result = call_exec
            .call_dry_run()
            .await
            .context("Failed at CallExec::call_dry_run(")?
            .result?
            .data;

        let value = call_exec
            .transcoder()
            .decode_message_return(call_exec.message(), &mut &call_result[..])
            .context("Failed at CallExec::decode_message_return()")?;

        Ok(value)
    }

    async fn _mutable_call(
        &self,
        message: &str,
        address: <C as Config>::AccountId,
        args: Vec<String>,
    ) -> Result<ExtrinsicEvents<C>, ClientError> {
        let extrinsic_opts = self.extrinsic_opts_builder().done();
        let call_exec: CallExec<C, E, S> =
            CallCommandBuilder::new(address, &message, extrinsic_opts)
                .args(args)
                .done()
                .await
                .context("Failed at CallCommandBuilder::done()")?;

        if !self._is_mutable(&call_exec, message)? {
            return Err(ClientError::MessageMutabilityError {
                message: format!("{} is not mutable message", message),
            });
        }

        let events = call_exec.call(None).await?;

        Ok(events)
    }

    fn extrinsic_opts_builder(&self) -> ExtrinsicOptsBuilder<C, E, S> {
        ExtrinsicOptsBuilder::new(self.signer.clone()).file(Some(self.artifact_file.clone()))
    }

    fn _is_mutable(
        &self,
        call_exec: &CallExec<C, E, S>,
        message: &str,
    ) -> Result<bool, ClientError> {
        let result = call_exec
            .transcoder()
            .metadata()
            .spec()
            .messages()
            .iter()
            .find(|msg| msg.label() == message)
            .ok_or_else(|| ClientError::MessageNotFound {
                message: message.to_string(),
            })?
            .mutates();

        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Instantiation Command Builder Error: {source}")]
    ContractExtrinsicCrateError {
        #[from]
        source: anyhow::Error,
    },

    #[error("Contract Dispatch Error: {error}")]
    ContractDispatchError { error: String },

    #[error("Message Not Found: {message}")]
    MessageNotFound { message: String },

    #[error("Unexpected Message Mutability State: {message}")]
    MessageMutabilityError { message: String },

    #[error("CallExec Error: {error}")]
    CallExecError { error: String },
}

impl From<DispatchError> for ClientError {
    fn from(value: DispatchError) -> Self {
        let error = match serde_json::to_string(&value) {
            Ok(val) => val,
            Err(err) => format!("Error Serializing DispatchError: {}", err),
        };
        ClientError::ContractDispatchError { error }
    }
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
        ClientError::CallExecError { error }
    }
}
