use super::{Job, JobT};
use wasmtime::{Engine, ExternType, Linker, Module, Store};

pub trait WasmJobRunnerService {
    type Err;
    type Job: JobT;

    fn start_job(&self, job: Self::Job) -> Result<(), Self::Err>;
}

struct WasmJobRunner;

impl WasmJobRunnerService for WasmJobRunner {
    type Err = WasmJobRunnerServiceError;
    type Job = Job;

    fn start_job(&self, job: Self::Job) -> Result<(), Self::Err> {
        let engine = Engine::default();
        let mut linker: Linker<()> = Linker::new(&engine);
        let mut store: Store<()> = Store::new(&engine, ());
        let module = Module::new(&engine, job.code_ref())
            .map_err(|e| WasmJobRunnerServiceError::WasmModule { err: e.to_string() })?;

        self.define_host_fn(&module, &mut linker)?;
        let _instance = linker.instantiate(&mut store, &module)?;

        Ok(())
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
}

// fn execute_export_function<T>(
//     module: &Module,
//     mut store: Store<T>,
//     instance: Instance,
// ) -> Result<(), JobServiceError> {
//     module.exports().try_for_each(|e| match e.ty() {
//         ExternType::Func(func) => {
//             let (params, mut results) = build_input_output(func, vec![foo])?;

//             instance
//                 .get_func(&mut store, e.name())
//                 .ok_or_else(|| JobServiceError::FunctionExportNotFound {
//                     func_name: e.name().to_string(),
//                 })?
//                 .call(&mut store, &params, &mut results)?;
//             Ok::<(), JobServiceError>(())
//         }
//         _ => Ok(()),
//     })?;

//     Ok(())
// }

// fn build_input_params(
//     func: FuncType,
//     raw_params: Vec<Vec<u8>>,
// ) -> Result<(Vec<Val>, Vec<Val>), CodecError> {
//     let params = func
//         .params()
//         .zip(raw_params)
//         .map(|(val_type, raw_param)| match val_type {
//             ValType::I32 => match <i32 as Decode>::decode(&mut raw_param.as_slice()) {
//                 Ok(p) => Ok(Val::I32(p)),
//                 Err(err) => Err(err),
//             },
//             _ => Ok(Val::AnyRef(None)),
//         })
//         .collect::<Result<Vec<Val>, CodecError>>()?;

//     let results = func
//         .results()
//         .map(|val_type| match val_type {
//             ValType::I32 => Val::I32(0),
//             _ => Val::AnyRef(None),
//         })
//         .collect::<Vec<Val>>();

//     Ok((params, results))
// }
