use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::time::Duration;

use crate::board::{SlotIndex, SwitchBoard};
use crate::client::Client;
use crate::poll::Poller;

const LISTENER: usize = 0;

pub struct Server {
    listener: TcpListener,
    poll: Poller,
    switch_board: SwitchBoard<Client>,
}

impl Server {
    pub fn new(address: SocketAddr, capacity: usize) -> io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        let switch_board = SwitchBoard::with_capacity(capacity);
        let mut poll = Poller::new(capacity)?;

        poll.register(listener.as_raw_fd(), LISTENER, false)?;

        Ok(Self {
            listener,
            poll,
            switch_board,
        })
    }

    pub fn serve(mut self, script: PathBuf) -> io::Result<()> {
        loop {
            self.poll.poll()?;

            for hint in self.poll.hints() {
                let token = hint.token();

                if token == LISTENER {
                    loop {
                        match self.listener.accept() {
                            Ok(client) => {
                                let file_descriptor = client.0.as_raw_fd();
                                let slot = self.switch_board.add(client.into());

                                self.poll.register(file_descriptor, slot.get(), false)?;
                            }
                            Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                                continue;
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                } else {
                    let client = &mut self.switch_board[token - 1];

                    loop {
                        match client.handle(&script) {
                            Ok(0) => {
                                self.switch_board.remove(SlotIndex::new(token));
                                break;
                            }
                            Ok(_) => {
                                continue;
                            }
                            Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                                continue;
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                eprintln!("Unexpected socket error {e}");
                                self.switch_board.remove(SlotIndex::new(token));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;

    fn new_connections() {
        let server = Server::new(SocketAddr::from(([127, 0, 0, 1], 0)), 1).unwrap();
        let address = server.listener.local_addr().unwrap();

        let thread = thread::spawn(|| server.serve("../examples/hello.cgi".into()));

        let mut client = TcpStream::connect_timeout(&address, Duration::from_millis(50)).unwrap();
        let mut output = String::new();

        client.write_all(b"Hi!").unwrap();
        client.read_to_string(&mut output).unwrap();

        assert_eq!(output.as_str(), "Hello, World!");
    }
}
