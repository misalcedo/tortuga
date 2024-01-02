use sha3::{Digest, Sha3_256};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use wasi_common::WasiCtx;
use wasmtime::{Config, Engine, InstancePre, Linker, Module, Store};

#[derive(Clone)]
struct CacheEntry {
    digest: String,
    module: InstancePre<WasiCtx>,
}

#[derive(Clone)]
pub struct ModuleLoader {
    cache: Option<Arc<RwLock<HashMap<PathBuf, CacheEntry>>>>,
    engine: Engine,
    root: PathBuf,
}

type ModuleResult = Result<InstancePre<WasiCtx>, wasmtime::Error>;

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

    pub async fn load(&self, path: &PathBuf) -> ModuleResult {
        match self.cache.as_ref() {
            None => self.load_from_file(path),
            Some(lock) => {
                let mut cache = lock.write().unwrap_or_else(|e| {
                    let mut guard = e.into_inner();
                    *guard = HashMap::new();
                    guard
                });

                match cache.entry(path.clone()) {
                    Entry::Occupied(entry) => Ok(entry.get().module.clone()),
                    Entry::Vacant(entry) => {
                        let cache_entry = self.new_cache_entry(path)?;
                        Ok(entry.insert(cache_entry).module.clone())
                    }
                }
            }
        }
    }

    fn delete_entry(&self, path: &PathBuf) {
        if let Some(cache) = self.cache.as_ref() {
            match cache.write() {
                Ok(mut cache) => {
                    cache.remove(path);
                }
                Err(e) => {
                    let mut guard = e.into_inner();
                    *guard = HashMap::new();
                }
            }
        }
    }

    fn get_cache_entry(&self, path: &PathBuf) -> Option<CacheEntry> {
        self.cache.as_ref()?.read().ok()?.get(path).cloned()
    }

    fn new_cache_entry(&self, path: &PathBuf) -> Result<CacheEntry, wasmtime::Error> {
        let module = self.load_from_file(path)?;
        let code = module.module().text();

        let mut hasher = Sha3_256::new();

        hasher.update(code.len().to_be_bytes());
        hasher.update(code);

        let result = hasher.finalize();
        let digest = hex::encode(result);

        Ok(CacheEntry { digest, module })
    }

    fn load_from_file(&self, path: &PathBuf) -> ModuleResult {
        let module = Module::from_file(&self.engine, path)?;
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        linker.instantiate_pre(&module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_digest() {
        let loader = ModuleLoader::new("examples/".into(), true).unwrap();
        let path = "examples/echo.wcgi".into();

        let first = loader.new_cache_entry(&path).unwrap();
        let second = loader.new_cache_entry(&path).unwrap();

        assert_eq!(first.digest, second.digest);
    }
}
