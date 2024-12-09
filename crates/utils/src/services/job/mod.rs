pub mod job_handler;
pub mod job_runner;

use codec::{Decode, Encode};
use std::{fmt, string::FromUtf8Error};

pub trait JobT: Encode + Decode {
    fn code_ref(&self) -> &[u8];
    fn params_ref(&self) -> &Vec<Vec<u8>>;
    fn func_name_string(&self) -> Result<String, FromUtf8Error>;
    fn into_parts(self) -> (Vec<u8>, Vec<Vec<u8>>, Vec<u8>);
    fn from_parts(code: Vec<u8>, params: Vec<Vec<u8>>, func_name: Vec<u8>) -> Self;
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

    fn params_ref(&self) -> &Vec<Vec<u8>> {
        &self.params
    }

    fn func_name_string(&self) -> Result<String, FromUtf8Error> {
        let string = String::from_utf8(self.func_name.clone())?;
        Ok(string)
    }

    fn into_parts(self) -> (Vec<u8>, Vec<Vec<u8>>, Vec<u8>) {
        (self.code, self.params, self.func_name)
    }

    fn from_parts(code: Vec<u8>, params: Vec<Vec<u8>>, func_name: Vec<u8>) -> Self {
        Self {
            code,
            params,
            func_name,
        }
    }
}

pub trait RawResultsT {
    fn from_vec(v: Vec<Vec<u8>>) -> Self;
    fn to_vec(self) -> Vec<Vec<u8>>;
}

pub struct RawResults(Vec<Vec<u8>>);
pub struct Results(Vec<Val>);

impl RawResultsT for RawResults {
    fn from_vec(v: Vec<Vec<u8>>) -> Self {
        Self(v)
    }

    fn to_vec(self) -> Vec<Vec<u8>> {
        self.0
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "{} ", v)?;
        }
        Ok(())
    }
}

pub enum Val {
    I32(i32),
    I64(i64),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::I32(val) => write!(f, "{}", val),
            Self::I64(val) => write!(f, "{}", val),
        }
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Val {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

#[cfg(test)]
mod test_job_handler;
#[cfg(test)]
mod test_job_runner;
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
