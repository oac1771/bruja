use clap::Parser;
use wasmtime::*;

#[derive(Debug, Parser)]
pub struct WasmTime;

impl WasmTime {
    pub async fn handle(&self) {
        let engine = Engine::default();

        // Modules can be compiled through either the text or binary format
        let wat = r#"
                (module
                    (import "host" "host_func" (func $host_hello (param i32)))

                    (func (export "hello")
                        i32.const 420
                        call $host_hello)
                )
            "#;

        let module = Module::new(&engine, wat).unwrap();

        module
            .imports()
            .for_each(|i| println!("{}, {}", i.module(), i.name()));
        module.exports().for_each(|i| println!("{}", i.name()));

        // Host functionality can be arbitrary Rust functions and is provided
        // to guests through a `Linker`.
        let mut linker = Linker::new(&engine);
        linker
            .func_wrap(
                "host",
                "host_func",
                |caller: Caller<'_, u32>, param: i32| {
                    println!("Got {} from WebAssembly", param);
                    println!("my host state is: {}", caller.data());
                },
            )
            .unwrap();

        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using `4` for.
        let mut store: Store<u32> = Store::new(&engine, 4);

        // // Instantiation of a module requires specifying its imports and then
        // // afterwards we can fetch exports by name, as well as asserting the
        // // type signature of the function with `get_typed_func`.
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let hello = instance
            .get_typed_func::<(), ()>(&mut store, "hello")
            .unwrap();

        // // And finally we can call the wasm!
        hello.call(&mut store, ()).unwrap();
    }
}
