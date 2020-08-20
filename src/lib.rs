mod actor;
pub mod errors;
mod wasm;

use wasmtime::*;

fn new_system() -> errors::Result<()> {
    let engine = Engine::default();
    // A `Store` is a sort of "global object" in a sense, but for now it suffices
    // to say that it's generally passed to most constructors.
    let store = Store::new(&engine);

    // We start off by creating a `Module` which represents a compiled form
    // of our input resources module. In this case it'll be JIT-compiled after
    // we parse the text format.
    let module = Module::from_file(&engine, "hello.wat")?;

    // After we have a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    let instance = Instance::new(&store, &module, &[])?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let answer = instance
        .get_func("answer")
        .expect("`answer` was not an exported function");

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `get0` (in this case asserting
    // it takes no arguments and returns one i32) and then call it.
    let answer = answer.get0::<i32>()?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the resources function traps.
    let result = answer()?;
    println!("Answer: {:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
