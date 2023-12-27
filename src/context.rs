use crate::{about, ServeOptions};
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
    software: String,
}

impl ServerContext {
    pub fn new(address: SocketAddr, options: ServeOptions) -> Self {
        let ip_address = address.ip().to_string();
        let port = address.port().to_string();

        let path: &'static str = env!("PATH");

        let software = format!("{}/{}", about::PROGRAM, about::VERSION);

        Self {
            document_root: options.document_root,
            cgi_bin: options.cgi_bin,
            hostname: options.hostname,
            ip_address,
            port,
            path,
            software,
        }
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn software(&self) -> &str {
        self.software.as_str()
    }

    pub fn server_name(&self) -> &str {
        self.hostname.as_str()
    }

    pub fn script_filename<'a>(&self, path: &'a str) -> Option<(PathBuf, &'a str)> {
        let script_path = path.strip_prefix("/cgi-bin/")?;
        let (filename, extra_path) = script_path
            .split_once('/')
            .unwrap_or_else(|| (script_path, &script_path[script_path.len()..]));

        let mut file_path = self.cgi_bin.clone();

        file_path.push(filename);

        Some((file_path, extra_path))
    }

    pub fn translate_path(&self, path: &str) -> PathBuf {
        self.document_root.join(path)
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
}
