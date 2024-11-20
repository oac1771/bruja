use super::*;
use job_builder::{JobBuilder, JobBuilderServiceError, RawParams};
use wabt::wat2wasm;
use wasmtime::{Engine, Module};

const ADD_ONE: &'static str = r#"
        (module
            (func $add_one (param $lhs i32) (result i32)
                local.get $lhs
                i32.const 1
                i32.add)
            (export "add_one" (func $add_one))
        )
    "#;

#[test]
fn raw_params_parses_none_option_to_empty_vec() {
    let params = RawParams::new(None);

    assert_eq!(params.to_vec().len(), 0)
}

#[test]
fn raw_params_parses_some_option_to_vec_strings() {
    let params = Some(String::from("10,10,10"));
    let raw_params = RawParams::new(params);

    assert_eq!(raw_params.to_vec().len(), 3)
}

#[test]
fn successfully_parse_params_into_encoded_values() {
    let val = 10;
    let parameters = Some(String::from(val.clone().to_string()));
    let code = wat2wasm(ADD_ONE).unwrap();

    let engine = Engine::default();
    let module = Module::new(&engine, code).unwrap();

    let job_builder = JobBuilder::test(parameters, "add_one");

    let res = job_builder.parse_params(&module).unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0], val.encode());
}

#[test]
fn returns_error_if_unable_to_parse_into_type() {
    let val = String::from("random string");
    let parameters = Some(val.clone());
    let code = wat2wasm(ADD_ONE).unwrap();

    let engine = Engine::default();
    let module = Module::new(&engine, code).unwrap();

    let job_builder = JobBuilder::test(parameters, "add_one");

    let err = job_builder.parse_params(&module).err().unwrap();

    if let JobBuilderServiceError::ParseParam { err } = err {
        assert_eq!(err, format!("Unable to parse param '{}' into i32", val));
    } else {
        panic!("Returned unexpected error")
    }
}
