use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>>{
    let engine = Engine::default();
    let module = Module::from_file(&engine, "../gcd.wat")?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let gcd = instance.get_typed_func::<(i32, i32), i32, _> (&mut store, "gcd")
        .expect("`gcd` was not an exported function");
    let result = gcd.call(&mut store, (27, 6))?;
    println!("GCD 27 and 6: {:?}", result);
    Ok(())
}
