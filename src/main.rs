// src/main.rs
use wasmtime::{Engine, Module, Store, Instance, TypedFunc};
use anyhow::Result;

fn main() -> Result<()> {
    // 1. Create an Engine
    let engine = Engine::default();

    // 2. Load or embed a wasm module — here we’ll embed a trivial WAT for illustration
    let wat = r#"
        (module
          (func (export "run") (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add))
    "#;
    let module = Module::new(&engine, wat)?;  // built‑in WAT support. :contentReference[oaicite:1]{index=1}

    // 3. Create a Store
    let mut store = Store::new(&engine, ());

    // 4. Instantiate
    let instance = Instance::new(&mut store, &module, &[])?;

    // 5. Get exported function
    let func: TypedFunc<(i32,i32), i32> = instance.get_typed_func(&mut store, "run")?;

    // 6. Call it
    let result = func.call(&mut store, (7, 8))?;
    println!("Result: {}", result);

    Ok(())
}
