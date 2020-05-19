use crate::broker::Broker;
use crate::errors::Error;
use crate::reference::Reference;
use std::borrow::Borrow;
use std::collections::HashMap;
use wasmer_runtime::{compile, func, imports, Array, Ctx, Func, Instance, Module, WasmPtr};

pub struct System {
    modules: HashMap<Reference, Module>,
    broker: Broker,
}

impl System {
    pub fn new() -> System {
        System {
            modules: HashMap::new(),
            broker: Broker::new(),
        }
    }

    pub fn register(&mut self, bytes: &[u8]) -> Result<Reference, Error> {
        let reference = Reference::new();
        let module = new_behavior(bytes)?;

        self.modules.insert(reference, module);

        Ok(reference)
    }

    fn new_instance(&self, actor: Reference) -> Result<Instance, Error> {
        let module = self.modules.get(&actor).ok_or(Error::NoSuchActor)?;
        let imports = imports! {
            "system" => {
                "send" => func!(send),
            },
        };

        module.instantiate(&imports).map_err(Error::Unkown)
    }

    pub fn send(&mut self, actor: Reference, message: &[u8]) -> Result<(), Error> {
        if self.modules.contains_key(&actor) {
            self.broker.send(actor, message);
            Ok(())
        } else {
            Err(Error::NoSuchActor)
        }
    }

    pub fn run(&mut self, actor: Reference) -> Result<Vec<Result<(), Error>>, Error> {
        let instance = self.new_instance(actor)?;
        let messages = self.broker.read(actor);

        Ok(messages.map(|message| instance.receive(&message)).collect())
    }
}

fn new_behavior(bytes: &[u8]) -> Result<Module, Error> {
    let module = wat::parse_bytes(bytes)?;

    compile(module.borrow()).map_err(Error::Compile)
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

pub fn send(source: &mut Ctx, address: WasmPtr<u8, Array>, length: u32) -> Result<(), Error> {
    let bytes = source.read(address, length)?;
    let value = std::str::from_utf8(&bytes)?;

    println!(
        "Address: {:?}, Length: {}, Bytes: {:?}, Value: {:?}",
        address, length, bytes, value
    );

    Ok(())
}

//
// pub struct System {
//     pub dc: DistributionCenter,
//     pub reference: Reference,
//     pub import: ImportObject,
//     pub instances: HashMap<Reference, Instance>,
// }
//
// impl System {
//     fn new() -> System {
//         let import = imports! {
//             "system" => {
//                 "print" => func!(print),
//                 "send" => func!(send),
//             },
//         };
//
//         System {
//             dc: DistributionCenter::new(),
//             reference: Reference::new(),
//             instances: HashMap::new(),
//             import,
//         }
//     }
//
//     pub fn send(&mut self, to: Reference, message: u32) -> Result<(), &'static str> {
//         if let Some(instance) = self.instances.get_mut(&to) {
//             let mut context = instance.context_mut();
//             let memory = context.memory(0);
//             let pointer: WasmPtr<u32> = WasmPtr::new(0);
//
//             let cell = pointer
//                 .deref(memory)
//                 .ok_or("Unable to dereference the memory pointer to write the message.")?;
//
//             cell.set(message);
//
//             instance
//                 .call("receive", &[])
//                 .map_err(|_| "Unabe to trigger the actor's behavior.")?;
//
//             Ok(())
//         } else {
//             Err("No such actor found.")
//         }
//     }
// }
