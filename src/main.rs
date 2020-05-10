extern crate wasmer_runtime;

use wasmer_runtime::{error, func, imports, instantiate, Array, Ctx, WasmPtr};

static WASM: &'static [u8] = include_bytes!("../resources/wasm/math.wasm");

mod actor;

fn main() -> error::Result<()> {
    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // We've defined a macro that makes it super easy.
    //
    // The signature tells the runtime what the signature (the parameter
    // and return types) of the function we're defining here is.
    // The allowed types are `i32`, `u32`, `i64`, `u64`,
    // `f32`, and `f64`.
    //
    // Make sure to check this carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our sample application.
        "system" => {
            // name        // the func! macro autodetects the signature
            "print" => func!(print),
        },
    };

    // Compile our webassembly into an `Instance`.
    let instance = instantiate(WASM, &import_object)?;

    // Call our exported function!
    instance.call("add", &[])?;
    instance.call("subtract", &[])?;
    instance.call("multiply", &[])?;
    instance.call("divide", &[])?;

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