use std::collections::HashMap;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use crate::errors::{Error, Result};
use crate::wasm::{Module, Store};
use crate::Envelope;

const PAGE_SIZE: usize = 64000;

/// An actor system. A system can host multiple actors.
pub struct System {
    actors: HashMap<Uuid, Module>,
    pub sender: UnboundedSender<Envelope>,
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

    pub async fn run(&mut self) {
        while let Some(envelope) = self.receiver.recv().await {
            if let Err(e) = self.process(envelope) {
                eprintln!("Encountered an error processing envelope: {}", e);
            } else {
                println!("Processed a message.");
            }
        }
    }

    fn process(&self, envelope: Envelope) -> Result<()> {
        let module = self.actor(&envelope.to()).ok_or(Error::NoSuchActor)?;
        let store = Store::new(self.sender.clone())?;
        let instance = module.instantiate(&store)?;
        let mut buffer = [0u8; PAGE_SIZE];

        let payload = postcard::to_slice(&envelope, &mut buffer)?;

        instance.receive(payload)
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
