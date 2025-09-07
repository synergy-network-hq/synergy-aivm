use anyhow::Result;
use wasmtime::*;

pub fn run_wasm(file: &str) -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, file)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    println!("WASM module {} loaded successfully", file);
    Ok(())
}
