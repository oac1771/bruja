use clap::Parser;
use ink::scale::{Decode, Encode};
use wasmtime::*;

#[derive(Debug, Parser)]
pub struct WasmTime;

impl WasmTime {
    pub async fn handle(&self) {
        let engine = Engine::default();
        let mut linker: Linker<()> = Linker::new(&engine);
        let mut store: Store<()> = Store::new(&engine, ());

        let module = Module::from_file(&engine, "../work/pkg/work_bg.wasm").unwrap();

        module.imports().for_each(|i| {
            if let ExternType::Func(func) = i.ty() {
                println!("{}, {}", i.module(), i.name());
                linker
                    .func_new(i.module(), i.name(), func, |_, _, _| Ok(()))
                    .unwrap();
            }
        });

        let instance = linker.instantiate(&mut store, &module).unwrap();

        module.exports().for_each(|e| {
            if let ExternType::Func(func) = e.ty() {
                let param = 10_u32.encode();
                let (params, mut results) = build_input_output(func, vec![param]);

                instance
                    .get_func(&mut store, e.name())
                    .unwrap()
                    .call(&mut store, &params, &mut results)
                    .unwrap();

                println!("results {:?}", results);
            }
        });
    }
}

fn build_input_output(func: FuncType, raw_params: Vec<Vec<u8>>) -> (Vec<Val>, Vec<Val>) {
    let params = func
        .params()
        .zip(raw_params)
        .map(|(val_type, raw_param)| {
            if let ValType::I32 = val_type {
                let p = <i32 as Decode>::decode(&mut raw_param.as_slice()).unwrap();
                Val::I32(p)
            } else {
                Val::AnyRef(None)
            }
        })
        .collect::<Vec<Val>>();

    let results = func
        .results()
        .map(|val_type| match val_type {
            ValType::I32 => Val::I32(0),
            _ => Val::AnyRef(None),
        })
        .collect::<Vec<Val>>();

    (params, results)
}
