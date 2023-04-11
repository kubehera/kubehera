use std::error::Error;
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<(),Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine,"../../../target/wasm32-unknown-unknown/debug/hello_world.wasm")?;
    let mut store = Store::new(&engine,());
    let linker = Linker::new(&engine);
    let instance = linker.instantiate(&mut store,&module)?;
    let memory = instance.get_memory(&mut store,"memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export")).unwrap();


//Write Vec[u8] to wasm
    let write_buffer = "Licheng".as_bytes();
    let init_write = instance.get_typed_func::<i32, ()>(&mut store, "init_write").unwrap();
    init_write.call(&mut store,write_buffer.len().try_into().unwrap())?;
    let get_write_addr = instance.get_typed_func::<(), i32>(&mut store, "get_write_addr").unwrap();
    let write_addr = get_write_addr.call(&mut store,()).unwrap();
    memory.write(&mut store,write_addr.try_into().unwrap(),write_buffer)?;

//greet handler
    let greet = instance.get_typed_func::<(), ()>(&mut store, "greet").unwrap();
    let _ = greet.call(&mut store,()).unwrap();

//Read String from wasm
    let get_string_len = instance.get_typed_func::<(), i32>(&mut store, "get_string_len").unwrap();
    let len = get_string_len.call(&mut store,()).unwrap();

    let mut read_buffer = vec![0u8; len.try_into().unwrap()];

    let get_string_addr = instance.get_typed_func::<(), i32>(&mut store, "get_string_addr").unwrap();
    let read_addr = get_string_addr.call(&mut store,()).unwrap();
    memory.read(&mut store, read_addr.try_into().unwrap(), &mut read_buffer).unwrap();
    println!("{}",std::str::from_utf8(&read_buffer[..]).unwrap());

    Ok(())
}