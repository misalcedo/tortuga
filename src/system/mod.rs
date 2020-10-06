mod error;

use crate::wasm::Guest;
pub use error::Error;
use std::convert::TryInto;
use wasmtime::{Config, Engine, Extern, Instance, Module, Store};

pub struct System {
    store: Store,
}

impl System {
    fn new() -> System {
        let mut config = Config::new();

        config.wasm_reference_types(true);

        let engine = Engine::new(&config);
        let store = Store::new(&engine);

        System { store }
    }

    fn store(&self) -> &Store {
        &self.store
    }

    fn engine(&self) -> &Engine {
        self.store.engine()
    }

    fn module(&self, intent: &[u8]) -> Result<Module, Error> {
        Ok(Module::new(self.store.engine(), intent)?)
    }

    fn instance(&self, module: &Module, imports: &[Extern]) -> Result<Instance, Error> {
        Ok(Instance::new(&self.store, module, imports)?)
    }

    fn send<T: Guest>(&self, guest: &T, sender: u128, message: &[u8]) -> Result<(), Error> {
        let length = message.len().try_into()?;
        let offset = guest.allocate(length)?;

        guest.write(offset, message)?;

        Ok(guest.receive(sender, offset, length)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::system::System;
    use crate::wasm::Guest;
    use std::sync::mpsc::channel;
    use wasmtime::{ExternRef, Func, Instance};

    #[test]
    fn act() {
        let mut system = System::new();
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
}
