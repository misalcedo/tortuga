mod actor;

extern crate wasmer_runtime;

use wasmer_runtime::{error, func, imports, instantiate, Instance, ImportObject, Array, Ctx, WasmPtr};
use std::collections::HashMap;
use crate::actor::DistributionCenter;
use crate::actor::reference::Reference;

static MATH_WASM: &'static [u8] = include_bytes!("../resources/wasm/math.wasm");
static ECHO_WASM: &'static [u8] = include_bytes!("../resources/wasm/echo.wasm");

struct System {
    dc: DistributionCenter,
    reference: Reference,
    import: ImportObject,
    instances: HashMap<Reference, Instance>
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

fn main() -> Result<(), &'static str> {
    let mut system = System::new();
    let math = system.create(MATH_WASM)?;
    let echo = system.create(ECHO_WASM)?;

    system.send(echo, 42)?;
    system.send(echo, 19)?;

    Ok(())
}

// Let's define our "print" function.
//
// The declaration must start with "extern" or "extern "C"".
fn print(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = ctx.memory(0);

    // Use helper method on `WasmPtr` to read a utf8 string
    let string = ptr.get_utf8_string(memory, len).unwrap();

    // Print it!
    println!("{}", string);
}

fn send(ctx: &mut Ctx, ptr: WasmPtr<u32>) {
    let memory = ctx.memory(0);
    
    if let Some(cell) = ptr.deref(memory) {
        println!("Message: {}", cell.get());
    } else {
        eprintln!("Unable to load memory to read the message.");
    }
}