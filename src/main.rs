// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, func, Func, Instance, WasmPtr, Array, Ctx};

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../examples/echo.wasm");

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {
        "system" => {
            "send" => func!(send),
        },
    };

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    pass(instance);

    // Return OK since everything executed successfully!
    Ok(())
}

fn pass(instance: Instance) {
    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);
    let buffer_pointer: WasmPtr<u8, Array> = WasmPtr::new(0);

    // Let's write a string to the Wasm memory
    let original_string = "Hello, World!";

    // We deref our WasmPtr to get a &[Cell<u8>]
    let memory_writer = buffer_pointer
        .deref(wasm_instance_memory, 0, original_string.len() as u32)
        .unwrap();
    for (i, b) in original_string.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    // Let's call the exported function that concatenates a phrase to our string.
    let receive: Func<(WasmPtr<u8, Array>, u32), ()> = instance
        .exports
        .get("receive")
        .expect("receive function not defined.");

    receive.call(WasmPtr::new(0), original_string.len() as u32).unwrap();
}

fn send(ctx: &mut Ctx, address: WasmPtr<u8, Array>, length: u32) {
    let memory = ctx.memory(0);
    let value = address.get_utf8_string(memory, length);

    println!("Address: {:?}, Length: {}, Value: {:?}", address, length, value);
}