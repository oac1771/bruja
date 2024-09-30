use anyhow::{anyhow, Error};
use codec::{Decode, Encode, Error as CodecError};
use wasmtime::*;

pub async fn start_job(raw_job: Vec<u8>) -> Result<(), Error> {
    let engine = Engine::default();
    let mut linker: Linker<()> = Linker::new(&engine);
    let mut store: Store<()> = Store::new(&engine, ());
    let module = Module::new(&engine, &raw_job)?;

    println!("Linking Host Functions...");
    module.imports().try_for_each(|i| match i.ty() {
        ExternType::Func(func) => {
            linker.func_new(i.module(), i.name(), func, |_, _, _| Ok(()))?;
            Ok::<(), Error>(())
        }
        _ => Ok(()),
    })?;

    let instance = linker.instantiate(&mut store, &module)?;

    println!("Executing Export Functions...");
    module.exports().try_for_each(|e| match e.ty() {
        ExternType::Func(func) => {
            let foo = 10_u32.encode();
            let (params, mut results) = build_input_output(func, vec![foo])?;

            instance
                .get_func(&mut store, e.name())
                .ok_or_else(|| anyhow!("Export Function Not Found"))?
                .call(&mut store, &params, &mut results)?;

            println!("results {:?}", results);
            Ok::<(), Error>(())
        }
        _ => Ok(()),
    })?;

    Ok(())
}

fn build_input_output(func: FuncType, raw_params: Vec<Vec<u8>>) -> Result<(Vec<Val>, Vec<Val>), CodecError> {
    let params = func
        .params()
        .zip(raw_params)
        .map(|(val_type, raw_param)| match val_type {
            ValType::I32 => {
                match <i32 as Decode>::decode(&mut raw_param.as_slice()) {
                    Ok(p) => Ok(Val::I32(p)),
                    Err(err) => Err(err)
                }
            }
            _ => Ok(Val::AnyRef(None)),
        })
        .collect::<Result<Vec<Val>, CodecError>>()?;

    let results = func
        .results()
        .map(|val_type| match val_type {
            ValType::I32 => Val::I32(0),
            _ => Val::AnyRef(None),
        })
        .collect::<Vec<Val>>();

    Ok((params, results))
}
