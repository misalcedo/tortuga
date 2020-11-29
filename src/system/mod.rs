use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use uuid::Uuid;
use wasmtime::{Caller, Config, Engine, ExternRef, Func, Instance, Module, Store};

pub use error::Error;

use crate::queue::{PostMark, RingBufferQueue};
use crate::wasm::Guest;

mod error;

pub struct System {
    engine: Engine,
    modules: HashMap<u128, Module>,
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
    fn new(capacity: usize) -> System {
        let mut config = Config::new();

        config.wasm_reference_types(true);

        let engine = Engine::new(&config);
        let queue = Arc::new(Mutex::new(RingBufferQueue::new(capacity)));
        let modules = HashMap::new();

        System { engine, queue, modules }
    }

    fn register_module(&mut self, name: &str, id: u128, intent: &[u8]) -> Result<Option<Module>, Error> {
        let module = Module::new_with_name(&self.engine, intent, name)?;

        Ok(self.modules.insert(id, module))
    }

    fn module(&self, id: u128) -> Option<&Module> {
        self.modules.get(&id)
    }

    fn instance(&self, identifier: u128) -> Result<Instance, Error> {
        let module = self.module(identifier)
            .ok_or_else(|| Error::ModuleNotFound(identifier))?;

        let store = Store::new(&self.engine);
        let lock = self.queue.clone();
        let send = Func::wrap(
            &store,
            move |caller: Caller<'_>, destination: Option<ExternRef>, offset: u32, length: u32| {
                // TODO: need to expose the array of the next position in the ring during a push operation. Otherwise, every actor instance needs to have its own buffer.
                let mut buffer = [0u8; 8192];
                caller.read(offset, &mut buffer[..length as usize]).unwrap();

                if let Some(external_reference) = destination {
                    let recipient = external_reference.data().downcast_ref::<u128>();

                    let mut queue = match lock.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => poisoned.into_inner(),
                    };
                    queue
                        .push(
                            PostMark::new(identifier, *recipient.unwrap()),
                            &buffer[..length as usize],
                        )
                        .unwrap();
                } else {
                    eprintln!(
                        "Actor instance {} sent a message of {} bytes with no intended recipient.",
                        identifier, length
                    );
                }
            },
        );

        Ok(Instance::new(&store, module, &[send.into()])?)
    }

    /// Registers a guest intent with the system under a given name.
    pub fn register(&mut self, name: &str, intent: &[u8]) -> Result<u128, Error> {
        let identifier = Uuid::new_v4().as_u128();

        self.register_module(name, identifier, intent)?;

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

        let ping = system.register("ping", include_bytes!("ping.wat")).unwrap();
        let pong = system.register("pong", include_bytes!("pong.wat")).unwrap();

        system.distribute(ping, pong, b"Pong!\n").unwrap();

        for i in 1..10 {
            system.run_step().unwrap();
        }

        let mut guard = system.queue.lock().unwrap();
        let envelope = guard.pop().unwrap();

        assert_eq!(envelope.0, PostMark::new(ping, pong));
        assert_eq!(envelope.1.as_ref(), b"Ping!\n");
    }
}
