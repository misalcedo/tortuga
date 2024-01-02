use crate::context::RequestContext;
use crate::Script;
use bytes::Bytes;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;
use tokio::fs::File;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasi_common::WasiCtx;
use wasmtime::{Config, Engine, InstancePre, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

async fn last_modified(path: &PathBuf) -> io::Result<SystemTime> {
    let file = File::open(path).await?;
    let metadata = file.metadata().await?;

    metadata.modified()
}

pub struct Wasm {
    engine: Engine,
    cache: RwLock<HashMap<PathBuf, (SystemTime, InstancePre<WasiCtx>)>>,
}

impl Wasm {
    pub fn new() -> Result<Self, wasmtime::Error> {
        let mut configuration = Config::new();

        configuration
            .async_support(true)
            .consume_fuel(true)
            .parallel_compilation(true);

        let engine = Engine::new(&configuration)?;
        let cache = RwLock::new(HashMap::new());

        Ok(Self { engine, cache })
    }

    pub async fn run(
        &self,
        context: RequestContext,
        body: Bytes,
    ) -> Result<Bytes, wasmtime::Error> {
        let file = context.script().map_err(wasmtime::Error::msg)?;
        let module = self.module(file).await?;

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

        let mut store = Store::new(&self.engine, wasi);

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

    async fn module(&self, path: &PathBuf) -> Result<InstancePre<WasiCtx>, wasmtime::Error> {
        let (timestamp, mut module) = self
            .cached_module(path)
            .map(|(time, module)| (Some(time), Some(Ok(module))))
            .unwrap_or((None, None));

        match last_modified(path).await {
            Ok(modified) => {
                // Update the cache if the file has changed or we don't have a cached modified timestamp.
                if timestamp.map(|t| modified > t).unwrap_or(true) {
                    match self.cache.write() {
                        Ok(mut cache) => {
                            let new_module = self.new_module(path)?;
                            module = Some(Ok(new_module.clone()));
                            cache.insert(path.clone(), (modified, new_module));
                        }
                        Err(e) => {
                            let mut guard = e.into_inner();
                            *guard = HashMap::new();
                        }
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // Delete the module from the cache if the file no longer exists.
                match self.cache.write() {
                    Ok(mut cache) => {
                        cache.remove(path);
                    }
                    Err(e) => {
                        let mut guard = e.into_inner();
                        *guard = HashMap::new();
                    }
                }
            }
            Err(_) => {}
        }

        module.unwrap_or_else(|| self.new_module(path))
    }

    fn new_module(&self, path: &PathBuf) -> Result<InstancePre<WasiCtx>, wasmtime::Error> {
        let module = Module::from_file(&self.engine, path)?;
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        linker.instantiate_pre(&module)
    }

    fn cached_module(&self, path: &PathBuf) -> Option<(SystemTime, InstancePre<WasiCtx>)> {
        self.cache.read().ok()?.get(path).cloned()
    }
}

impl Script for Wasm {
    async fn invoke(&self, context: RequestContext, body: Bytes) -> io::Result<Bytes> {
        self.run(context, body)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
