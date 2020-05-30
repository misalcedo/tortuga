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
            .send(envelope.message())
    }

    /// Runs the actor.
    pub fn run(&mut self, actor: Reference) -> Result<()> {
        let actor = self.actors.get(&actor).ok_or(Error::NoSuchActor)?;
        let instance = actor.module().instantiate(&Store::new()?)?;
        let message = actor.receive()?;

        instance.receive(&message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ECHO_WAT_MODULE: &'static str = r#"(module
        (import "system" "send" (func $send (param i32 i32)))
      
        (memory 1)
      
        (func (export "receive") (param $address i32) (param $length i32)
          (call $send (local.get $address) (local.get $length))
        )
      )"#;

    #[test]
    fn register_an_actor() {
        let mut system = System::new();

        let reference = system.register(ECHO_WAT_MODULE.as_bytes()).unwrap();
        let other_reference = system.register(ECHO_WAT_MODULE.as_bytes()).unwrap();

        assert_ne!(reference, other_reference);
    }
}