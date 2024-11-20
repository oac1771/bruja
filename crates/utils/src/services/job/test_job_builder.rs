use wabt::wat2wasm;
use wasmtime::{Engine, Linker, Module, Store};

#[test]
fn foo() {
    let wat = r#"
        (module
            (func $add_one (param $lhs i32) (result i32)
                local.get $lhs
                i32.const 1
                i32.add)
            (export "add_one" (func $add_one))
        )
    "#;

    let code = wat2wasm(wat).unwrap();

    let engine = Engine::default();
    let module = Module::new(&engine, code).unwrap();
    let mut store: Store<()> = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    let instance = linker.instantiate(&mut store, &module).unwrap();
    let foo = instance
        .get_typed_func::<i32, i32>(&mut store, "add_one")
        .unwrap();
    let bar = foo.call(&mut store, 10).unwrap();

    assert_eq!(bar, 11);
}
