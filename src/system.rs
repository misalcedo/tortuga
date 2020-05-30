use crate::actor::Actor;
use crate::errors::{Error, Result};
use crate::reference::Reference;
use crate::wasm::{Module, Store};
use std::collections::HashMap;

pub struct System {
    actors: HashMap<Reference, Actor>,
}

impl System {
    pub fn new() -> System {
        System {
            actors: HashMap::new(),
        }
    }

    pub fn register(&mut self, bytes: &[u8]) -> Result<Reference> {
        let module = Module::new(bytes)?;
        let actor = Actor::new(module);
        let reference = actor.reference();

        self.actors.insert(reference, actor);

        Ok(reference)
    }

    fn actor(&self, actor: &Reference) -> Option<&Actor> {
        self.actors.get(actor)
    }

    pub fn send(&mut self, actor: Reference, message: &[u8]) -> Result<()> {
        self.actor(&actor).ok_or(Error::NoSuchActor)?.send(message)
    }

    pub fn run(&mut self, actor: Reference) -> Result<()> {
        let actor = self.actors.get(&actor).ok_or(Error::NoSuchActor)?;
        let instance = actor.module().instantiate(&Store::new()?)?;
        let message = actor.receive()?;

        instance.receive(&message)
    }
}
