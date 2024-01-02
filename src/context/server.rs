use crate::script::{Process, Wasm};
use crate::{about, server};
use std::ffi::OsStr;
use std::net::SocketAddr;
use std::path::PathBuf;

pub struct ServerContext {
    document_root: PathBuf,
    cgi_bin: PathBuf,
    hostname: String,
    ip_address: String,
    port: String,
    path: &'static str,
    scheme: String,
    software: String,
    cgi_scripts: ScriptMapping,
}

pub struct ScriptMapping {
    process: Process,
    wasm: Wasm,
}

impl ScriptMapping {
    pub fn new(process: Process, wasm: Wasm) -> Self {
        Self { process, wasm }
    }

    pub fn process(&self) -> &Process {
        &self.process
    }

    pub fn wasm(&self) -> &Wasm {
        &self.wasm
    }
}

impl ServerContext {
    pub fn new(address: SocketAddr, options: server::Options, cgi_scripts: ScriptMapping) -> Self {
        let ip_address = address.ip().to_string();
        let port = address.port().to_string();

        let path: &'static str = env!("PATH");
        let scheme = String::from("http");
        let software = format!("{}/{}", about::PROGRAM, about::VERSION);

        Self {
            document_root: options.document_root,
            cgi_bin: options.cgi_bin,
            hostname: options.hostname,
            ip_address,
            port,
            path,
            scheme,
            software,
            cgi_scripts,
        }
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn scheme(&self) -> &str {
        self.scheme.as_str()
    }

    pub fn software(&self) -> &str {
        self.software.as_str()
    }

    pub fn server_name(&self) -> &str {
        self.hostname.as_str()
    }

    pub fn script_filename<'a>(&self, path: &'a str) -> Option<(PathBuf, &'a str)> {
        let script_path = path.strip_prefix("/cgi-bin/")?;

        let index = script_path
            .chars()
            .position(|c| c == '/')
            .unwrap_or_else(|| script_path.len());
        let (filename, extra_path) = script_path.split_at(index);

        let mut file_path = self.cgi_bin.clone();

        file_path.push(filename);

        Some((file_path, extra_path))
    }

    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let mut normalized_path = path.strip_prefix('/').unwrap_or(path);

        if normalized_path.is_empty() {
            normalized_path = "index.html";
        }

        self.document_root.join(normalized_path)
    }

    pub fn translate_path(&self, path: &str) -> PathBuf {
        self.document_root
            .join(path.strip_prefix('/').unwrap_or(path))
    }

    pub fn working_directory(&self) -> &OsStr {
        self.document_root.as_os_str()
    }

    pub fn ip_address(&self) -> &str {
        self.ip_address.as_str()
    }

    pub fn port(&self) -> &str {
        self.port.as_str()
    }

    pub fn script_mappings(&self) -> &ScriptMapping {
        &self.cgi_scripts
    }
}
