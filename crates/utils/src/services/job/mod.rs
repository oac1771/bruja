pub mod job_builder;
pub mod job_runner;

use codec::{Decode, Encode};

pub trait JobT: Encode + Decode {
    fn code_ref(&self) -> &[u8];
    fn code(self) -> Vec<u8>;
    fn params_ref(&self) -> &Vec<Vec<u8>>;
}

#[derive(Encode, Decode)]
pub struct Job {
    code: Vec<u8>,
    params: Vec<Vec<u8>>,
}

impl Job {
    pub fn new(code: Vec<u8>, params: Vec<Vec<u8>>) -> Self {
        Self { code, params }
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
}
