use crate::cgi;
use std::io::{self, Cursor, Write};
use std::net::SocketAddr;
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::Child;

pub struct Client {
    stream: TcpStream,
    address: SocketAddr,
    read_buffer: Cursor<Vec<u8>>,
    write_buffer: Cursor<Vec<u8>>,
    child: Option<Child>,
}

impl Client {
    pub fn handle(&mut self, script: &PathBuf) -> io::Result<usize> {
        match self.child.as_mut() {
            None => {
                let args: Option<String> = None;
                let env: Option<(String, String)> = None;

                let mut child = cgi::spawn(script, args, env)?;

                if let Some(stdin) = &mut child.stdin {
                    std::io::copy(&mut self.read_buffer, stdin)?;

                    stdin.write(&self.read_buffer.get_ref()[0..])?;
                }

                self.child = Some(child);

                Ok(1)
            }
            Some(child) => Ok(0),
        }
    }
}

impl From<(TcpStream, SocketAddr)> for Client {
    fn from((stream, address): (TcpStream, SocketAddr)) -> Self {
        Self {
            stream,
            address,
            read_buffer: Cursor::new(Vec::with_capacity(1024 * 16)),
            write_buffer: Cursor::new(Vec::with_capacity(1024 * 16)),
            child: None,
        }
    }
}
