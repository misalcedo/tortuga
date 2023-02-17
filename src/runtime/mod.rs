//! The embedding runtime for the Tortuga WASM modules.

use wasmtime::{Caller, Config, Engine, Linker, Module, Store};

#[derive(Default)]
pub struct Runtime {
    engine: Engine,
}

pub struct Shell {
    module: Module,
}

impl Runtime {
    pub fn load(&mut self, code: impl AsRef<[u8]>) -> Shell {
        // Modules can be compiled through either the text or binary format
        let module = Module::new(&self.engine, code).unwrap();

        Shell { module }
    }

    pub fn execute<Data>(&mut self, shell: &Shell, data: Data) -> i32 {
        // All wasm objects operate within the context of a "store". Each
        // `Store` has a type parameter to store host-specific data, which in
        // this case we're using `4` for.
        let mut store = Store::new(&self.engine, data);

        // Create a `Linker` which will be later used to instantiate this module.
        // Host functionality is defined by name within the `Linker`.
        let mut linker = Linker::new(&self.engine);

        linker
            .func_wrap("host", "hello", |_: Caller<'_, Data>, x: i32| x * 2)
            .unwrap();

        let instance = linker.instantiate(&mut store, &shell.module).unwrap();
        let hello = instance
            .get_typed_func::<(), i32>(&mut store, "hello")
            .unwrap();

        // And finally we can call the wasm!
        hello.call(&mut store, ()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::Module;

    #[test]
    fn execute_shell() {
        let code = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
        let mut runtime = Runtime::default();
        let shell = runtime.load(code);

        assert_eq!(runtime.execute(&shell, 42), 6)
    }
}
