use super::{
    job_runner::{Error, WasmJobRunner},
    Encode, Job,
};
use crate::services::job::wat::*;
use wabt::wat2wasm;
use wasmtime::{Engine, Linker, Module, Store};

#[test]
fn get_func_type_returns_func_type() {
    let code = wat2wasm(ADD_ONE).unwrap();

    let engine = Engine::default();
    let module = Module::new(&engine, code.clone()).unwrap();

    let job = Job::new(code.clone(), vec![], "add_one");
    let runner = WasmJobRunner;

    runner.get_func_type(&job, &module).unwrap();
}

#[test]
fn get_func_type_returns_error_if_not_found() {
    let code = wat2wasm(ADD_ONE).unwrap();
    let function_name = "foo".to_string();

    let engine = Engine::default();
    let module = Module::new(&engine, code.clone()).unwrap();

    let job = Job::new(code.clone(), vec![], &function_name);
    let runner = WasmJobRunner;

    let err = runner.get_func_type(&job, &module).err().unwrap();

    if let Error::FunctionExportNotFound { func_name } = err {
        assert_eq!(func_name, function_name);
    } else {
        panic!("Returned unexpected error")
    }
}

#[test]
fn build_params_returns_wasm_val() {
    let code = wat2wasm(ADD_ONE).unwrap();
    let func_name = "add_one";
    let val: i32 = 10;
    let params = vec![val.encode()];

    let engine = Engine::default();
    let module = Module::new(&engine, code.clone()).unwrap();

    let job = Job::new(code.clone(), params, func_name);
    let runner = WasmJobRunner;

    let func = runner.get_func_type(&job, &module).unwrap();
    let params = runner.build_params(&job, &func).unwrap();

    assert_eq!(params.len(), 1);
    assert_eq!(params[0].i32().unwrap(), val);
}

#[test]
fn build_params_errors_if_cannot_parse_param() {
    let code = wat2wasm(ADD_ONE).unwrap();
    let func_name = "add_one";
    let val = i8::MAX;
    let params = vec![val.encode()];

    let engine = Engine::default();
    let module = Module::new(&engine, code.clone()).unwrap();

    let job = Job::new(code.clone(), params, func_name);
    let runner = WasmJobRunner;

    let func = runner.get_func_type(&job, &module).unwrap();
    let err = runner.build_params(&job, &func).err().unwrap();

    if let Error::Codec { source: _ } = err {
        assert!(true)
    } else {
        panic!("Unexpected error returned")
    }
}

#[test]
fn build_params_returns_empty_vec_if_init_params_is_empty() {
    let code = wat2wasm(ADD_ONE).unwrap();
    let func_name = "add_one";
    let params = vec![];

    let engine = Engine::default();
    let module = Module::new(&engine, code.clone()).unwrap();

    let job = Job::new(code.clone(), params, func_name);
    let runner = WasmJobRunner;

    let func = runner.get_func_type(&job, &module).unwrap();
    let res = runner.build_params(&job, &func).unwrap();

    assert_eq!(res.len(), 0);
}

#[test]
fn execute_export_fn_works() {
    let code = wat2wasm(ADD_ONE).unwrap();
    let func_name = "add_one";
    let val: i32 = 10;
    let params = vec![val.encode()];

    let engine = Engine::default();
    let linker: Linker<()> = Linker::new(&engine);
    let mut store: Store<()> = Store::new(&engine, ());
    let module = Module::new(&engine, code.clone()).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();

    let job = Job::new(code.clone(), params, func_name);
    let runner = WasmJobRunner;

    let func = runner.get_func_type(&job, &module).unwrap();
    let params = runner.build_params(&job, &func).unwrap();
    let results = runner.build_results(&func);

    let res = runner
        .execute_export_function(store, instance, &job, params.as_slice(), results)
        .unwrap();

    assert_eq!(res[0].i32().unwrap(), 11);
}
