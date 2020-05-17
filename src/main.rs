// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, func, Func, Instance, WasmPtr, Array};

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

    // Let's get the pointer to the buffer defined by the Wasm module in the Wasm memory.
    // We use the type system and the power of generics to get a function we can call
    // directly with a type signature of no arguments and returning a WasmPtr<u8, Array>
    let request_buffer_pointer: Func<(), WasmPtr<u8, Array>> = instance
        .exports
        .get("request_buffer")
        .expect("request_buffer function not defined.");
    let buffer_pointer = request_buffer_pointer.call().unwrap();

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
    let receive: Func<(u32, u32), ()> = instance
        .exports
        .get("receive")
        .expect("receive function not defined.");
        
    receive.call(0, original_string.len() as u32).unwrap();
}

fn send(addres: u32, length: u32) {
    println!("Address: {}, Length: {}", addres, length);
}