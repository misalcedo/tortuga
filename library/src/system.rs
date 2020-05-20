use crate::errors::{Error, Result};
use crate::messages::Broker;
use crate::reference::Reference;
use crate::wasm::{Module, Store};
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

    pub fn register(&mut self, bytes: &[u8]) -> Result<Reference> {
        let reference = Reference::new();
        let module = Module::new(bytes)?;

        self.modules.insert(reference, module);

        Ok(reference)
    }

    pub fn send(&mut self, actor: Reference, message: &[u8]) -> Result<()> {
        if self.modules.contains_key(&actor) {
            self.broker.send(actor, message);
            Ok(())
        } else {
            Err(Error::NoSuchActor)
        }
    }

    pub fn run(&mut self, actor: Reference) -> Result<Vec<Result<()>>> {
        let module = self.modules.get(&actor).ok_or(Error::NoSuchActor)?;
        let instance = module.instantiate(&Store::new()?)?;
        let messages = self.broker.read(actor);

        Ok(messages.map(|message| instance.receive(&message)).collect())
    }
}
