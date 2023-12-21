use crate::about;
use std::ffi::OsStr;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;

pub struct ServerContext {
    script: PathBuf,
    script_path: PathBuf,
    ip_address: String,
    port: String,
    path: &'static str,
    software: String,
    signature: String,
}

impl ServerContext {
    pub fn new(address: SocketAddr, script: PathBuf) -> io::Result<Self> {
        let script_path = script.canonicalize()?;

        let ip_address = address.ip().to_string();
        let port = address.port().to_string();

        let path: &'static str = env!("PATH");

        let software = format!("{}/{}", about::PROGRAM, about::VERSION);
        let signature = format!(
            "<address>{} Server at {} Port {}</address>\n",
            software, ip_address, port
        );

        Ok(Self {
            script,
            script_path,
            ip_address,
            port,
            path,
            software,
            signature,
        })
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn software(&self) -> &str {
        self.software.as_str()
    }

    pub fn signature(&self) -> &str {
        self.signature.as_str()
    }

    pub fn script_filename(&self) -> &OsStr {
        self.script_path.as_os_str()
    }

    pub fn script_name(&self) -> &OsStr {
        self.script.as_os_str()
    }

    pub fn ip_address(&self) -> &str {
        self.ip_address.as_str()
    }

    pub fn port(&self) -> &str {
        self.port.as_str()
    }
}
