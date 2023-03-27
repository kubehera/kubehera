use wasmtime::*;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::sync::WasiCtxBuilder;
use std::error::Error;
use std::marker;

pub struct WasmInstance<T>{
    pub store: Store<WasiCtx>,
    memory: Memory,
    input_pointer: i32,
    output_pointer: i32,
    run_handler: TypedFunc<(i32,i32,i32,i32),i32>,
    _marker: marker::PhantomData<fn() -> T>,
}


impl<T> WasmInstance<T> {
    pub fn new() -> WasmInstance<T>{
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

        let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_file(&engine, "target/wasm32-wasi/debug/capable_wasm.wasm").unwrap();
        let instance = linker.instantiate(&mut store, &module).unwrap();
        linker.instance(&mut store, "", instance).unwrap();

        let memory = instance.get_memory(&mut store,"memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export")).unwrap();
        let malloc = instance.get_typed_func::<i32, i32>(&mut store, "my_alloc").unwrap();
        let run_handler = instance.get_typed_func::<(i32,i32,i32,i32), i32>(&mut store, "run_handler").unwrap();
        let input_pointer = malloc.call(& mut store,0).unwrap();
        let output_pointer = malloc.call(& mut store,65536).unwrap();
        WasmInstance {
            store,
            memory,
            input_pointer,
            output_pointer,
            run_handler,
            _marker: marker::PhantomData,
        }
    }
    
    pub fn write_data_to_wasm(&mut self,input : &[u8]){
        let input_size:i32 = input.len().try_into().unwrap();
        let _ = self.memory.grow(&mut self.store, input_size.try_into().unwrap());
        let input_offset = self.input_pointer.try_into().unwrap();
        let _ = self.memory.write(&mut self.store,input_offset,input);
    }

    pub fn read_from_wasm(&self, mut output_size: usize)->String{
        let output_offset = self.output_pointer.try_into().unwrap();
        let mut output_buffer = [0u8; 65536];
        if output_size >= 65535{
            output_size = 65535
        }
        self.memory.read(&self.store, output_offset, &mut output_buffer).unwrap();
        format!("{}",std::str::from_utf8(&output_buffer[..output_size]).unwrap())
    }

    pub fn run(&mut self,extra_fields: i32,input_size:i32) -> Result<i32,Box<dyn Error>> {
        self.run_handler.call(&mut self.store, (extra_fields,self.input_pointer,self.output_pointer,input_size))?;

        Ok(0)
    }
}