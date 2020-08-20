use crate::actor::{Actor, Address};
use wasmtime::Instance;

impl Actor for Instance {
    /// Allocates a slice whose length is greater than or equal to the given minimum.
    fn allocate(&self, minimum_length: u32) -> Address {
        let module_allocate = self
            .get_func("allocate")
            .expect("`allocate` was not an exported function");
        let module_allocate = module_allocate.get1::<u32, u32>().unwrap();
        let offset = module_allocate(minimum_length).unwrap();

        Address::new(offset, minimum_length)
    }

    /// Receives a message from another actor. The system makes no guarantees about the contents.
    fn receive(&self, message: Address) {
        let module_receive = self
            .get_func("receive")
            .expect("`receive` was not an exported function");
        let module_receive = module_receive.get2::<u32, u32, ()>().unwrap();

        module_receive(message.offset(), message.length()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::actor::{Actor, Address};
    use std::sync::mpsc::channel;
    use wasmtime::{Engine, Func, Instance, Module, Store};

    #[test]
    fn allocate_happy_case() {
        let engine = Engine::default();
        let store = Store::new(&engine);

        let intent: &[u8] = include_bytes!("../resources/echo.wat");
        let module = Module::new(&engine, intent).unwrap();
        let send = Func::wrap(&store, move |offset: u32, length: u32| {
            panic!("Allocate must not send any messages.")
        });
        let mut instance = Instance::new(&store, &module, &[send.into()]).unwrap();

        assert_eq!(instance.allocate(42), Address::new(0, 42));
    }

    #[test]
    fn receive_happy_case() {
        let engine = Engine::default();
        let store = Store::new(&engine);

        let intent: &[u8] = include_bytes!("../resources/echo.wat");
        let module = Module::new(&engine, intent).unwrap();

        let (sender, receiver) = channel();
        let send = Func::wrap(&store, move |offset: u32, length: u32| {
            sender.send((offset, length)).unwrap();
        });

        let mut instance = Instance::new(&store, &module, &[send.into()]).unwrap();
        let address = Address::new(42, 1);

        instance.receive(address);

        assert_eq!(receiver.recv(), Ok((42, 1)));
    }
}
