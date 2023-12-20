use std::io::{self, Cursor, Read, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Child;
use std::time::Duration;

use mio::net::{TcpListener, TcpStream};
use mio::{event::Event, Events, Interest, Poll, Token};

use crate::board::{SlotIndex, SwitchBoard};
use crate::cgi;

const LISTENER: Token = Token(0);

struct Client {
    stream: TcpStream,
    read_buffer: Cursor<Vec<u8>>,
    write_buffer: Cursor<Vec<u8>>,
    child: Option<Child>,
}

impl Client {
    pub fn handle(&mut self, event: &Event, script: &PathBuf) -> io::Result<usize> {
        if event.is_readable() {
            // TODO: handle would block to allow write as well.
            io::copy(&mut self.stream, &mut self.read_buffer)?;
        }

        if event.is_writable() {
            io::copy(&mut self.write_buffer, &mut self.stream)?;
        }

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
            Some(child) => {
                if event.is_readable() {}

                Ok(0)
            }
        }
    }
}

impl From<TcpStream> for Client {
    fn from(stream: TcpStream) -> Self {
        Self {
            stream,
            read_buffer: Cursor::new(Vec::with_capacity(1024 * 16)),
            write_buffer: Cursor::new(Vec::with_capacity(1024 * 16)),
            child: None,
        }
    }
}

pub struct Server {
    listener: TcpListener,
    poll: Poll,
    switch_board: SwitchBoard<Client>,
    events: Events,
}

impl Server {
    pub fn new(address: SocketAddr, capacity: usize) -> io::Result<Self> {
        let mut listener = TcpListener::bind(address)?;
        let poll = Poll::new()?;
        let switch_board = SwitchBoard::with_capacity(capacity);
        let events = Events::with_capacity(capacity);

        poll.registry()
            .register(&mut listener, LISTENER, Interest::READABLE)?;

        Ok(Self {
            listener,
            poll,
            switch_board,
            events,
        })
    }

    pub fn serve(mut self, script: PathBuf) -> io::Result<()> {
        loop {
            self.poll
                .poll(&mut self.events, Some(Duration::from_millis(10)))?;

            for event in &self.events {
                match event.token() {
                    LISTENER => loop {
                        match self.listener.accept() {
                            Ok((mut client, _)) => {
                                let slot = self.switch_board.reserve();
                                self.poll.registry().register(
                                    &mut client,
                                    Token(slot.get()),
                                    Interest::READABLE,
                                )?;
                                self.switch_board[slot] = Some(Client::from(client));
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    Token(token) => {
                        let client = &mut self.switch_board[token - 1];

                        loop {
                            match client.handle(&event, &script) {
                                Ok(0) => {
                                    self.switch_board.remove(SlotIndex::new(token));
                                    break;
                                }
                                Ok(_) => {}
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;

    #[test]
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
