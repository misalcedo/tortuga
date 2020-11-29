use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use uuid::Uuid;
use wasmtime::{Caller, Config, Engine, ExternRef, Func, Instance, Linker, Module, Store};

pub use error::Error;

use crate::queue::{PostMark, RingBufferQueue};
use crate::wasm::Guest;

mod error;

pub struct System {
    engine: Engine,
    modules: HashMap<u128, Module>,
    identifiers: HashMap<String, u128>,
    queue: Arc<Mutex<RingBufferQueue>>,
}

enum GuestWrite {
    Guest {
        instance: Instance,
        sender: u128,
        offset: u32,
        length: u32,
    },
    Empty,
}

impl System {
    /// The capacity determines the maximum number of messages the system can buffer.
    pub fn new(capacity: usize) -> System {
        let mut config = Config::new();

        config.wasm_reference_types(true);

        let engine = Engine::new(&config);
        let queue = Arc::new(Mutex::new(RingBufferQueue::new(capacity)));
        let modules = HashMap::new();
        let identifiers = HashMap::new();

        System {
            engine,
            queue,
            modules,
            identifiers,
        }
    }

    /// Registers an intent by a given name.
    /// Overrides existing intents for the name; the internal identifier for the name remains the same.
    fn register_module(&mut self, name: &str, intent: &[u8]) -> Result<u128, Error> {
        let module = Module::new_with_name(&self.engine, intent, name)?;
        let identifier = self
            .identifiers
            .entry(name.to_string())
            .or_insert_with(|| Uuid::new_v4().as_u128());

        self.modules.insert(*identifier, module);

        Ok(*identifier)
    }

    fn module_by_identifier(&self, id: u128) -> Option<&Module> {
        self.modules.get(&id)
    }

    fn module_by_name(&self, name: &str) -> Option<u128> {
        self.identifiers.get(name).copied()
    }

    fn instance(&self, identifier: u128) -> Result<Instance, Error> {
        let module = self
            .module_by_identifier(identifier)
            .ok_or_else(|| Error::ModuleNotFound(identifier))?;

        let store = Store::new(&self.engine);
        let mut linker = Linker::new(&store);

        linker.define("system", "send", self.export_reply_send(identifier, &store))?;

        for import in module.imports() {
            if !"send".eq(import.name()) {
                // Skip any import that is not sending a message.
                eprintln!(
                    "Skipping linking of import {}/{} for guest {}.",
                    import.module(),
                    import.name(),
                    identifier
                );
                continue;
            }

            if linker.get(&import).is_some() {
                // Skip system send.
                continue;
            }

            let child = self
                .module_by_name(import.module())
                .ok_or_else(|| Error::ModuleNotFoundByName(import.module().to_string()))?;

            let send = self.export_child_send(identifier, child, &store);
            linker.define(import.module(), import.name(), send)?;
        }

        Ok(linker.instantiate(module)?)
    }

    /// Defines the system export used by guests to reply to messages.
    fn export_reply_send(&self, sender: u128, store: &Store) -> Func {
        let lock = self.queue.clone();

        Func::wrap(
            &store,
            move |caller: Caller<'_>, destination: Option<ExternRef>, offset: u32, length: u32| {
                // TODO: need to expose the array of the next position in the ring during a push operation. Otherwise, every actor instance needs to have its own buffer.

                if let Some(external_reference) = destination {
                    let recipient = external_reference.data().downcast_ref::<u128>();

                    if let Some(recipient) = recipient {
                        System::enqueue(sender, *recipient, lock.clone(), caller, offset, length);
                    } else {
                        panic!("Unable to dereference extern reference into a guest identifier.");
                    }
                } else {
                    eprintln!(
                        "Actor instance {} sent a message of {} bytes with no intended recipient.",
                        sender, length
                    );
                }
            },
        )
    }

    /// Defines the system export used by guests to send a message to a child guest.
    fn export_child_send(&self, sender: u128, recipient: u128, store: &Store) -> Func {
        let lock = self.queue.clone();

        Func::wrap(
            &store,
            move |caller: Caller<'_>, offset: u32, length: u32| {
                System::enqueue(sender, recipient, lock.clone(), caller, offset, length)
            },
        )
    }

    fn enqueue(
        sender: u128,
        recipient: u128,
        lock: Arc<Mutex<RingBufferQueue>>,
        caller: Caller,
        offset: u32,
        length: u32,
    ) {
        // TODO: need to expose the array of the next position in the ring during a push operation. Otherwise, every actor instance needs to have its own buffer.

        let mut buffer = [0u8; 8192];
        caller.read(offset, &mut buffer[..length as usize]).unwrap();

        let mut queue = match lock.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        queue
            .push(PostMark::new(sender, recipient), &buffer[..length as usize])
            .unwrap();
    }

    /// Registers a guest intent with the system under a given name.
    pub fn register(&mut self, name: &str, intent: &[u8]) -> Result<u128, Error> {
        let identifier = self.register_module(name, intent)?;

        Ok(identifier)
    }

    //TODO: Need to add interrupts to stop running code that runs too long.
    /// Queues a message in the system.
    pub fn distribute(&self, recipient: u128, sender: u128, message: &[u8]) -> Result<(), Error> {
        let post_mark = PostMark::new(sender, recipient);

        let mut queue = match self.queue.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        queue.push(post_mark, message).map_err(Error::Wrapped)
    }

    /// Processes a single message from the queue.
    /// Returns false if the queue is empty, true otherwise.
    pub fn run_step(&self) -> Result<bool, Error> {
        if let GuestWrite::Guest {
            instance,
            sender,
            offset,
            length,
        } = self.send()?
        {
            instance.receive(sender, offset, length)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Writes the message to the guest memory.
    /// Does not trigger the guest to receive the message.
    fn send(&self) -> Result<GuestWrite, Error> {
        let mut queue = match self.queue.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if queue.is_empty() {
            return Ok(GuestWrite::Empty);
        }

        let (post_mark, message) = queue.pop().unwrap();

        let guest = self.instance(post_mark.recipient())?;

        let length = message.len().try_into()?;
        let offset = guest.allocate(length)?;

        guest.write(offset, message.as_ref())?;

        Ok(GuestWrite::Guest {
            instance: guest,
            sender: post_mark.sender(),
            offset,
            length,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::PostMark;
    use crate::system::System;

    #[test]
    fn usability() {
        let mut system = System::new(1);

        let ping = system
            .register("ping", include_bytes!("../../examples/ping.wat"))
            .unwrap();
        let pong = system
            .register("pong", include_bytes!("../../examples/pong.wat"))
            .unwrap();

        system.distribute(ping, 0, b"Pong!\n").unwrap();

        for i in 1..10 {
            system.run_step().unwrap();
        }

        let mut guard = system.queue.lock().unwrap();
        let envelope = guard.pop().unwrap();

        assert_eq!(envelope.0, PostMark::new(ping, pong));
        assert_eq!(envelope.1.as_ref(), b"Ping!\n");
    }

    #[test]
    fn partial_register() {
        let mut system = System::new(1);

        let ping = system
            .register("ping", include_bytes!("../../examples/ping.wat"))
            .unwrap();

        system.distribute(ping, 0, b"Pong!\n").unwrap();

        let result = system.run_step();
        assert!(result.is_err());

        let queue = system.queue.lock().unwrap();
        assert!(queue.is_empty());
    }
}
