use crate::errors::Error;
use crate::messages::Broker;
use crate::reference::Reference;
use crate::wasm::{Instance, Module, Store};
use std::collections::HashMap;

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
        let module = Module::new(bytes)?;

        self.modules.insert(reference, module);

        Ok(reference)
    }

    fn new_instance(&self, actor: Reference) -> Result<Instance, Error> {
        let module = self.modules.get(&actor).ok_or(Error::NoSuchActor)?;

        module.instantiate(&Store::new()?)
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
