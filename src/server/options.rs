use std::path::PathBuf;

#[derive(Clone)]
pub struct Options {
    /// The document root path to load CGI scripts and other assets from.
    pub document_root: PathBuf,

    /// The path to CGI scripts; may be relative or absolute.
    /// Relative paths are resolved from the document root.
    pub cgi_bin: PathBuf,

    /// The hostname of the local TCP interface for the server to listen on.
    pub hostname: String,

    /// The TCP port for the server to listen on.
    pub port: u16,

    /// Enable an in-memory cache for compiled WebAssembly modules.
    pub wasm_cache: bool,

    /// Pre-load compiled WebAssembly modules into the in-memory cache.
    pub preload_wasm: bool,
}
