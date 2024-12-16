use super::{Job, JobT, RawResults, RawResultsT};
use codec::{Decode, Encode};
use futures::Future;
use mockall::automock;
use tokio::task::JoinHandle;
use wasmtime::{Engine, ExternType, Func, FuncType, Instance, Linker, Module, Store, Val, ValType};

#[automock(type Job=Job; type RawResults=RawResults; type Err=WasmJobRunnerServiceError;)]
pub trait WasmJobRunnerService {
    type Err;
    type Job: JobT;
    type RawResults: RawResultsT;

    fn start_job(
        &self,
        job: Self::Job,
    ) -> impl Future<Output = Result<Self::RawResults, Self::Err>> + Send;
}

pub struct WasmJobRunner {
    engine: Engine,
}

impl Default for WasmJobRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmJobRunnerService for WasmJobRunner {
    type Err = WasmJobRunnerServiceError;
    type Job = Job;
    type RawResults = RawResults;

    async fn start_job(&self, job: Self::Job) -> Result<Self::RawResults, Self::Err> {
        let module = Module::new(&self.engine, job.code_ref())
            .map_err(|e| WasmJobRunnerServiceError::WasmModule { err: e.to_string() })?;
        let mut linker: Linker<()> = Linker::new(&self.engine);
        let mut store = Store::new(&self.engine, ());
        let instance = linker.instantiate(&mut store, &module)?;
        let func = self.get_func(&job, instance, &mut store)?;

        let (params, results) = self.prepare_job(&module, &job, &mut linker)?;
        let handle = self.run_job(func, params, results, store).await?;
        let result = handle.await??;

        Ok(result)
    }
}

impl WasmJobRunner {
    pub fn new() -> Self {
        let engine = Engine::default();

        Self { engine }
    }

    pub(crate) async fn run_job(
        &self,
        func: Func,
        params: Vec<Val>,
        mut results: Vec<Val>,
        store: Store<()>,
    ) -> Result<JoinHandle<Result<RawResults, Error>>, Error> {
        let job_handle: JoinHandle<Result<RawResults, Error>> = tokio::spawn(async move {
            func.call(store, &params, &mut results)
                .map_err(|e| Error::JobError { err: e.to_string() })?;
            let result = results
                .iter()
                .map(|v| match v {
                    Val::I32(t) => t.encode(),
                    Val::I64(t) => t.encode(),
                    _ => vec![],
                })
                .collect::<Vec<Vec<u8>>>();
            let r = RawResults::from_vec(result);

            Ok(r)
        });

        Ok(job_handle)
    }

    fn prepare_job(
        &self,
        module: &Module,
        job: &Job,
        linker: &mut Linker<()>,
    ) -> Result<(Vec<Val>, Vec<Val>), Error> {
        let func = self.get_func_type(job, module)?;
        let params = self.build_params(job, &func)?;
        let results = self.build_results(&func);
        self.define_host_fn(module, linker)?;

        Ok((params, results))
    }

    fn define_host_fn(&self, module: &Module, linker: &mut Linker<()>) -> Result<(), Error> {
        module.imports().try_for_each(|i| match i.ty() {
            ExternType::Func(func) => {
                linker.func_new(i.module(), i.name(), func, |_, _, _| Ok(()))?;
                Ok::<(), Error>(())
            }
            _ => Ok(()),
        })?;

        Ok(())
    }

    pub(crate) fn get_func_type(
        &self,
        job: &<WasmJobRunner as WasmJobRunnerService>::Job,
        module: &Module,
    ) -> Result<FuncType, Error> {
        let name = job.func_name_string()?;

        let func = module
            .get_export(&name)
            .ok_or_else(|| Error::FunctionExportNotFound { func_name: name })?;

        let res = if let ExternType::Func(f) = func {
            Ok(f)
        } else {
            Err(Error::FuncTypeNotFound)
        }?;

        Ok(res)
    }

    pub(crate) fn get_func(
        &self,
        job: &<WasmJobRunner as WasmJobRunnerService>::Job,
        instance: Instance,
        store: &mut Store<()>,
    ) -> Result<Func, Error> {
        let name = job.func_name_string()?;

        let func = instance
            .get_func(store, &name)
            .ok_or_else(|| Error::FunctionExportNotFound { func_name: name })?;

        Ok(func)
    }

    pub(crate) fn build_params(
        &self,
        job: &<WasmJobRunner as WasmJobRunnerService>::Job,
        func: &FuncType,
    ) -> Result<Vec<Val>, Error> {
        let params = job
            .params_ref()
            .iter()
            .zip(func.params())
            .map(|(raw_param, val_type)| {
                let decoded = match val_type {
                    ValType::I32 => Some(self.decode_param::<i32>(raw_param)),
                    ValType::I64 => Some(self.decode_param::<i64>(raw_param)),
                    _ => None,
                };

                match decoded {
                    Some(Ok(val)) => Ok(val),
                    Some(Err(e)) => Err(e),
                    None => Err(Error::ParamTypeNotFound),
                }
            })
            .collect::<Result<Vec<Val>, Error>>()?;

        Ok(params)
    }

    pub(crate) fn build_results(&self, func: &FuncType) -> Vec<Val> {
        let results = func
            .results()
            .map(|val_type| match val_type {
                ValType::I32 => Val::I32(0),
                _ => Val::AnyRef(None),
            })
            .collect::<Vec<Val>>();

        results
    }

    fn decode_param<P: Decode + Into<Val>>(&self, mut p: &[u8]) -> Result<Val, Error> {
        let param = <P as Decode>::decode(&mut p)?;
        let val: Val = param.into();
        Ok(val)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WasmJobRunnerServiceError {
    #[error("{source}")]
    WasmJobRunner {
        #[from]
        source: Error,
    },

    #[error("{source}")]
    WasmTime {
        #[from]
        source: wasmtime::Error,
    },

    #[error("")]
    Join {
        #[from]
        source: tokio::task::JoinError,
    },

    #[error("")]
    WasmModule { err: String },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    WasmTime {
        #[from]
        source: wasmtime::Error,
    },

    #[error("")]
    FromUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },

    #[error("")]
    Codec {
        #[from]
        source: codec::Error,
    },

    #[error("{err}")]
    JobError { err: String },

    #[error("")]
    FuncTypeNotFound,

    #[error("Export {func_name} not defined in job")]
    FunctionExportNotFound { func_name: String },

    #[error("Param type not found")]
    ParamTypeNotFound,
}
