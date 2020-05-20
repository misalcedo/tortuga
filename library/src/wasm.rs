use wasmer_runtime::{compile, func, imports, Array, Ctx, Func, WasmPtr};

use crate::errors::{Error, Result};

pub(crate) struct Module(wasmer_runtime::Module);

impl Module {
    pub fn new(bytes: &[u8]) -> Result<Module> {
        let binary = wat::parse_bytes(bytes)?;
        let module = compile(&binary)?;

        Ok(Module(module))
    }

    pub fn instantiate(&self, store: &Store) -> Result<Instance> {
        let instance = self.0.instantiate(&store.0)?;

        Ok(Instance(instance))
    }
}

pub(crate) struct Instance(wasmer_runtime::Instance);

impl Instance {
    pub fn receive(&self, message: &[u8]) -> Result<()> {
        let memory = self.0.context().memory(0);
        let message_buffer: WasmPtr<u8, Array> = WasmPtr::new(0);
        let length = message.len() as u32;

        // We deref our WasmPtr to get a &[Cell<u8>]
        let memory_writer = message_buffer.deref(memory, 0, length).unwrap();

        for i in 0..message.len() {
            memory_writer[i].set(message[i]);
        }

        // Let's call the exported function that concatenates a phrase to our string.
        let receive: Func<(WasmPtr<u8, Array>, u32), ()> = self
            .0
            .exports
            .get("receive")
            .expect("receive function not defined.");

        receive.call(message_buffer, length).map_err(Error::Runtime)
    }
}

pub struct Store(wasmer_runtime::ImportObject);

impl Store {
    pub(crate) fn new() -> Result<Store> {
        let imports = imports! {
            "system" => {
                "send" => func!(send),
            }
        };

        Ok(Store(imports))
    }
}

fn send(source: &mut Ctx, address: WasmPtr<u8, Array>, length: u32) -> Result<()> {
    let bytes = read(source, address, length)?;
    let value = std::str::from_utf8(&bytes)?;

    println!(
        "Address: {:?}, Length: {}, Bytes: {:?}, Value: {:?}",
        address, length, bytes, value
    );

    Ok(())
}

fn read(context: &mut Ctx, address: WasmPtr<u8, Array>, length: u32) -> Result<Vec<u8>> {
    let memory = context.memory(0);
    let cells = address
        .deref(memory, 0, length)
        .ok_or(Error::PointerReference)?;
    let bytes: Vec<u8> = cells.iter().map(|cell| cell.get()).collect();

    Ok(bytes)
}
