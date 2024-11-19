pub mod job_builder;
pub mod job_runner;

use codec::{Decode, Encode};
use std::any::Any;

pub trait JobT: Encode + Decode + Any {
    fn code_ref(&self) -> &[u8];
    fn code(self) -> Vec<u8>;
    fn params_ref(&self) -> &Vec<Vec<u8>>;
    fn func_name_ref(&self) -> &Vec<u8>;
}

#[derive(Encode, Decode)]
pub struct Job {
    code: Vec<u8>,
    params: Vec<Vec<u8>>,
    func_name: Vec<u8>,
}

impl Job {
    pub fn new(code: Vec<u8>, params: Vec<Vec<u8>>, func_name: String) -> Self {
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

    fn func_name_ref(&self) -> &Vec<u8> {
        &self.func_name
    }
}
