use super::{Job, JobT};
use codec::Decode;
use wasmtime::{Engine, ExternType, FuncType, Instance, Linker, Module, Store, Val, ValType};

pub trait WasmJobRunnerService {
    type Err;
    type Job: JobT;
    type Results;

    fn start_job(&self, job: Self::Job) -> Result<Self::Results, Self::Err>;
}

pub struct WasmJobRunner;

impl WasmJobRunnerService for WasmJobRunner {
    type Err = WasmJobRunnerServiceError;
    type Job = Job;
    type Results = Vec<Val>;

    fn start_job(&self, job: Self::Job) -> Result<Self::Results, Self::Err> {
        let engine = Engine::default();
        let mut linker: Linker<()> = Linker::new(&engine);
        let mut store: Store<()> = Store::new(&engine, ());
        let module = Module::new(&engine, job.code_ref())
            .map_err(|e| WasmJobRunnerServiceError::WasmModule { err: e.to_string() })?;
        let instance = linker.instantiate(&mut store, &module)?;

        self.define_host_fn(&module, &mut linker)?;
        let func = self.get_func_type(&job, &module)?;
        let params = self.build_params(&job, &func)?;
        let results = self.build_results(&func);
        let res =
            self.execute_export_function(store, instance, &job, params.as_slice(), results)?;

        Ok(res)
    }
}

impl WasmJobRunner {
    fn define_host_fn<T>(
        &self,
        module: &Module,
        linker: &mut Linker<T>,
    ) -> Result<(), WasmJobRunnerServiceError> {
        module.imports().try_for_each(|i| match i.ty() {
            ExternType::Func(func) => {
                linker.func_new(i.module(), i.name(), func, |_, _, _| Ok(()))?;
                Ok::<(), WasmJobRunnerServiceError>(())
            }
            _ => Ok(()),
        })?;

        Ok(())
    }

    pub(crate) fn get_func_type(
        &self,
        job: &<WasmJobRunner as WasmJobRunnerService>::Job,
        module: &Module,
    ) -> Result<FuncType, WasmJobRunnerServiceError> {
        let name = job.func_name_string()?;

        let func = module
            .get_export(&name)
            .ok_or_else(|| WasmJobRunnerServiceError::FunctionExportNotFound { func_name: name })?;

        let res = if let ExternType::Func(f) = func {
            Ok(f)
        } else {
            Err(WasmJobRunnerServiceError::FuncTypeNotFound)
        }?;

        Ok(res)
    }

    pub(crate) fn build_params(
        &self,
        job: &<WasmJobRunner as WasmJobRunnerService>::Job,
        func: &FuncType,
    ) -> Result<Vec<Val>, WasmJobRunnerServiceError> {
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
                    None => Err(WasmJobRunnerServiceError::ParamTypeNotFound),
                }
            })
            .collect::<Result<Vec<Val>, WasmJobRunnerServiceError>>()?;

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

    pub(crate) fn execute_export_function<T>(
        &self,
        mut store: Store<T>,
        instance: Instance,
        job: &<WasmJobRunner as WasmJobRunnerService>::Job,
        params: &[Val],
        mut results: Vec<Val>,
    ) -> Result<Vec<Val>, WasmJobRunnerServiceError> {
        let name = job.func_name_string().unwrap();
        instance
            .get_func(&mut store, &name)
            .ok_or_else(|| WasmJobRunnerServiceError::FunctionExportNotFound { func_name: name })?
            .call(store, params, &mut results)
            .unwrap();

        Ok(results.clone())
    }

    fn decode_param<P: Decode + Into<Val>>(
        &self,
        mut p: &[u8],
    ) -> Result<Val, WasmJobRunnerServiceError> {
        let param = <P as Decode>::decode(&mut p)?;
        let val: Val = param.into();
        Ok(val)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WasmJobRunnerServiceError {
    #[error("{source}")]
    WasmTime {
        #[from]
        source: wasmtime::Error,
    },

    #[error("")]
    WasmModule { err: String },

    #[error("")]
    FuncTypeNotFound,

    #[error("Export {func_name} not defined in job")]
    FunctionExportNotFound { func_name: String },

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

    #[error("Param type not found")]
    ParamTypeNotFound,
}
