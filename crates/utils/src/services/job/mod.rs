pub mod job_builder;
pub mod job_runner;

#[cfg(test)]
mod test_job_builder;
#[cfg(test)]
mod test_job_runner;

use codec::{Decode, Encode};
use std::{any::Any, string::FromUtf8Error};

pub trait JobT: Encode + Decode + Any {
    fn code_ref(&self) -> &[u8];
    fn code(self) -> Vec<u8>;
    fn params_ref(&self) -> &Vec<Vec<u8>>;
    fn func_name_string(&self) -> Result<String, FromUtf8Error>;
}

#[derive(Encode, Decode)]
pub struct Job {
    code: Vec<u8>,
    params: Vec<Vec<u8>>,
    func_name: Vec<u8>,
}

impl Job {
    pub fn new(code: Vec<u8>, params: Vec<Vec<u8>>, func_name: &str) -> Self {
        let func_name = func_name.as_bytes().to_vec();
        Self {
            code,
            params,
            func_name,
        }
    }
}

impl JobT for Job {
    fn code_ref(&self) -> &[u8] {
        self.code.as_slice()
    }

    fn code(self) -> Vec<u8> {
        self.code
    }

    fn params_ref(&self) -> &Vec<Vec<u8>> {
        &self.params
    }

    fn func_name_string(&self) -> Result<String, FromUtf8Error> {
        let string = String::from_utf8(self.func_name.clone())?;
        Ok(string)
    }
}

#[cfg(test)]
mod wat {
    pub const ADD_ONE: &'static str = r#"
        (module
            (func $add_one (param $lhs i32) (result i32)
                local.get $lhs
                i32.const 1
                i32.add)
            (export "add_one" (func $add_one))
        )
    "#;
}
