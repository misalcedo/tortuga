use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use uuid::Uuid;
use wasmtime::{Caller, Config, Engine, Extern, ExternRef, Func, Instance, Linker, Module, Store};

pub use error::Error;

use crate::queue::{PostMark, RingBufferQueue};
use crate::wasm::Guest;

mod error;

pub struct System {
    linker: Linker,
    queue: Arc<Mutex<RingBufferQueue>>,
}

enum GuestWrite {
    Guest {
        id: String,
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
        let store = Store::new(&engine);
        let linker = Linker::new(&store);
        let queue = Arc::new(Mutex::new(RingBufferQueue::new(capacity)));

        System { linker, queue }
    }

    fn store(&self) -> &Store {
        &self.linker.store()
    }

    fn engine(&self) -> &Engine {
        self.store().engine()
    }

    fn module(&self, intent: &[u8]) -> Result<Module, Error> {
        Ok(Module::new(self.engine(), intent)?)
    }

    fn instance(&self, module: &Module, imports: &[Extern]) -> Result<Instance, Error> {
        Ok(Instance::new(&self.store(), module, imports)?)
    }

    pub fn register(&mut self, intent: &[u8]) -> Result<u128, Error> {
        let module = self.module(intent).unwrap();
        let uuid = Uuid::new_v4();
        let id = uuid.as_u128();

        let lock = self.queue.clone();

        let send = Func::wrap(
            self.store(),
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
                            PostMark::new(id, *recipient.unwrap()),
                            &buffer[..length as usize],
                        )
                        .unwrap();
                } else {
                    eprintln!(
                        "Actor instance {} sent a message of {} bytes with no intended recipient.",
                        id, length
                    );
                }
            },
        );

        let instance = self.instance(&module, &[send.into()])?;
        self.linker.instance(uuid.to_string().as_str(), &instance)?;

        Ok(id)
    }

    //TODO: Need to add interrupts to stop running code that runs too long.
    pub fn distribute(&self, recipient: u128, sender: u128, message: &[u8]) -> Result<(), Error> {
        let post_mark = PostMark::new(sender, recipient);

        let mut queue = match self.queue.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        queue.push(post_mark, message).map_err(Error::Wrapped)
    }

    // TODO: return different values for empty queue versus the queue with an envelope.
    pub fn run_step(&self) -> Result<(), Error> {
        if let GuestWrite::Guest {
            id,
            sender,
            offset,
            length,
        } = self.send()?
        {
            let guest = &(id.as_str(), &self.linker);

            Ok(guest.receive(sender, offset, length)?)
        } else {
            Ok(())
        }
    }

    fn send(&self) -> Result<GuestWrite, Error> {
        let mut queue = match self.queue.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if queue.is_empty() {
            return Ok(GuestWrite::Empty);
        }

        let (post_mark, message) = queue.pop().unwrap();

        let recipient = Uuid::from_u128(post_mark.recipient()).to_string();
        let guest = &(recipient.as_str(), &self.linker);

        let length = message.len().try_into()?;
        let offset = guest.allocate(length)?;

        guest.write(offset, message.as_ref())?;

        Ok(GuestWrite::Guest {
            id: recipient,
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

        let ping = system.register(include_bytes!("ping.wat")).unwrap();
        let pong = system.register(include_bytes!("pong.wat")).unwrap();

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
