use crate::{
    chain::{
        self,
        contracts::events::{ContractEmitted, Instantiated},
        runtime_types::sp_weights::weight_v2::Weight,
    },
    ink_project::{InkProject, InkProjectError},
};
use codec::{Decode, Encode};
use std::{fmt::Display, fs::File, io::BufReader, marker::PhantomData};

use pallet_contracts::{Code, ContractAccessError, ContractExecResult, ContractInstantiateResult};

use ink::{
    env::Environment,
    primitives::{LangError, MessageResult},
};
use serde::Serialize;
use subxt::{
    backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
    blocks::ExtrinsicEvents,
    config::{Config, DefaultExtrinsicParams, ExtrinsicParams},
    ext::{scale_decode::IntoVisitor, scale_encode::EncodeAsType},
    tx::{Payload, Signer, TxPayload},
    utils::{AccountId32, MultiAddress},
    OnlineClient,
};

pub struct Client<'a, C, E, S> {
    ink_project: InkProject,
    signer: &'a S,
    rpc_client: RpcClient,
    _config: PhantomData<C>,
    _env: PhantomData<E>,
}

impl<'a, C: Config, E: Environment, S: Signer<C> + Clone> Client<'a, C, E, S>
where
    C::Hash: From<[u8; 32]> + EncodeAsType + IntoVisitor,
    C::AccountId:
        Display + IntoVisitor + Decode + EncodeAsType + Into<MultiAddress<AccountId32, ()>>,
    <<C as Config>::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params> + Default,
    E::Balance: Default + EncodeAsType + Serialize + From<u128>,
{
    pub async fn new(artifact_file: &'a str, signer: &'a S) -> Result<Self, ClientError> {
        let file = File::open(artifact_file)?;
        let reader = BufReader::new(file);
        let ink_project: InkProject = serde_json::from_reader(reader)?;

        let rpc_client = RpcClient::from_insecure_url("ws://127.0.0.1:9944").await?;

        Ok(Self {
            ink_project,
            signer,
            rpc_client,
            _config: PhantomData::default(),
            _env: PhantomData::default(),
        })
    }

    pub async fn instantiate(&self, constructor: &str) -> Result<AccountId32, ClientError> {
        let salt = rand::random::<[u8; 8]>().to_vec();
        let code = self.ink_project.code()?;

        let data = self
            .ink_project
            .get_constructor(constructor)?
            .get_selector()?;

        let gas_limit = self
            .estimate_gas_instantiate(
                self.signer.account_id(),
                0_u128.into(),
                code.clone(),
                data.clone(),
                salt.clone(),
            )
            .await?;

        let instantiate_tx = chain::tx()
            .contracts()
            .instantiate_with_code(0, gas_limit, None, code, data, salt);

        let events = self.submit_extrinsic(instantiate_tx).await?;

        let instantiated = events
            .find_first::<Instantiated>()?
            .ok_or_else(|| ClientError::EventNotFound)?;

        Ok(instantiated.contract)
    }

    pub async fn write<Ev: Decode, Args: Encode + Clone>(
        &self,
        address: <C as Config>::AccountId,
        message: &str,
        args: Args,
    ) -> Result<Ev, ClientError> {
        let message = self.ink_project.get_message(message)?;
        let mut data = message.get_selector()?;

        let gas_limit = self
            .call(address.clone(), message.get_label(), args.clone())
            .await?
            .gas_required;

        args.encode_to(&mut data);

        let call_tx = chain::tx()
            .contracts()
            .call(address.into(), 0, gas_limit.into(), None, data);

        let events = self.submit_extrinsic(call_tx).await?;

        let contract_emitted = events
            .find_first::<ContractEmitted>()?
            .ok_or_else(|| ClientError::EventNotFound)?;

        let result = <Ev as Decode>::decode(&mut contract_emitted.data.as_slice())?;

        Ok(result)
    }

    pub async fn read<D: Decode, Args: Encode + Clone>(
        &self,
        address: <C as Config>::AccountId,
        message: &str,
        args: Args,
    ) -> Result<D, ClientError> {
        let exec_return = self.call(address, message, args).await?.result?;

        let result = <MessageResult<D>>::decode(&mut exec_return.data.as_slice())??;

        Ok(result)
    }

    pub async fn read_storage<D: Decode>(
        &self,
        contract_address: C::AccountId,
        field_name: &str,
        key: &[u8],
    ) -> Result<D, ClientError> {
        let field = self.ink_project.get_storage_field(field_name)?;
        let mut field_key = field.get_storage_key()?;

        field_key.append(&mut key.to_vec());

        let params = (contract_address, field_key).encode();

        let raw_bytes = self
            .call_runtime_api::<Result<Option<Vec<u8>>, ContractAccessError>>(
                "ContractsApi_get_storage",
                Some(&params),
                None,
            )
            .await??
            .ok_or_else(|| ClientError::StorageEntryIsEmpty)?;

        let data = D::decode(&mut raw_bytes.as_slice())?;

        Ok(data)
    }

    pub async fn online_client(&self) -> Result<OnlineClient<C>, ClientError> {
        let client = OnlineClient::<C>::from_rpc_client(self.rpc_client.clone()).await?;

        Ok(client)
    }

    async fn estimate_gas_instantiate(
        &self,
        origin: C::AccountId,
        value: E::Balance,
        code: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> Result<Weight, ClientError> {
        let instantiate_call_data: Instantiate<C::AccountId, E::Balance, C::Hash> =
            Instantiate::new(origin, value, code, data.clone(), salt);

        let result = self
            .call_runtime_api::<ContractInstantiateResult<C::AccountId, E::Balance, ()>>(
                "ContractsApi_instantiate",
                Some(&instantiate_call_data.encode()),
                None,
            )
            .await?;

        let gas_consumed = result.gas_required;

        Ok(gas_consumed.into())
    }

    async fn call<Args: Encode + Clone>(
        &self,
        address: <C as Config>::AccountId,
        message: &str,
        args: Args,
    ) -> Result<ContractExecResult<E::Balance, ()>, ClientError> {
        let message = self.ink_project.get_message(message)?;

        let mut input_data = message.get_selector()?;

        args.encode_to(&mut input_data);

        let params = Call::new(
            self.signer.account_id(),
            address,
            0_u128,
            None,
            None,
            input_data,
        )
        .encode();

        let contract_result = self
            .call_runtime_api::<ContractExecResult<E::Balance, ()>>(
                "ContractsApi_call",
                Some(&params),
                None,
            )
            .await?;

        Ok(contract_result)
    }

    async fn call_runtime_api<R: Decode>(
        &self,
        function: &str,
        call_parameters: Option<&[u8]>,
        at: Option<C::Hash>,
    ) -> Result<R, ClientError> {
        let rpc_client: LegacyRpcMethods<C> = LegacyRpcMethods::new(self.rpc_client.clone());
        let response = rpc_client.state_call(function, call_parameters, at).await?;

        let result = R::decode(&mut response.as_slice())?;

        Ok(result)
    }

    async fn submit_extrinsic<Tx>(
        &self,
        tx_payload: Payload<Tx>,
    ) -> Result<ExtrinsicEvents<C>, ClientError>
    where
        Payload<Tx>: TxPayload,
    {
        let client = OnlineClient::<C>::from_rpc_client(self.rpc_client.clone()).await?;

        let signed_extrinsic = client
            .tx()
            .create_signed(&tx_payload, self.signer, Default::default())
            .await?;

        let events = signed_extrinsic
            .submit_and_watch()
            .await?
            .wait_for_finalized_success()
            .await?;

        Ok(events)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Codec Decode Error: {source}")]
    Decode {
        #[from]
        source: codec::Error,
    },

    #[error("Subxt Crate Error: {source}")]
    Subxt {
        #[from]
        source: subxt::Error,
    },

    #[error("Io error: {source}")]
    StdIo {
        #[from]
        source: std::io::Error,
    },

    #[error("SerdeJson error: {source}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },

    #[error("Ink project error: {source}")]
    InkProject {
        #[from]
        source: InkProjectError,
    },

    #[error("Sp runtime dispatch error {error}")]
    SpRuntime { error: String },

    #[error("Contract access Error {error}")]
    ContractAccess { error: String },

    #[error("Contract Dispatch Error: {error}")]
    ContractDispatch { error: String },

    #[error("Message Not Found: {message}")]
    MessageNotFound { message: String },

    #[error("Unexpected Message Mutability State: {message}")]
    MessageMutability { message: String },

    #[error("Codec Decode Error: {message}")]
    InkMessage { message: String },

    #[error("CallExec Error: {error}")]
    CallExec { error: String },

    #[error("ContractEmitted event not found")]
    ContractEmitted,

    #[error("Event not found")]
    EventNotFound,

    #[error("No data found at provided storage key")]
    StorageEntryIsEmpty,
}

impl From<LangError> for ClientError {
    fn from(_value: LangError) -> Self {
        Self::InkMessage {
            message: "Failed to read execution input for the dispatchable.".to_string(),
        }
    }
}

impl From<sp_runtime::DispatchError> for ClientError {
    fn from(value: sp_runtime::DispatchError) -> Self {
        let error = match value {
            sp_runtime::DispatchError::Other(err) => err.to_string(),
            sp_runtime::DispatchError::CannotLookup => "cannot lookup".to_string(),
            sp_runtime::DispatchError::BadOrigin => "bad origin".to_string(),
            sp_runtime::DispatchError::Module(_) => "module error".to_string(),
            sp_runtime::DispatchError::ConsumerRemaining => "consumer remaining".to_string(),
            sp_runtime::DispatchError::NoProviders => "no providers".to_string(),
            sp_runtime::DispatchError::TooManyConsumers => "to many consumers".to_string(),
            sp_runtime::DispatchError::Token(_) => "token error".to_string(),
            sp_runtime::DispatchError::Arithmetic(_) => "arithmetic error".to_string(),
            sp_runtime::DispatchError::Transactional(_) => "transactional error".to_string(),
            sp_runtime::DispatchError::Exhausted => "exhausted error".to_string(),
            sp_runtime::DispatchError::Corruption => "corruption error".to_string(),
            sp_runtime::DispatchError::Unavailable => "unavailable error".to_string(),
            sp_runtime::DispatchError::RootNotAllowed => "root not allowed error".to_string(),
        };

        Self::SpRuntime { error }
    }
}

impl From<ContractAccessError> for ClientError {
    fn from(value: ContractAccessError) -> Self {
        let error = match value {
            ContractAccessError::DoesntExist => {
                "Contract does not exist at specified address".to_string()
            }
            ContractAccessError::KeyDecodingFailed => {
                "Storage key cannot be decoded from input".to_string()
            }
            ContractAccessError::MigrationInProgress => "Migration in progress".to_string(),
        };

        Self::ContractAccess { error: error }
    }
}

#[derive(Encode)]
struct Call<AccountId, Balance> {
    origin: AccountId,
    dest: AccountId,
    value: Balance,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<Balance>,
    input_data: Vec<u8>,
}

#[derive(Encode)]
struct Instantiate<AccountId, Balance, Hash> {
    origin: AccountId,
    value: Balance,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<Balance>,
    code: Code<Hash>,
    data: Vec<u8>,
    salt: Vec<u8>,
}

impl<AccountId, Balance> Call<AccountId, Balance> {
    fn new(
        origin: AccountId,
        dest: AccountId,
        value: Balance,
        gas_limit: Option<Weight>,
        storage_deposit_limit: Option<Balance>,
        input_data: Vec<u8>,
    ) -> Self {
        Self {
            origin,
            dest,
            value,
            gas_limit,
            storage_deposit_limit,
            input_data,
        }
    }
}

impl<AccountId, Balance, Hash> Instantiate<AccountId, Balance, Hash> {
    fn new(origin: AccountId, value: Balance, code: Vec<u8>, data: Vec<u8>, salt: Vec<u8>) -> Self {
        Self {
            origin,
            value,
            gas_limit: None,
            storage_deposit_limit: None,
            code: Code::Upload(code),
            data,
            salt,
        }
    }
}

impl From<sp_weights::Weight> for Weight {
    fn from(value: sp_weights::Weight) -> Self {
        Self {
            ref_time: value.ref_time(),
            proof_size: value.proof_size(),
        }
    }
}
