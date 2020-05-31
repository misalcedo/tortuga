use crate::errors::{Error, Result};
use crate::wasm::{Module, Store};
use crate::Envelope;
use std::collections::HashMap;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

/// An actor system. A system can host multiple actors.
pub struct System {
    actors: HashMap<Uuid, Module>,
    sender: UnboundedSender<Envelope>,
    receiver: UnboundedReceiver<Envelope>,
}

impl System {
    /// Creates an empty actor system.
    pub fn new() -> System {
        let (sender, receiver) = unbounded_channel();

        System {
            actors: HashMap::new(),
            sender,
            receiver,
        }
    }

    /// Registers an actor with the given intent.
    pub fn register(&mut self, intent: &[u8]) -> Result<Uuid> {
        let module = Module::new(intent)?;
        let reference = Uuid::new_v4();

        self.actors.insert(reference, module);

        Ok(reference)
    }

    fn actor(&self, actor: &Uuid) -> Option<&Module> {
        self.actors.get(actor)
    }

    pub fn send(&mut self, envelope: Envelope) -> Result<()> {
        self.actor(&envelope.to())
            .ok_or(Error::NoSuchActor)?
            .send(envelope.message())
    }

    /// Runs the actor.
    pub fn run(&mut self, actor: Uuid) -> Result<()> {
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

        let Uuid = system.register(ECHO_WAT_MODULE.as_bytes()).unwrap();
        let other_Uuid = system.register(ECHO_WAT_MODULE.as_bytes()).unwrap();

        assert_ne!(Uuid, other_Uuid);
    }
}
