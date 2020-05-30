use crate::actor::Actor;
use crate::errors::{Error, Result};
use crate::reference::Reference;
use crate::wasm::{Module, Store};
use crate::Envelope;
use std::collections::HashMap;

/// An actor system. A system can host multiple actors.
pub struct System {
    actors: HashMap<Reference, Actor>,
}

impl System {
    /// Creates an empty actor system.
    pub fn new() -> System {
        System {
            actors: HashMap::new(),
        }
    }

    /// Registers an actor with the given intent.
    pub fn register(&mut self, intent: &[u8]) -> Result<Reference> {
        let module = Module::new(intent)?;
        let actor = Actor::new(module);
        let reference = actor.reference();

        self.actors.insert(reference, actor);

        Ok(reference)
    }

    fn actor(&self, actor: &Reference) -> Option<&Actor> {
        self.actors.get(actor)
    }

    pub fn send(&mut self, envelope: Envelope) -> Result<()> {
        self.actor(&envelope.to())
            .ok_or(Error::NoSuchActor)?
            .send(envelope.message)
    }

    /// Runs the actor.
    pub fn run(&mut self, actor: Reference) -> Result<()> {
        let actor = self.actors.get(&actor).ok_or(Error::NoSuchActor)?;
        let instance = actor.module().instantiate(&Store::new()?)?;
        let message = actor.receive()?;

        instance.receive(&message)
    }
}
