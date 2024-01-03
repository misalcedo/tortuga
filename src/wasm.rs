use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
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

    pub async fn scan(&self) -> io::Result<()> {
        let seen = self.walk_filesystem().await?;

        self.purge(seen);

        Ok(())
    }

    async fn walk_filesystem(&self) -> io::Result<HashSet<PathBuf>> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back(self.root.clone());

        while let Some(next) = queue.pop_front() {
            if seen.contains(&next) {
                continue;
            }

            if next.is_dir() {
                let mut reader = tokio::fs::read_dir(&next).await?;

                while let Some(entry) = reader.next_entry().await? {
                    let path = entry.path();

                    queue.push_back(path)
                }
            } else if next.extension() == Some("wcgi".as_ref()) {
                self.load(&next).await?;
            }

            seen.insert(next.clone());
        }
        Ok(seen)
    }

    fn purge(&self, seen: HashSet<PathBuf>) {
        if let Some(lock) = self.cache.as_ref() {
            let mut cache = lock.write().unwrap_or_else(|e| {
                let mut guard = e.into_inner();
                *guard = HashMap::new();
                guard
            });

            let paths: Vec<PathBuf> = cache.keys().cloned().collect();

            for path in paths {
                if !seen.contains(&path) {
                    cache.remove(&path);
                }
            }
        }
    }

    pub async fn load(&self, path: &PathBuf) -> io::Result<InstancePre<WasiCtx>> {
        match self.cache.as_ref() {
            None => self.load_from_file(path),
            Some(lock) => match self.get_cache_entry(path) {
                None => {
                    let mut cache = lock.write().unwrap_or_else(|e| {
                        let mut guard = e.into_inner();
                        *guard = HashMap::new();
                        guard
                    });

                    match cache.entry(path.clone()) {
                        Entry::Occupied(entry) => Ok(entry.get().clone()),
                        Entry::Vacant(entry) => {
                            Ok(entry.insert(self.load_from_file(path)?).clone())
                        }
                    }
                }
                Some(module) => Ok(module),
            },
        }
    }

    fn get_cache_entry(&self, path: &PathBuf) -> Option<InstancePre<WasiCtx>> {
        self.cache.as_ref()?.read().ok()?.get(path).cloned()
    }

    fn load_from_file(&self, path: &PathBuf) -> io::Result<InstancePre<WasiCtx>> {
        let module = Module::from_file(&self.engine, path)
            .map_err(|_| io::Error::from(io::ErrorKind::NotFound))?;
        let mut linker = Linker::new(&self.engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| s)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        linker
            .instantiate_pre(&module)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
