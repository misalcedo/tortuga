use crate::context::RequestContext;
use crate::{wasm::ModuleLoader, Script};
use bytes::Bytes;
use std::io;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime_wasi::WasiCtxBuilder;

pub struct Wasm {
    loader: ModuleLoader,
}

impl Wasm {
    pub fn new(loader: ModuleLoader) -> Self {
        Self { loader }
    }
}

impl Script for Wasm {
    async fn invoke(&self, context: RequestContext, body: Bytes) -> io::Result<Bytes> {
        let file = context.script()?;
        let module = self.loader.load(file).await?;

        let stdout = WritePipe::new_in_memory();
        let mut builder = WasiCtxBuilder::new();

        for argument in context.arguments() {
            builder
                .arg(argument)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        for (variable, value) in context.variables() {
            builder
                .env(variable, value)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        let wasi = builder
            .stdin(Box::new(ReadPipe::from(body.as_ref())))
            .stdout(Box::new(stdout.clone()))
            .inherit_stderr()
            .build();

        let mut store = self.loader.new_store(wasi);

        store
            .set_fuel(1_000_000)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        store
            .fuel_async_yield_interval(Some(50_000))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let instance = module
            .instantiate_async(&mut store)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut function = instance.get_typed_func(&mut store, "");

        if function.is_err() {
            function = instance.get_typed_func(&mut store, "_start");
        }

        function
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .call_async(&mut store, ())
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        drop(store);

        let bytes = stdout
            .try_into_inner()
            .map_err(|_| io::Error::from(io::ErrorKind::BrokenPipe))?;

        Ok(Bytes::from(bytes.into_inner()))
    }
}
