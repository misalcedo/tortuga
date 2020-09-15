use crate::wasm::guest::Guest;
use crate::wasm::memory::{Address, Allocator};
use crate::wasm::Error;
use std::convert::TryInto;
use wasmtime::Instance;

const EXPORTED_MEMORY: &str = "io";
const ALLOCATE_EXPORT: &str = "allocate";
const RECEIVE_EXPORT: &str = "receive";

impl Guest for Instance {
    /// There is no way for the guest to ensure the host copies the message into the given bytes.
    /// Therefore, the guest must take care to handle uninitialized memory.
    fn copy(&self, message: &[u8]) -> Result<Address, Error> {
        let length = message.len().try_into()?;
        let address = self.allocate(length)?;

        let memory = self
            .get_memory(EXPORTED_MEMORY)
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            memory.data_unchecked_mut()[address.offset()..][..address.length()]
                .copy_from_slice(message);
        }

        Ok(address)
    }

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    /// The guest implicitly trusts the host to send the previously allocated slice.
    fn receive(&self, message: Address) -> Result<(), Error> {
        let module_receive = self
            .get_func(RECEIVE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(RECEIVE_EXPORT)))?;

        let module_receive = module_receive.get2::<u32, u32, ()>().unwrap();

        Ok(module_receive(
            message.offset() as u32,
            message.length() as u32,
        )?)
    }
}

impl Allocator for Instance {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Result<Address, Error> {
        let module_allocate = self
            .get_func(ALLOCATE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(ALLOCATE_EXPORT)))?;

        let module_allocate = module_allocate.get1::<u32, u32>()?;
        let offset = module_allocate(minimum_length)?;

        Ok(Address::new(offset as usize, minimum_length as usize))
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::guest::Guest;
    use crate::wasm::memory::{Address, Allocator};
    use std::sync::mpsc::channel;
    use wasmtime::{Engine, Func, Instance, Module, Store};

    #[test]
    fn allocate_happy_case() {
        let engine = Engine::default();
        let store = Store::new(&engine);

        let intent: &[u8] = include_bytes!("echo.wat");
        let module = Module::new(&engine, intent).unwrap();
        let send = Func::wrap(&store, move |_: u32, _: u32| {
            panic!("Allocate must not send any messages.")
        });
        let instance = Instance::new(&store, &module, &[send.into()]).unwrap();

        assert_eq!(instance.allocate(42).unwrap(), Address::new(0, 42));
    }

    #[test]
    fn receive_happy_case() {
        let engine = Engine::default();
        let store = Store::new(&engine);

        let intent: &[u8] = include_bytes!("echo.wat");
        let module = Module::new(&engine, intent).unwrap();

        let (sender, receiver) = channel();
        let send = Func::wrap(&store, move |offset: u32, length: u32| {
            sender.send((offset, length)).unwrap();
        });

        let instance = Instance::new(&store, &module, &[send.into()]).unwrap();
        let address = Address::new(42, 1);

        instance.receive(address).unwrap();

        assert_eq!(receiver.recv(), Ok((42, 1)));
    }
}
