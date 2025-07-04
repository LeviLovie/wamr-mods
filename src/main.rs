use std::{ffi::c_void, path::PathBuf};
use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
    wasi_context::WasiCtxBuilder, RuntimeError,
};

extern "C" fn log(val: WasmValue) {
    println!("println: {:#?}", val);
}

fn main() -> Result<(), RuntimeError> {
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("host_log", log as *mut c_void)
        .build()?;

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("mod.wasm");
    let mut module = Module::from_file(&runtime, d.as_path())?;

    let wasi_ctx = WasiCtxBuilder::new()
        .set_pre_open_path(vec!["."], vec![])
        .build();

    module.set_wasi_context(wasi_ctx);

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    let function = Function::find_export_func(&instance, "gcd")?;

    let params: Vec<WasmValue> = vec![WasmValue::I32(9), WasmValue::I32(27)];
    let result = function.call(&instance, &params)?;
    assert_eq!(*result.get(0).unwrap(), WasmValue::I32(9));

    Ok(())
}
