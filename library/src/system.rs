use crate::actor::DistributionCenter;
use wasmer_runtime::{func, imports, instantiate, Instance, ImportObject, WasmPtr};
use std::collections::HashMap;
use crate::actor::reference::Reference;

pub struct System {
    pub dc: DistributionCenter,
    pub reference: Reference,
    pub import: ImportObject,
    pub instances: HashMap<Reference, Instance>
}

impl System {
    fn new() -> System {
        let import = imports! {
            "system" => {
                "print" => func!(print),
                "send" => func!(send),
            },
        };

        System {
            dc: DistributionCenter::new(),
            reference: Reference::new(),
            instances: HashMap::new(),
            import
        }
    }

    pub fn create(&mut self, module: &[u8]) -> Result<Reference, &'static str> {
        let reference = Reference::new();
        let instance = instantiate(module, &self.import).map_err(|_| "Unable to instantiate the WASM module.")?;

        self.instances.insert(reference, instance);

        Ok(reference)
    }

    pub fn send(&mut self, to: Reference, message: u32) -> Result<(), &'static str> {
        if let Some(instance) = self.instances.get_mut(&to) {
            let mut context = instance.context_mut();
            let memory = context.memory(0);
            let pointer: WasmPtr<u32> = WasmPtr::new(0);

            let cell = pointer.deref(memory).ok_or("Unable to dereference the memory pointer to write the message.")?;

            cell.set(message);

            instance.call("receive", &[]).map_err(|_| "Unabe to trigger the actor's behavior.")?;
            
            Ok(())
        } else {
            Err("No such actor found.")
        }
    }
}