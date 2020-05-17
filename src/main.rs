mod errors;

use wasmer_runtime::{imports, compile, validate, func, Ctx, Func, Module, ImportObject, Instance, WasmPtr, Array};
use crate::errors::WasmError;

// Our entry point to our application
fn main() -> Result<(), WasmError> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../examples/echo.wasm");

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {
        "system" => {
            "send" => func!(send),
        },
    };

    let behavior = new_behavior(wasm_bytes)?;

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let continuation = new_continuation(&behavior, &import_object)?;

    continuation.receive("Hello, World!".as_bytes())?;

    // Return OK since everything executed successfully!
    Ok(())
}

fn send(source: &mut impl Source, address: WasmPtr<u8, Array>, length: u32) -> Result<(), WasmError> {
    let bytes = source.read(address, length)?;
    let value = std::str::from_utf8(&bytes)?;

    println!("Address: {:?}, Length: {}, Value: {:?}", address, length, value);

    Ok(())
}

fn new_behavior(module: &[u8]) -> Result<Module, WasmError> {
    if !validate(module) {
        return Err(WasmError::Invalid);
    }

    compile(module).map_err(WasmError::Compile)
}

fn new_continuation(behavior: &Module, imports: &ImportObject) -> Result<impl Continuation, WasmError> {
    behavior.instantiate(imports).map_err(WasmError::Unkown)
}

trait Continuation {
    fn receive(&self, message: &[u8]) -> Result<(), WasmError>;
}

trait Source {
    fn read(&self, address: WasmPtr<u8, Array>, length: u32) -> Result<Vec<u8>, WasmError>;
}

impl Continuation for Instance {
    fn receive(&self, message: &[u8]) -> Result<(), WasmError> {
        let memory = self.context().memory(0);
        let message_buffer: WasmPtr<u8, Array> = WasmPtr::new(0);
        let length = message.len() as u32;

        // We deref our WasmPtr to get a &[Cell<u8>]
        let memory_writer = message_buffer
            .deref(memory, 0, length)
            .unwrap();

        for i in 0..message.len() {
            memory_writer[i].set(message[i]);
        }

        // Let's call the exported function that concatenates a phrase to our string.
        let receive: Func<(WasmPtr<u8, Array>, u32), ()> = self
            .exports
            .get("receive")
            .expect("receive function not defined.");

        receive.call(message_buffer, length)?;

        Ok(())
    }
}

impl Source for Ctx {
    fn read(&self, address: WasmPtr<u8, Array>, length: u32) -> Result<Vec<u8>, WasmError> {
        let memory = self.memory(0);
        let cells = address.deref(memory, 0, length).ok_or(WasmError::PointerReference)?;
        let bytes: Vec<u8> = cells.iter().map(|cell| cell.get()).collect();
        
        Ok(bytes) 
    }
}