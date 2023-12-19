use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Child;
use httparse::{EMPTY_HEADER, Status};

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

use crate::board::{SlotIndex, SwitchBoard};
use crate::cgi;

const LISTENER: Token = Token(0);

struct Client {
    stream: TcpStream,
    buffer: Vec<u8>,
    child: Option<Child>
}

enum ClientError {
    IO(io::Error),
    HTTP(httparse::Error)
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::IO(e) => Display::fmt(&e, f),
            ClientError::HTTP(e) => Display::fmt(&e, f)
        }
    }
}

impl From<io::Error> for ClientError {
    fn from(value: io::Error) -> Self {
        ClientError::IO(value)
    }
}

impl From<httparse::Error> for ClientError {
    fn from(value: httparse::Error) -> Self {
        ClientError::HTTP(value)
    }
}

impl Client {
    pub fn handle(&mut self, script: &PathBuf) -> Result<usize, ClientError> {
        let bytes_read = self.stream.read_to_end(&mut self.buffer)?;

        let mut headers = vec![EMPTY_HEADER; 16];
        let mut request = httparse::Request::new(&mut headers);

        if let Status::Complete(index) = request.parse(&self.buffer)? {
            let args: Option<String> = None;
            let env: Option<(String, String)> = None;

            let mut child = cgi::spawn(script, args, env)?;

            if let Some(stdin) = &mut child.stdin {
                stdin.write_all(&self.buffer[index..])?;
            }

            self.child = Some(child);
        }

        Ok(bytes_read)
    }
}

impl From<TcpStream> for Client {
    fn from(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::with_capacity(1024 * 16),
            child: None
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
            self.poll.poll(&mut self.events, None)?;

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
                            match client.handle(&script) {
                                Ok(0) => {
                                    self.switch_board.remove(SlotIndex::new(token));
                                    break;
                                }
                                Ok(_) => {}
                                Err(ClientError::IO(e)) if e.kind() == io::ErrorKind::WouldBlock => {
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

    #[test]
    fn new_connections() {
        let address = SocketAddr::from(([127, 0, 0, 1], 0));
        let _server = Server::new(address, 1).unwrap();
    }
}
