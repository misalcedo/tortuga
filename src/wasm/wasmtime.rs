use crate::wasm::guest::Guest;
use crate::wasm::Error;
use wasmtime::Instance;

const EXPORTED_MEMORY: &str = "io";
const ALLOCATE_EXPORT: &str = "allocate";
const RECEIVE_EXPORT: &str = "receive";

impl Guest for Instance {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Result<u32, Error> {
        let module_allocate = self
            .get_func(ALLOCATE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(ALLOCATE_EXPORT)))?;

        let module_allocate = module_allocate.get1::<u32, u32>()?;
        let offset = module_allocate(minimum_length)?;

        Ok(offset)
    }

    /// Writes a message into an instance of a WebAssembly module.
    fn write(&self, offset: u32, message: &[u8]) -> Result<(), Error> {
        let memory = self
            .get_memory(EXPORTED_MEMORY)
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            memory.data_unchecked_mut()[offset as usize..][..message.len()]
                .copy_from_slice(message);
        }

        Ok(())
    }

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    /// The guest implicitly trusts the host to send the previously allocated slice.
    fn receive(&self, offset: u32, length: u32) -> Result<(), Error> {
        let module_receive = self
            .get_func(RECEIVE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(RECEIVE_EXPORT)))?;

        let module_receive = module_receive.get2::<u32, u32, ()>().unwrap();

        Ok(module_receive(offset, length)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::guest::Guest;
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

        assert_eq!(instance.allocate(42).unwrap(), 0);
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

        instance.receive(42, 1).unwrap();

        assert_eq!(receiver.recv(), Ok((42, 1)));
    }

    #[test]
    fn copy_message() {
        let engine = Engine::default();
        let store = Store::new(&engine);

        let intent: &[u8] = include_bytes!("echo.wat");
        let module = Module::new(&engine, intent).unwrap();

        let send = Func::wrap(&store, move |_: u32, _: u32| {
            panic!("Copy must not send any messages.")
        });

        let instance = Instance::new(&store, &module, &[send.into()]).unwrap();
        let message = b"Hello, World!";

        let length = message.len() as u32;
        let offset = instance.allocate(length).unwrap();

        instance.write(offset, message).unwrap();

        unsafe {
            let memory = instance.get_memory("io").unwrap();
            let data = &memory.data_unchecked()[offset as usize..][..message.len()];

            assert_eq!(message, &data);
        };
    }
}
