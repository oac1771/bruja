use clap::Parser;
use wasmtime::*;

#[derive(Debug, Parser)]
pub struct WasmTime;

impl WasmTime {
    pub async fn handle(&self) {
        let engine = Engine::default();
        let mut linker: Linker<()> = Linker::new(&engine);
        let mut store: Store<()> = Store::new(&engine, ());

        let wat = r#"
                (module
                    (import "host" "host_func" (func $host_hello (param i32)))

                    (func (export "hello")
                        i32.const 420
                        call $host_hello)
                )
            "#;

        let module = Module::new(&engine, wat).unwrap();

        module.imports().for_each(|i| {
            match i.ty() {
                ExternType::Func(func) => {
                    linker.func_new(i.module(), i.name(), func, |_, params, _| {
                        let foo = params[0].unwrap_i32();
                        println!("from wasm binary {:?}", foo);
                        Ok(())
                    }).unwrap();
                },
                _ => {}
            }

        });

        let instance = linker.instantiate(&mut store, &module).unwrap();

        module.exports().for_each(|e| {
            let func = instance
                .get_typed_func::<(), ()>(&mut store, e.name())
                .unwrap();
            func.call(&mut store, ()).unwrap();
            
        });

    }
}
