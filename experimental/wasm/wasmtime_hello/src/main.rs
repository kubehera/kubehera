use std::error::Error;
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<(),Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine,"../hello-world/target/wasm32-unknown-unknown/debug/hello_world.wasm")?;
    let mut store = Store::new(&engine,());
    let instance = Instance::new(&mut store, &module,&[])?;
    let memory = instance.get_memory(&mut store,"memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    let malloc_fn = instance.get_typed_func::<i32, i32>(&mut store, "my_alloc")?;
    let greet_fn = instance.get_typed_func::<(i32,i32), ()>(&mut store, "greet")?;
    let output_fn = instance.get_typed_func::<(i32,i32), ()>(&mut store, "output")?;


    let helloworld = b"hello world    ";
    let size:i32 =  helloworld.len().try_into().unwrap();
    let pointer = malloc_fn.call(&mut store,size)?;

    let offset = pointer.try_into().unwrap();
    memory.write(&mut store,offset, helloworld)?;

    memory.grow(&mut store,20)?;
    let mut oldbuffer = [0u8; 15];
    memory.read(&store, offset, &mut oldbuffer)?;
    println!("{}",std::str::from_utf8(&oldbuffer).unwrap());

    greet_fn.call(&mut store,(pointer,size))?;
    let mut buffer = [0u8; 15];
    memory.read(&store, offset, &mut buffer)?;

    let output_pointer = malloc_fn.call(&mut store,size)?;
    let output_offset = output_pointer.try_into().unwrap();
    output_fn.call(&mut store,(output_pointer,size))?;
    let mut output_buffer = [0u8; 4];
    
    memory.read(&store, output_offset, &mut output_buffer)?;


    println!("{}",std::str::from_utf8(&buffer).unwrap());
    println!("{}",std::str::from_utf8(&output_buffer).unwrap());

    Ok(())
}