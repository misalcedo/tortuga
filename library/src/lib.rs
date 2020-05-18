pub mod errors;

use crate::errors::Error;
use wasmer_runtime::{
    compile, func, imports, validate, Array, Ctx, Func, ImportObject, Instance, Module, WasmPtr,
};

pub struct System();

impl System {
    pub fn new() -> System {
        System()
    }

    pub fn run(&self, wasm_bytes: &[u8], message: &[u8]) -> Result<(), Error> {
        let imports = imports! {
            "system" => {
                "send" => func!(send),
            },
        };

        let behavior = new_behavior(wasm_bytes)?;
        let continuation = new_continuation(&behavior, &imports)?;

        continuation.receive(message)?;

        Ok(())
    }
}

pub fn send(source: &mut Ctx, address: WasmPtr<u8, Array>, length: u32) -> Result<(), Error> {
    let bytes = source.read(address, length)?;
    let value = std::str::from_utf8(&bytes)?;

    println!(
        "Address: {:?}, Length: {}, Bytes: {:?}, Value: {:?}",
        address, length, bytes, value
    );

    Ok(())
}

fn new_behavior(module: &[u8]) -> Result<Module, Error> {
    if !validate(module) {
        return Err(Error::Invalid);
    }

    compile(module).map_err(Error::Compile)
}

fn new_continuation(behavior: &Module, imports: &ImportObject) -> Result<impl Continuation, Error> {
    behavior.instantiate(imports).map_err(Error::Unkown)
}

trait Continuation {
    fn receive(&self, message: &[u8]) -> Result<(), Error>;
}

trait Source {
    fn read(&self, address: WasmPtr<u8, Array>, length: u32) -> Result<Vec<u8>, Error>;
}

impl Continuation for Instance {
    fn receive(&self, message: &[u8]) -> Result<(), Error> {
        let memory = self.context().memory(0);
        let message_buffer: WasmPtr<u8, Array> = WasmPtr::new(0);
        let length = message.len() as u32;

        // We deref our WasmPtr to get a &[Cell<u8>]
        let memory_writer = message_buffer.deref(memory, 0, length).unwrap();

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
    fn read(&self, address: WasmPtr<u8, Array>, length: u32) -> Result<Vec<u8>, Error> {
        let memory = self.memory(0);
        let cells = address
            .deref(memory, 0, length)
            .ok_or(Error::PointerReference)?;
        let bytes: Vec<u8> = cells.iter().map(|cell| cell.get()).collect();

        Ok(bytes)
    }
}
