use crate::{
    interface::{self, AccountId, Balance, EventRecord, Hash},
    Block, BlockNumber, Contracts, Header, InherentDataExt, Runtime, RuntimeBlockWeights,
    RuntimeExecutive, RuntimeGenesisConfig, System, TransactionPayment, CONTRACTS_DEBUG_OUTPUT,
    CONTRACTS_EVENTS, VERSION,
};

use frame::{
    deps::frame_support::genesis_builder_helper::{build_state, get_preset},
    prelude::*,
    runtime::{
        apis::{
            self, impl_runtime_apis, ApplyExtrinsicResult, CheckInherentsResult,
            ExtrinsicInclusionMode, OpaqueMetadata,
        },
        prelude::*,
    },
};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};

impl_runtime_apis! {
    impl apis::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            RuntimeExecutive::execute_block(block)
        }

        fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
            RuntimeExecutive::initialize_block(header)
        }
    }
    impl apis::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl apis::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
            RuntimeExecutive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> HeaderFor<Runtime> {
            RuntimeExecutive::finalize_block()
        }

        fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: InherentData,
        ) -> CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl apis::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: ExtrinsicFor<Runtime>,
            block_hash: <Runtime as frame_system::Config>::Hash,
        ) -> TransactionValidity {
            RuntimeExecutive::validate_transaction(source, tx, block_hash)
        }
    }

    impl apis::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &HeaderFor<Runtime>) {
            RuntimeExecutive::offchain_worker(header)
        }
    }

    impl apis::SessionKeys<Block> for Runtime {
        fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
            Default::default()
        }

        fn decode_session_keys(
            _encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, apis::KeyTypeId)>> {
            Default::default()
        }
    }

    impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
        fn account_nonce(account: interface::AccountId) -> interface::Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
        Block,
        interface::Balance,
    > for Runtime {
        fn query_info(uxt: ExtrinsicFor<Runtime>, len: u32) -> RuntimeDispatchInfo<interface::Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(uxt: ExtrinsicFor<Runtime>, len: u32) -> FeeDetails<interface::Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> interface::Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> interface::Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash, EventRecord>
    for Runtime {
    fn call(
        origin: AccountId,
        dest: AccountId,
        value: Balance,
        gas_limit: Option<Weight>,
        storage_deposit_limit: Option<Balance>,
        input_data: Vec<u8>,
    ) -> pallet_contracts::ContractExecResult<Balance, EventRecord> {
        let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
        Contracts::bare_call(
            origin,
            dest,
            value,
            gas_limit,
            storage_deposit_limit,
            input_data,
            CONTRACTS_DEBUG_OUTPUT,
            CONTRACTS_EVENTS,
            pallet_contracts::Determinism::Enforced,
        )
    }

    fn instantiate(
        origin: AccountId,
        value: Balance,
        gas_limit: Option<Weight>,
        storage_deposit_limit: Option<Balance>,
        code: pallet_contracts::Code<Hash>,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> pallet_contracts::ContractInstantiateResult<AccountId, Balance, EventRecord>
    {
        let gas_limit = gas_limit.unwrap_or(RuntimeBlockWeights::get().max_block);
        Contracts::bare_instantiate(
            origin,
            value,
            gas_limit,
            storage_deposit_limit,
            code,
            data,
            salt,
            CONTRACTS_DEBUG_OUTPUT,
            CONTRACTS_EVENTS,
        )
    }

    fn upload_code(
        origin: AccountId,
        code: Vec<u8>,
        storage_deposit_limit: Option<Balance>,
        determinism: pallet_contracts::Determinism,
    ) -> pallet_contracts::CodeUploadResult<Hash, Balance>
    {
        Contracts::bare_upload_code(origin, code, storage_deposit_limit, determinism)
    }

    fn get_storage(
        address: AccountId,
        key: Vec<u8>,
    ) -> pallet_contracts::GetStorageResult {
        Contracts::get_storage(address, key)
    }
}

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }

    // add contracts runtime apis
}
