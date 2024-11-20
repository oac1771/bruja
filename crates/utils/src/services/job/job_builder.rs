use super::{Job, JobT};
use codec::Encode;
use std::{any::type_name, future::Future, path::Path, str::FromStr};
use tokio::{fs::File, io::AsyncReadExt};
use wasmtime::{Engine, Module, ValType};

pub trait JobBuilderService {
    type Err;
    type Job: JobT;

    fn build_job(&self) -> impl Future<Output = Result<Self::Job, Self::Err>> + Send;
}

pub struct JobBuilder<'a> {
    code_path: Box<&'a Path>,
    params: RawParams,
    function_name: String,
}

impl<'a> JobBuilderService for JobBuilder<'a> {
    type Err = JobBuilderServiceError;
    type Job = Job;

    async fn build_job(&self) -> Result<Self::Job, JobBuilderServiceError> {
        let mut code = Vec::<u8>::new();
        let mut code_file = File::open(self.code_path.as_ref()).await?;
        code_file.read_to_end(&mut code).await?;

        let engine = Engine::default();
        let module = Module::new(&engine, code.as_slice())
            .map_err(|e| JobBuilderServiceError::WasmModule { err: e.to_string() })?;

        let params = self.parse_params(&module)?;
        let job = Job::new(code, params, self.function_name.to_string());

        Ok(job)
    }
}

impl<'a> JobBuilder<'a> {
    pub async fn new(
        code_path: &'a str,
        parameters: Option<String>,
        function_name: &str,
    ) -> Result<Self, JobBuilderServiceError> {
        let path = Path::new(code_path);

        if !path.exists() {
            return Err(JobBuilderServiceError::CodeFileNotFound);
        }

        let raw_param = if let Some(p) = parameters {
            p
        } else {
            String::from("")
        };

        let params = RawParams::new(raw_param);

        Ok(Self {
            code_path: Box::new(path),
            params,
            function_name: function_name.to_string(),
        })
    }

    fn parse_params(&self, module: &Module) -> Result<Vec<Vec<u8>>, JobBuilderServiceError> {
        let extern_type = module.get_export(&self.function_name).ok_or_else(|| {
            JobBuilderServiceError::FunctionExportNotFound {
                func_name: self.function_name.clone(),
            }
        })?;
        let func =
            extern_type
                .func()
                .ok_or_else(|| JobBuilderServiceError::FunctionNameNotFound {
                    func_name: self.function_name.clone(),
                })?;

        let p = self
            .params
            .to_vec()
            .iter()
            .zip(func.params())
            .map(|(param, ty)| {
                let parsed = match ty {
                    ValType::I32 => Some(self.parse::<i32>(param)),
                    ValType::I64 => Some(self.parse::<i64>(param)),
                    _ => None,
                };

                match parsed {
                    Some(Ok(val)) => Ok(val),
                    Some(Err(e)) => Err(e),
                    None => Err(JobBuilderServiceError::ParamTypeNotFound),
                }
            })
            .collect::<Result<Vec<Vec<u8>>, JobBuilderServiceError>>()?;

        Ok(p)
    }

    fn parse<T: FromStr + Encode>(&self, t: &str) -> Result<Vec<u8>, JobBuilderServiceError> {
        match t.parse::<T>() {
            Ok(val) => Ok(val.encode()),
            Err(_) => Err(JobBuilderServiceError::ParseParam {
                err: format!("Unable to parse param {} into {}", t, type_name::<T>()),
            }),
        }
    }
}

pub struct RawParams(Vec<String>);

impl RawParams {
    pub fn new(params: String) -> Self {
        let res = params
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Self(res)
    }

    pub fn to_vec(&self) -> &Vec<String> {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JobBuilderServiceError {
    #[error("{source}")]
    StdIo {
        #[from]
        source: std::io::Error,
    },

    #[error("{source}")]
    WasmTime {
        #[from]
        source: wasmtime::Error,
    },

    #[error("")]
    CodeFileNotFound,

    #[error("")]
    WasmModule { err: String },

    #[error("Function {func_name} not defined in job")]
    FunctionNameNotFound { func_name: String },

    #[error("Export {func_name} not defined in job")]
    FunctionExportNotFound { func_name: String },

    #[error("Unable to parse param: {err}")]
    ParseParam { err: String },

    #[error("Param type not found")]
    ParamTypeNotFound,
}
