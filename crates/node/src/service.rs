//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use futures::FutureExt;
use runtime::{api::dispatch, opaque::Block, RuntimeApi};
use std::sync::Arc;

use sc_basic_authorship::ProposerFactory;
use sc_client_api::Backend;
use sc_consensus::{DefaultImportQueue, LongestChain};
use sc_consensus_manual_seal::{
    import_queue, run_delayed_finalize, run_instant_seal, DelayedFinalizeParams, InstantSealParams,
};
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch, NativeVersion};
use sc_network::config::FullNetworkConfiguration;
use sc_offchain::OffchainWorkers;
use sc_service::{
    build_network, error::Error as ServiceError, new_full_parts, new_native_or_wasm_executor,
    spawn_tasks, BuildNetworkParams, Configuration, PartialComponents, SpawnTasksParams,
    TFullBackend, TFullClient, TaskManager,
};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool::{BasicPool, FullPool};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;

use sp_timestamp::InherentDataProvider;

use crate::rpc::{create_full, FullDeps};

// Our native executor instance.
pub struct ExecutorDispatch;

impl NativeExecutionDispatch for ExecutorDispatch {
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        dispatch(method, data)
    }

    fn native_version() -> NativeVersion {
        runtime::native_version()
    }
}

pub(crate) type FullClient =
    TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = TFullBackend<Block>;
type FullSelectChain = LongestChain<FullBackend, Block>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        DefaultImportQueue<Block>,
        FullPool<Block, FullClient>,
        (Option<Telemetry>,),
    >,
    ServiceError,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = new_native_or_wasm_executor(config);

    let (client, backend, keystore_container, task_manager) = new_full_parts::<Block, RuntimeApi, _>(
        config,
        telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
        executor,
    )?;
    let client = Arc::new(client);

    let select_chain = LongestChain::new(backend.clone());

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let import_queue = import_queue(
        Box::new(client.clone()),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    let transaction_pool = BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    Ok(PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (telemetry,),
    })
}

pub fn new_full(
    config: Configuration,
    finalize_delay_sec: u64,
) -> Result<TaskManager, ServiceError> {
    let PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (mut telemetry,),
    } = new_partial(&config)?;

    let net_config = FullNetworkConfiguration::new(&config.network);

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        build_network(BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_params: None,
            block_relay: None,
        })?;

    if config.offchain_worker.enabled {
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
                runtime_api_provider: client.clone(),
                is_validator: config.role.is_authority(),
                keystore: Some(keystore_container.keystore()),
                offchain_db: backend.offchain_storage(),
                transaction_pool: Some(OffchainTransactionPoolFactory::new(
                    transaction_pool.clone(),
                )),
                network_provider: network.clone(),
                enable_http_requests: true,
                custom_extensions: |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    let prometheus_registry = config.prometheus_registry().cloned();
    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps = FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
            };
            create_full(deps).map_err(Into::into)
        })
    };

    spawn_tasks(SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend,
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
    })?;

    let proposer = ProposerFactory::new(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool.clone(),
        prometheus_registry.as_ref(),
        telemetry.as_ref().map(|x| x.handle()),
    );

    let params = InstantSealParams {
        block_import: client.clone(),
        env: proposer,
        client: client.clone(),
        pool: transaction_pool,
        select_chain,
        consensus_data_provider: None,
        create_inherent_data_providers: move |_, ()| async move {
            Ok(InherentDataProvider::from_system_time())
        },
    };

    let authorship_future = run_instant_seal(params);
    task_manager
        .spawn_essential_handle()
        .spawn_blocking("instant-seal", None, authorship_future);

    let delayed_finalize_params = DelayedFinalizeParams {
        client,
        spawn_handle: task_manager.spawn_handle(),
        delay_sec: finalize_delay_sec,
    };
    task_manager.spawn_essential_handle().spawn_blocking(
        "delayed_finalize",
        None,
        run_delayed_finalize(delayed_finalize_params),
    );

    network_starter.start_network();
    Ok(task_manager)
}
