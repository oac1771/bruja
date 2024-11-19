// async fn start_job(&self, job: Job) -> Result<(), JobServiceError> {
//     let engine = Engine::default();
//     let mut linker: Linker<()> = Linker::new(&engine);
//     let mut store: Store<()> = Store::new(&engine, ());

//     let module = Module::new(&engine, &job.code())
//         .map_err(|e| JobServiceError::WasmModule { err: e.to_string() })?;
//     self.define_host_fn(&module, &mut linker)?;
//     let instance = linker.instantiate(&mut store, &module)?;

//     Ok(())
// }

// fn parse_params(&self, module: &Module) -> Result<Vec<Vec<u8>>, JobServiceError> {
//     let extern_type = module.get_export(&self.function_name).ok_or_else(|| {
//         JobServiceError::FunctionExportNotFound {
//             func_name: self.function_name.clone(),
//         }
//     })?;
//     let func = extern_type
//         .func()
//         .ok_or_else(|| JobServiceError::FunctionNameNotFound {
//             func_name: self.function_name.clone(),
//         })?;

//     let p = self
//         .params
//         .to_vec()
//         .iter()
//         .zip(func.params())
//         .map(|(param, ty)| {
//             let parse_res = match ty {
//                 ValType::I32 => Some(self.parse::<i32>(param)),
//                 ValType::I64 => Some(self.parse::<i64>(param)),
//                 _ => None,
//             };

//             match parse_res {
//                 Some(Ok(val)) => Ok(val),
//                 Some(Err(e)) => Err(e),
//                 None => Err(JobServiceError::ParamTypeNotFound),
//             }
//         })
//         .collect::<Result<Vec<Vec<u8>>, JobServiceError>>()?;

//     Ok(p)
// }

// fn parse<T: FromStr + Encode>(&self, t: &str) -> Result<Vec<u8>, JobServiceError> {
//     match t.parse::<T>() {
//         Ok(val) => Ok(val.encode()),
//         Err(_) => Err(JobServiceError::ParseParam {
//             err: format!("Unable to parse param {} into {}", t, type_name::<T>()),
//         }),
//     }
// }

// fn define_host_fn<T>(
//     &self,
//     module: &Module,
//     linker: &mut Linker<T>,
// ) -> Result<(), JobServiceError> {
//     module.imports().try_for_each(|i| match i.ty() {
//         ExternType::Func(func) => {
//             linker.func_new(i.module(), i.name(), func, |_, _, _| Ok(()))?;
//             Ok::<(), JobServiceError>(())
//         }
//         _ => Ok(()),
//     })?;

//     Ok(())
// }

// fn execute_export_function<T>(
//     module: &Module,
//     mut store: Store<T>,
//     instance: Instance,
// ) -> Result<(), JobServiceError> {
//     module.exports().try_for_each(|e| match e.ty() {
//         ExternType::Func(func) => {
//             let (params, mut results) = build_input_output(func, vec![foo])?;

//             instance
//                 .get_func(&mut store, e.name())
//                 .ok_or_else(|| JobServiceError::FunctionExportNotFound {
//                     func_name: e.name().to_string(),
//                 })?
//                 .call(&mut store, &params, &mut results)?;
//             Ok::<(), JobServiceError>(())
//         }
//         _ => Ok(()),
//     })?;

//     Ok(())
// }

// fn build_input_params(
//     func: FuncType,
//     raw_params: Vec<Vec<u8>>,
// ) -> Result<(Vec<Val>, Vec<Val>), CodecError> {
//     let params = func
//         .params()
//         .zip(raw_params)
//         .map(|(val_type, raw_param)| match val_type {
//             ValType::I32 => match <i32 as Decode>::decode(&mut raw_param.as_slice()) {
//                 Ok(p) => Ok(Val::I32(p)),
//                 Err(err) => Err(err),
//             },
//             _ => Ok(Val::AnyRef(None)),
//         })
//         .collect::<Result<Vec<Val>, CodecError>>()?;

//     let results = func
//         .results()
//         .map(|val_type| match val_type {
//             ValType::I32 => Val::I32(0),
//             _ => Val::AnyRef(None),
//         })
//         .collect::<Vec<Val>>();

//     Ok((params, results))
// }
