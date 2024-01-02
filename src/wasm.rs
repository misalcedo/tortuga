use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;
use wasi_common::WasiCtx;
use wasmtime::{Config, Engine, InstancePre, Linker, Module, Store};

#[derive(Clone)]
pub struct ModuleLoader {
    cache: Option<Arc<RwLock<HashMap<PathBuf, InstancePre<WasiCtx>>>>>,
    engine: Engine,
    root: PathBuf,
}

impl ModuleLoader {
    pub fn new(root: PathBuf, cache: bool) -> Result<Self, wasmtime::Error> {
        let mut configuration = Config::new();

        configuration
            .async_support(true)
            .consume_fuel(true)
            .parallel_compilation(true);

        let engine = Engine::new(&configuration)?;
        let cache = if cache {
            Some(Arc::new(RwLock::new(HashMap::new())))
        } else {
            None
        };

        Ok(Self {
            cache,
            engine,
            root,
        })
    }

    pub fn new_store(&self, wasi_ctx: WasiCtx) -> Store<WasiCtx> {
        Store::new(&self.engine, wasi_ctx)
    }

    pub async fn load(&self, path: &PathBuf) -> io::Result<InstancePre<WasiCtx>> {
        match self.cache.as_ref() {
            None => self.load_from_file(path).await,
            Some(lock) => match self.get_cache_entry(path).await {
                None => {
                    let mut cache = lock.write().await;

                    match cache.entry(path.clone()) {
                        Entry::Occupied(entry) => Ok(entry.get().clone()),
                        Entry::Vacant(entry) => {
                            Ok(entry.insert(self.load_from_file(path).await?).clone())
                        }
                    }
                }
                Some(module) => Ok(module),
            },
        }
    }

    async fn get_cache_entry(&self, path: &PathBuf) -> Option<InstancePre<WasiCtx>> {
        self.cache.as_ref()?.read().await.get(path).cloned()
    }

    async fn load_from_file(&self, path: &PathBuf) -> io::Result<InstancePre<WasiCtx>> {
        let mut file = File::open(path).await?;
        let mut buffer = Vec::with_capacity(1024 * 8);

        file.read_to_end(&mut buffer).await?;

        let module = Module::from_binary(&self.engine, buffer.as_slice())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| s)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        linker
            .instantiate_pre(&module)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
