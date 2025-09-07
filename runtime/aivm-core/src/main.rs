mod transcript;
mod wasm_runner;

fn main() -> anyhow::Result<()> {
    println!("Synergy-AIVM Core starting...");

    // Example: initialize WASM runner
    let wasm_file = "examples/sample_model.wasm";
    wasm_runner::run_wasm(wasm_file)?;

    Ok(())
}
