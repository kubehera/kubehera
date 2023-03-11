use std::error::Error;
use anyhow::Result;
use wasmtime::*;

fn main() -> Result<(),Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine,"../hello-world/target/wasm32-unknown-unknown/debug/hello_world.wasm")?;
    let mut store = Store::new(&engine,());
    let instance = Instance::new(&mut store, &module,&[])?;
    //let hellow = instance.get_typed_func::<(),()>(&mut store,"print_hello")?;
    //hellow.call(&mut store,())?;
    let memory = instance.get_memory(&mut store,"memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    //let get_input_buf_fn = instance.get_typed_func::<(), i32>(&mut store, "get_input_buf")?;
    let malloc_fn = instance.get_typed_func::<i32, i32>(&mut store, "my_alloc")?;
    let greet_fn = instance.get_typed_func::<(i32,i32), ()>(&mut store, "greet")?;
    let output_fn = instance.get_typed_func::<(i32,i32), ()>(&mut store, "output")?;
    //let free_fn = instance.get_typed_func::<(), ()>(&mut store, "input_buf_free")?;
    //let memcpy_fn = instance.get_typed_func::<(i32,i32), ()>(&mut store, "input_buf_memcpy")?;


    let helloworld = b"hello world";
    let size:i32 =  helloworld.len().try_into().unwrap();
    let pointer = malloc_fn.call(&mut store,size)?;
   // println!("hello1:{}",get_input_buf_fn.call(&mut store,())?.to_string());
    //let hw_ptr = helloworld.as_ptr();
    //memcpy_fn.call(&mut store,(1,1))?;
    //println!("hello2:{}",get_input_buf_fn.call(&mut store,())?);


    let offset = pointer.try_into().unwrap();
    memory.write(&mut store,offset, helloworld)?;

    memory.grow(&mut store,20)?;
    greet_fn.call(&mut store,(pointer,size))?;
    let mut buffer = [0u8; 16];

    
    memory.read(&store, offset, &mut buffer)?;


    println!("{:#?}",&buffer[0..15].to_owned());
    

    let output_pointer = malloc_fn.call(&mut store,size)?;

    let output_offset = output_pointer.try_into().unwrap();

    output_fn.call(&mut store,(output_pointer,size))?;
    let mut output_buffer = [0u8; 16];

    
    memory.read(&store, output_offset, &mut output_buffer)?;


    println!("{:#?}",&buffer[0..15].to_owned());
    println!("{:#?}",&output_buffer.to_owned());
   // buf := memory::data();


    //memset_fn.call(&mut store,(0,1))?;
    //println!("{}",get_input_buf_fn.call(&mut store,())?);
    //memset_fn.call(&mut store,(1,2))?;
    //println!("{}",get_input_buf_fn.call(&mut store,())?);
    //memset_fn.call(&mut store,(2,3))?;
    //println!("{}",get_input_buf_fn.call(&mut store,())?);

    //free_fn.call(&mut store,())?;

    Ok(())
}