use crate::wasm::guest::Guest;
use crate::wasm::Error;
use wasmtime::{Caller, ExternRef, Instance, Linker};

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

    fn read(&self, offset: u32, buffer: &mut [u8]) -> Result<(), Error> {
        let memory = self
            .get_memory(EXPORTED_MEMORY)
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            buffer.copy_from_slice(&memory.data_unchecked()[offset as usize..][..buffer.len()]);
        }

        Ok(())
    }

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    /// The guest implicitly trusts the host to send the previously allocated slice.
    fn receive(&self, uuid: u128, offset: u32, length: u32) -> Result<(), Error> {
        let module_receive = self
            .get_func(RECEIVE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(RECEIVE_EXPORT)))?;

        let module_receive = module_receive
            .get3::<Option<ExternRef>, u32, u32, ()>()
            .unwrap();
        let source = ExternRef::new(uuid);

        Ok(module_receive(Some(source), offset, length)?)
    }
}

impl Guest for Caller<'_> {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Result<u32, Error> {
        let module_allocate = self
            .get_export(ALLOCATE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(ALLOCATE_EXPORT)))?
            .into_func()
            .ok_or_else(|| Error::NoMatchingFunction(String::from(ALLOCATE_EXPORT)))?;

        let module_allocate = module_allocate.get1::<u32, u32>()?;
        let offset = module_allocate(minimum_length)?;

        Ok(offset)
    }

    /// Writes a message into an instance of a WebAssembly module.
    fn write(&self, offset: u32, message: &[u8]) -> Result<(), Error> {
        let memory = self
            .get_export(EXPORTED_MEMORY)
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?
            .into_memory()
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            memory.data_unchecked_mut()[offset as usize..][..message.len()]
                .copy_from_slice(message);
        }

        Ok(())
    }

    fn read(&self, offset: u32, buffer: &mut [u8]) -> Result<(), Error> {
        let memory = self
            .get_export(EXPORTED_MEMORY)
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?
            .into_memory()
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            buffer.copy_from_slice(&memory.data_unchecked()[offset as usize..][..buffer.len()]);
        }

        Ok(())
    }

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    /// The guest implicitly trusts the host to send the previously allocated slice.
    fn receive(&self, uuid: u128, offset: u32, length: u32) -> Result<(), Error> {
        let module_receive = self
            .get_export(RECEIVE_EXPORT)
            .ok_or_else(|| Error::NoMatchingFunction(String::from(RECEIVE_EXPORT)))?
            .into_func()
            .ok_or_else(|| Error::NoMatchingFunction(String::from(RECEIVE_EXPORT)))?;

        let module_receive = module_receive
            .get3::<Option<ExternRef>, u32, u32, ()>()
            .unwrap();
        let source = ExternRef::new(uuid);

        Ok(module_receive(Some(source), offset, length)?)
    }
}

impl Guest for (&str, &Linker) {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Result<u32, Error> {
        let module_allocate = self
            .1
            .get_one_by_name(self.0, ALLOCATE_EXPORT)?
            .into_func()
            .ok_or_else(|| Error::NoMatchingFunction(String::from(ALLOCATE_EXPORT)))?;

        let module_allocate = module_allocate.get1::<u32, u32>()?;
        let offset = module_allocate(minimum_length)?;

        Ok(offset)
    }

    /// Writes a message into an instance of a WebAssembly module.
    fn write(&self, offset: u32, message: &[u8]) -> Result<(), Error> {
        let memory = self
            .1
            .get_one_by_name(self.0, EXPORTED_MEMORY)?
            .into_memory()
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            memory.data_unchecked_mut()[offset as usize..][..message.len()]
                .copy_from_slice(message);
        }

        Ok(())
    }

    fn read(&self, offset: u32, buffer: &mut [u8]) -> Result<(), Error> {
        let memory = self
            .1
            .get_one_by_name(self.0, EXPORTED_MEMORY)?
            .into_memory()
            .ok_or_else(|| Error::NoMatchingMemory(String::from(EXPORTED_MEMORY)))?;

        unsafe {
            buffer.copy_from_slice(&memory.data_unchecked()[offset as usize..][..buffer.len()]);
        }

        Ok(())
    }

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    /// The guest implicitly trusts the host to send the previously allocated slice.
    fn receive(&self, uuid: u128, offset: u32, length: u32) -> Result<(), Error> {
        let module_receive = self
            .1
            .get_one_by_name(self.0, RECEIVE_EXPORT)?
            .into_func()
            .ok_or_else(|| Error::NoMatchingFunction(String::from(RECEIVE_EXPORT)))?;

        let module_receive = module_receive
            .get3::<Option<ExternRef>, u32, u32, ()>()
            .unwrap();
        let source = ExternRef::new(uuid);

        Ok(module_receive(Some(source), offset, length)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::guest::Guest;
    use std::sync::mpsc::channel;
    use wasmtime::{Config, Engine, ExternRef, Func, Instance, Module, Store};

    #[test]
    fn allocate_happy_case() {
        let module = create_echo_module();
        let store = Store::new(module.engine());
        let send = Func::wrap(&store, move |_: Option<ExternRef>, _: u32, _: u32| {
            panic!("Allocate must not send any messages.")
        });
        let instance = Instance::new(&store, &module, &[send.into()]).unwrap();

        assert_eq!(instance.allocate(42).unwrap(), 0);
    }

    #[test]
    fn receive_happy_case() {
        let module = create_echo_module();
        let store = Store::new(module.engine());
        let (sender, receiver) = channel();
        let send = Func::wrap(
            &store,
            move |destination: Option<ExternRef>, offset: u32, length: u32| {
                sender.send((destination, offset, length)).unwrap();
            },
        );

        let instance = Instance::new(&store, &module, &[send.into()]).unwrap();

        instance.receive(7, 42, 1).unwrap();

        let message = receiver.recv().unwrap();

        assert_eq!(
            message.0.unwrap().data().downcast_ref::<u128>(),
            Some(7).as_ref()
        );
        assert_eq!(message.1, 42);
        assert_eq!(message.2, 1);
    }

    #[test]
    fn read_and_write_message() {
        let module = create_echo_module();
        let store = Store::new(module.engine());
        let send = Func::wrap(&store, move |_: Option<ExternRef>, _: u32, _: u32| {
            panic!("Copy must not send any messages.")
        });

        let instance = Instance::new(&store, &module, &[send.into()]).unwrap();
        let message = b"Hello, World!";

        let length = message.len() as u32;
        let offset = instance.allocate(length).unwrap();

        instance.write(offset, message).unwrap();

        let mut data = [0; 13];

        instance.read(offset, &mut data[..]).unwrap();

        assert_eq!(Vec::from(&message[..]), data);
    }

    fn create_echo_module() -> Module {
        let mut config = Config::new();

        config.wasm_reference_types(true);

        let engine = Engine::new(&config);
        let intent: &[u8] = include_bytes!("../../examples/echo.wat");

        Module::new(&engine, intent).unwrap()
    }
}
