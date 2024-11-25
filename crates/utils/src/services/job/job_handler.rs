use super::{Job, JobT, RawResults, RawResultsT, Results, Val};
use codec::{Decode, Encode};
use std::{
    any::type_name,
    fmt::Display,
    future::Future,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::{fs::File, io::AsyncReadExt};
use wasmtime::{Engine, ExternType, FuncType, Module, ValType};

pub trait JobHandlerService {
    type Err;
    type Job: JobT;
    type RawResults: RawResultsT;
    type Results: Display;

    fn build_job(&self) -> impl Future<Output = Result<Self::Job, Self::Err>> + Send;
    fn unpack_results(
        &self,
        results: Self::RawResults,
    ) -> impl Future<Output = Result<Self::Results, Self::Err>> + Send;
    fn validate_params(&self) -> Result<(), Self::Err>;
}

pub struct JobHandler {
    params: RawParams,
    function_name: String,
    func_type: FuncType,
    path: PathBuf,
}

impl JobHandlerService for JobHandler {
    type Err = JobHandlerServiceError;
    type Job = Job;
    type RawResults = RawResults;
    type Results = Results;

    async fn build_job(&self) -> Result<Self::Job, Self::Err> {
        let code = Self::get_code(&self.path).await?;

        self.validate_params()?;
        let params = self.parse_params()?;
        let job = Job::new(code, params, &self.function_name);

        Ok(job)
    }

    async fn unpack_results(&self, results: Self::RawResults) -> Result<Self::Results, Self::Err> {
        let r = self
            .func_type
            .results()
            .zip(results.to_vec())
            .map(|(val_type, res)| {
                let decoded = match val_type {
                    ValType::I32 => Some(self.decode_result::<i32>(&mut res.as_slice())),
                    ValType::I64 => Some(self.decode_result::<i64>(&mut res.as_slice())),
                    _ => None,
                };

                match decoded {
                    Some(Ok(val)) => Ok(val),
                    Some(Err(e)) => Err(e),
                    None => Err(Error::ParamTypeNotFound),
                }
            })
            .collect::<Result<Vec<Val>, Error>>()?;

        let results = Results(r);
        Ok(results)
    }

    fn validate_params(&self) -> Result<(), Self::Err> {
        if self.func_type.params().len() != self.params.to_vec().len() {
            return Err(Self::Err::InvalidParameterNumber);
        }

        Ok(())
    }
}

impl JobHandler {
    pub async fn new(
        code_path: &str,
        parameters: Option<String>,
        function_name: &str,
    ) -> Result<Self, Error> {
        let path = PathBuf::from(code_path);

        if !path.exists() {
            return Err(Error::CodeFileNotFound);
        }

        let code = Self::get_code(&path).await?;

        let engine = Engine::default();
        let module = Module::new(&engine, code.as_slice())
            .map_err(|e| Error::WasmModule { err: e.to_string() })?;

        let extern_type =
            module
                .get_export(function_name)
                .ok_or_else(|| Error::FunctionExportNotFound {
                    func_name: function_name.to_string(),
                })?;

        let func_type = if let ExternType::Func(f) = extern_type {
            Ok(f)
        } else {
            Err(Error::FuncTypeNotFound)
        }?;

        let params = RawParams::new(parameters);

        Ok(Self {
            params,
            function_name: function_name.to_string(),
            func_type,
            path,
        })
    }

    pub(crate) fn parse_params(&self) -> Result<Vec<Vec<u8>>, Error> {
        let p = self
            .params
            .to_vec()
            .iter()
            .zip(self.func_type.params())
            .map(|(param, ty)| {
                let parsed = match ty {
                    ValType::I32 => Some(self.parse::<i32>(param)),
                    ValType::I64 => Some(self.parse::<i64>(param)),
                    _ => None,
                };

                match parsed {
                    Some(Ok(val)) => Ok(val),
                    Some(Err(e)) => Err(e),
                    None => Err(Error::ParamTypeNotFound),
                }
            })
            .collect::<Result<Vec<Vec<u8>>, Error>>()?;

        Ok(p)
    }

    fn parse<T: FromStr + Encode>(&self, t: &str) -> Result<Vec<u8>, Error> {
        match t.parse::<T>() {
            Ok(val) => Ok(val.encode()),
            Err(_) => Err(Error::ParseParam {
                err: format!("Unable to parse param '{}' into {}", t, type_name::<T>()),
            }),
        }
    }

    async fn get_code(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let mut code = Vec::<u8>::new();
        let mut code_file = File::open(path).await?;
        code_file.read_to_end(&mut code).await?;

        Ok(code)
    }

    fn decode_result<R: Decode + Into<Val>>(&self, mut res: &[u8]) -> Result<Val, Error> {
        let param = <R as Decode>::decode(&mut res)?;
        let val: Val = param.into();
        Ok(val)
    }
}

pub struct RawParams(Vec<String>);

impl RawParams {
    pub fn new(params: Option<String>) -> Self {
        let res = if let Some(p) = params {
            p.split(',').map(|s| s.to_string()).collect::<Vec<String>>()
        } else {
            vec![]
        };

        Self(res)
    }

    pub fn to_vec(&self) -> &Vec<String> {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JobHandlerServiceError {
    #[error("{source}")]
    JobHandler {
        #[from]
        source: Error,
    },

    #[error("")]
    InvalidParameterNumber,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{source}")]
    StdIo {
        #[from]
        source: std::io::Error,
    },

    #[error("")]
    WasmModule { err: String },

    #[error("")]
    Codec {
        #[from]
        source: codec::Error,
    },

    #[error("")]
    CodeFileNotFound,

    #[error("")]
    FuncTypeNotFound,

    #[error("Function {func_name} not defined in job")]
    FunctionNameNotFound { func_name: String },

    #[error("Export {func_name} not defined in job")]
    FunctionExportNotFound { func_name: String },

    #[error("Unable to parse param: {err}")]
    ParseParam { err: String },

    #[error("Param type not found")]
    ParamTypeNotFound,
}

#[cfg(test)]
impl JobHandler {
    pub(crate) fn test(parameters: Option<String>, function_name: &str, module: Module) -> Self {
        let params = RawParams::new(parameters);
        let path = PathBuf::new();

        let extern_type = module
            .get_export(function_name)
            .ok_or_else(|| Error::FunctionExportNotFound {
                func_name: function_name.to_string(),
            })
            .unwrap();

        let func_type = if let ExternType::Func(f) = extern_type {
            Ok(f)
        } else {
            Err(Error::FuncTypeNotFound)
        }
        .unwrap();

        Self {
            func_type,
            params,
            function_name: function_name.to_string(),
            path,
        }
    }
}
