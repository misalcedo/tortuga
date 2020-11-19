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

impl System {
    /// The capacity determines the maximum number of messages the system can buffer.
    fn new(capacity: usize) -> System {
        let mut config = Config::new();

        config.wasm_reference_types(true);

        let engine = Engine::new(&config);
        let store = Store::new(&engine);
        let mut linker = Linker::new(&store);
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

    fn send<T: Guest>(&self, guest: &T, sender: u128, message: &[u8]) -> Result<(), Error> {
        let length = message.len().try_into()?;
        let offset = guest.allocate(length)?;

        guest.write(offset, message)?;

        Ok(guest.receive(sender, offset, length)?)
    }

    pub fn register(&mut self, intent: &[u8]) -> Result<u128, Error> {
        let module = self.module(intent).unwrap();
        let uuid = Uuid::new_v4();
        let id = uuid.as_u128();

        let lock = self.queue.clone();

        let send = Func::wrap(
            self.store(),
            move |caller: Caller<'_>, destination: Option<ExternRef>, offset: u32, length: u32| {
                let mut queue = match lock.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => poisoned.into_inner(),
                };

                // need to expose the array of the next position in the ring during a push operation.
                // Otherwise, every actor instance needs to have its own buffer.
                let mut buffer = [0u8; 8192];
                caller.read(offset, &mut buffer[..length as usize]).unwrap();

                if let Some(external_reference) = destination {
                    let optional_recipient = external_reference.data().downcast_ref::<u128>();

                    if let Some(recipient) = optional_recipient {
                        eprintln!("Actor instance {} sent a message of {} bytes with to {} recipient (message: {:?}).", id, length, *recipient, &buffer[..length as usize]);

                        queue
                            .push(PostMark::new(id, *recipient), &buffer[..length as usize])
                            .unwrap();
                    } else {
                        eprintln!("Actor instance {} sent a message of {} bytes with no intended recipient.", id, length);
                    }
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
        let recipient = Uuid::from_u128(recipient).to_string();

        self.send(&(recipient.as_str(), &self.linker), sender, message)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use uuid::Uuid;
    use wasmtime::{ExternRef, Func, Instance};

    use crate::queue::{PostMark, RingBufferQueue};
    use crate::system::System;
    use crate::wasm::Guest;

    #[test]
    fn act() {
        let mut system = System::new(1);
        let intent: &[u8] = include_bytes!("ping.wat");

        let module = system.module(intent).unwrap();
        let (sender, receiver) = channel();
        let send = Func::wrap(
            &system.store(),
            move |destination: Option<ExternRef>, offset: u32, length: u32| {
                sender.send((destination, offset, length)).unwrap();
            },
        );

        let instance: Instance = system.instance(&module, &[send.into()]).unwrap();

        system.send(&instance, 42, &b"Pong!\n"[..]).unwrap();

        let message = receiver.recv().unwrap();

        assert_eq!(
            message.0.unwrap().data().downcast_ref::<u128>(),
            Some(42).as_ref()
        );

        let mut buffer = [0; 6];
        instance
            .read(message.1, &mut buffer[..message.2 as usize])
            .unwrap();

        assert_eq!(b"Pong!\n", &buffer);
    }

    #[test]
    fn usability() {
        let message_distributor = RingBufferQueue::new(3);
        let mut system = System::new(1);
        let ping_intent: &[u8] = include_bytes!("ping.wat");
        let pong_intent: &[u8] = include_bytes!("pong.wat");

        let ping = system.register(ping_intent).unwrap();
        let pong = system.register(pong_intent).unwrap();

        println!("Ping: {}, Pong: {}", ping, pong);

        system.distribute(ping, pong, b"Pong!\n");

        let mut guard = system.queue.lock().unwrap();
        let envelope = guard.pop().unwrap();

        assert_eq!(envelope.0, PostMark::new(ping, pong));
        // TODO: fix return of ping. somehow we are sending this to the wrong module.
        assert_eq!(envelope.1.as_ref(), b"Ping!\n");
    }
}
