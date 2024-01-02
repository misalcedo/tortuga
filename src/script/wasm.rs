use crate::context::RequestContext;
use crate::{ModuleLoader, Script};
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

    pub async fn run(
        &self,
        context: RequestContext,
        body: Bytes,
    ) -> Result<Bytes, wasmtime::Error> {
        let file = context.script().map_err(wasmtime::Error::msg)?;
        let module = self.loader.load(file).await?;

        let stdout = WritePipe::new_in_memory();
        let mut builder = WasiCtxBuilder::new();

        for argument in context.arguments() {
            builder.arg(argument)?;
        }

        for (variable, value) in context.variables() {
            builder.env(variable, value)?;
        }

        let wasi = builder
            .stdin(Box::new(ReadPipe::from(body.as_ref())))
            .stdout(Box::new(stdout.clone()))
            .inherit_stderr()
            .build();

        let mut store = self.loader.new_store(wasi);

        store.set_fuel(1_000_000)?;
        store.fuel_async_yield_interval(Some(1_000))?;

        let instance = module.instantiate_async(&mut store).await?;
        let mut function = instance.get_typed_func(&mut store, "");

        if function.is_err() {
            function = instance.get_typed_func(&mut store, "_start");
        }

        function?.call_async(&mut store, ()).await?;

        drop(store);

        match stdout.try_into_inner() {
            Ok(output) => Ok(Bytes::from(output.into_inner())),
            Err(_) => Err(wasmtime::Error::msg(
                "Unable to extract output from CGI script",
            )),
        }
    }
}

impl Script for Wasm {
    async fn invoke(&self, context: RequestContext, body: Bytes) -> io::Result<Bytes> {
        self.run(context, body)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
