use std::net::SocketAddr;

pub struct ClientContext {
    remote_address: SocketAddr,
    remote_ip_address: String,
    remote_port: String,
}

impl ClientContext {
    pub fn new(remote_address: SocketAddr) -> Self {
        let remote_ip_address = remote_address.ip().to_string();
        let remote_port = remote_address.port().to_string();

        Self {
            remote_address,
            remote_ip_address,
            remote_port,
        }
    }

    pub fn remote_ip_address(&self) -> &str {
        self.remote_ip_address.as_str()
    }

    pub fn remote_port(&self) -> &str {
        self.remote_port.as_str()
    }
}
