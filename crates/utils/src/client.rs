use crate::chain::contracts::events::ContractEmitted;
use anyhow::Context;
use codec::Encode;
use std::{fmt::Display, marker::PhantomData};

use pallet_contracts::ContractAccessError;

use contract_extrinsics::{
    CallCommandBuilder, CallExec, ErrorVariant, ExtrinsicOptsBuilder, InstantiateCommandBuilder,
    InstantiateExec,
};
use ink::primitives::{LangError, MessageResult};
use ink::env::Environment;
use ink_metadata::layout::Layout;
use serde::Serialize;
use sp_core::{Bytes, Decode};
use sp_runtime::DispatchError;
use subxt::{
    config::{Config, DefaultExtrinsicParams, ExtrinsicParams},
    ext::{scale_decode::IntoVisitor, scale_encode::EncodeAsType},
    tx::Signer,
    SubstrateConfig, backend::{legacy::LegacyBackend, rpc::RpcClient, Backend}
};

pub struct Client<'a, C, E, S> {
    artifact_file: &'a str,
    signer: &'a S,
    _config: PhantomData<C>,
    _env: PhantomData<E>,
}

// reformat this so it takes suri and builds signer in ::new() method
// add method to return keypair type
impl<'a, C: Config, E: Environment, S: Signer<C> + Clone> Client<'a, C, E, S>
where
    C::Hash: From<[u8; 32]> + EncodeAsType + IntoVisitor,
    C::AccountId: Display + IntoVisitor + Decode + EncodeAsType,
    <<C as Config>::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    E::Balance: Default + EncodeAsType + Serialize,
{
    pub fn new(artifact_file: &'a str, signer: &'a S) -> Self {
        Self {
            artifact_file,
            signer,
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

    pub async fn immutable_call<T: Decode>(
        &self,
        message: &str,
        address: <C as Config>::AccountId,
        args: Vec<String>,
    ) -> Result<T, ClientError> {
        let extrinsic_opts = self.extrinsic_opts_builder().done();
        let call_exec: CallExec<C, E, S> =
            CallCommandBuilder::new(address, &message, extrinsic_opts)
                .args(args)
                .done()
                .await
                .context("Failed at CallCommandBuilder::done()")?;

        if self.is_mutable(&call_exec, message)? {
            return Err(ClientError::MessageMutabilityError {
                message: format!("{} is not immutable message", message),
            });
        }

        let data = call_exec
            .call_dry_run()
            .await
            .context("Failed at CallExec::call_dry_run(")?
            .result?
            .data;

        let result = <MessageResult<T>>::decode(&mut data.as_slice())??;

        Ok(result)
    }

    pub async fn mutable_call<Ev: Decode>(
        &self,
        message: &str,
        address: <C as Config>::AccountId,
        args: Vec<&str>,
    ) -> Result<Ev, ClientError> {
        let extrinsic_opts = self.extrinsic_opts_builder().done();
        let call_exec: CallExec<C, E, S> =
            CallCommandBuilder::new(address, &message, extrinsic_opts)
                .args(args)
                .done()
                .await
                .context("Failed at CallCommandBuilder::done()")?;

        if !self.is_mutable(&call_exec, message)? {
            return Err(ClientError::MessageMutabilityError {
                message: format!("{} is not mutable message", message),
            });
        }

        let events = call_exec.call(None).await?;
        match events.find_first::<ContractEmitted>()? {
            Some(event) => {
                let result = <Ev as Decode>::decode(&mut event.data.as_slice())?;
                Ok(result)
            }
            None => Err(ClientError::ContractEmittedError),
        }
    }

    pub async fn get_storage<D: Decode>(&self, address: <C as Config>::AccountId) -> Result<D, ClientError> {
        let contract_message_transcoder = self
            .extrinsic_opts_builder()
            .done()
            .contract_artifacts()?
            .contract_transcoder()?;

        let mut jobs_key: u32 = 0;

        if let Layout::Root(root) = contract_message_transcoder.metadata().layout() {
            if let Layout::Struct(struct_layout) = root.layout() {
                for field in struct_layout.fields() {
                    if field.name() == "jobs" {
                        if let Layout::Root(root) = field.layout() {
                            jobs_key = *root.root_key().key();
                        }
                    }
                }
            }
        }

        let storage_key = (jobs_key, self.signer.account_id()).encode();
        let args = (address, storage_key.clone()).encode();

        let client = RpcClient::from_insecure_url("ws://127.0.0.1:9944")
            .await
            .unwrap();
        let backend: LegacyBackend<SubstrateConfig> = LegacyBackend::builder().build(client);

        let latest_block = backend.latest_finalized_block_ref().await.unwrap();

        let storage_data = backend
            .call("ContractsApi_get_storage", Some(&args), latest_block.hash())
            .await
            .unwrap();

        let result: Result<Option<Vec<u8>>, ContractAccessError> =
            Decode::decode(&mut storage_data.as_slice()).unwrap();
        let raw_bytes = result.unwrap().unwrap();
        let data = D::decode(&mut raw_bytes.as_slice()).unwrap();

        Ok(data)
    }

    pub fn extrinsic_opts_builder(&self) -> ExtrinsicOptsBuilder<C, E, S> {
        ExtrinsicOptsBuilder::new(self.signer.clone()).file(Some(self.artifact_file))
    }

    fn is_mutable(
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
    #[error("Contract Extrinsic Crate Error: {source}")]
    ContractExtrinsicCrateError {
        #[from]
        source: anyhow::Error,
    },

    #[error("Codec Decode Error: {source}")]
    DecodeError {
        #[from]
        source: codec::Error,
    },

    #[error("Subxt Crate Error: {source}")]
    SubxtError {
        #[from]
        source: subxt::Error,
    },

    #[error("Contract Dispatch Error: {error}")]
    ContractDispatchError { error: String },

    #[error("Message Not Found: {message}")]
    MessageNotFound { message: String },

    #[error("Unexpected Message Mutability State: {message}")]
    MessageMutabilityError { message: String },

    #[error("Codec Decode Error: {message}")]
    MessageError { message: String },

    #[error("CallExec Error: {error}")]
    CallExecError { error: String },

    #[error("ContractEmitted event not found")]
    ContractEmittedError,
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
        ClientError::CallExecError {
            error: format!("Error Executing Call: {}", error),
        }
    }
}

impl From<LangError> for ClientError {
    fn from(_value: LangError) -> Self {
        Self::MessageError {
            message: "Failed to read execution input for the dispatchable.".to_string(),
        }
    }
}
