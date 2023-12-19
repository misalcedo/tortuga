use std::collections::LinkedList;
use std::io;
use std::io::Read;
use std::net::SocketAddr;
use std::ops::{Index, IndexMut};

use mio::{Events, Poll, Interest, Token};
use mio::net::{TcpListener, TcpStream};

enum Socket {
    Listener(TcpListener),
    Request(Vec<u8>, TcpStream),
    Response(usize, TcpStream),
}

impl Socket {
    fn register(&mut self, token: usize, poll: &mut Poll) -> io::Result<()> {
        match self {
            Socket::Listener(listener) => {
                poll.registry().register(listener, Token(token), Interest::READABLE)
            }
            Socket::Request(_, client) => {
                poll.registry().register(client, Token(token), Interest::READABLE)
            }
            Socket::Response(_, client) => {
                poll.registry().reregister(client, Token(token), Interest::READABLE | Interest::WRITABLE)
            }
        }
    }
}

impl From<TcpListener> for Socket {
    fn from(value: TcpListener) -> Self {
        Self::Listener(value)
    }
}

impl From<TcpStream> for Socket {
    fn from(value: TcpStream) -> Self {
        Self::Request(Vec::with_capacity(1024 * 16), value)
    }
}

struct SwitchBoard {
    slots: Vec<Option<Socket>>,
    available: LinkedList<usize>,
}

impl Default for SwitchBoard {
    fn default() -> Self {
        SwitchBoard {
            slots: Vec::new(),
            available: LinkedList::new(),
        }
    }
}

impl SwitchBoard {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            available: LinkedList::new(),
        }
    }

    pub fn add<S: Into<Socket>>(&mut self, socket: S, poll: &mut Poll) -> io::Result<()> {
        let index = self.next_slot();
        let mut socket = socket.into();

        socket.register(index, poll)?;

        self.slots[index] = Some(socket);

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> bool {
        let length = self.slots.len();
        let last_index = length.checked_sub(1).unwrap_or(0);

        if index < last_index {
            self.available.push_back(index);
            self.slots[index].take().is_some()
        } else if index == last_index {
            self.slots.pop().is_some()
        } else {
            false
        }
    }

    pub fn accept(&mut self) {
        Unsupported
    }

    fn next_slot(&mut self) -> usize {
        self.available.pop_front().unwrap_or_else(|| {
            let index = self.slots.len();
            self.slots.reserve(1);
            self.slots.push(None);
            index
        })
    }
}

impl Index<usize> for SwitchBoard {
    type Output = Socket;

    fn index(&self, index: usize) -> &Self::Output {
        self.slots.index(index).as_ref().expect("Indexed an empty slot.")
    }
}

impl IndexMut<usize> for SwitchBoard {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.slots.index_mut(index).as_mut().expect("Indexed an empty slot.")
    }
}

pub struct Server {
    switch_board: SwitchBoard,
    poll: Poll,
    events: Events,
}

impl Server {
    pub fn new(address: SocketAddr, capacity: usize) -> io::Result<Self> {
        let mut poll = Poll::new()?;
        let mut switch_board = SwitchBoard::with_capacity(capacity);

        switch_board.add(TcpListener::bind(address)?, &mut poll)?;

        Ok(Self {
            switch_board,
            poll,
            events: Events::with_capacity(capacity),
        })
    }

    pub fn serve(mut self) -> io::Result<()> {
        loop {
            self.poll.poll(&mut self.events, None)?;

            for event in &self.events {
                let index = event.token().0;

                match &mut self.switch_board[index] {
                    Socket::Listener(listener) => {
                        loop {
                            match listener.accept() {
                                Ok((client, _)) => {
                                    self.switch_board.add(client, &mut self.poll)?;
                                }
                                Err(e) if e.kind() == io::ErrorKind::WouldBlock => { break; }
                                Err(e) => { return Err(e); }
                            }
                        }
                    }
                    Socket::Request(buffer, client) => {
                        loop {
                            match client.read_to_end(buffer) {
                                Ok(0) => {
                                    self.switch_board.remove(index);
                                    break;
                                }
                                Ok(_) => {
                                    let mut headers = Vec::with_capacity(16);
                                    let mut request = httparse::Request::new(&mut headers);

                                    match request.parse(buffer) {
                                        Ok(result) if result.is_complete() => {

                                        }
                                        Ok(result) => {

                                        }
                                        Err(e) => {
                                            eprintln!("HTTP parse error {e}");
                                            self.switch_board.remove(index);
                                            break;
                                        }
                                    }
                                },
                                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("Unexpected socket error {e}");
                                    self.switch_board.remove(index);
                                    break;
                                }
                            }
                        }
                    }
                    Socket::Response(remaining, client) => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use std::thread;
    use std::thread::Thread;
    use std::time::Duration;
    use slab::Slab;
    use super::*;

    #[test]
    fn new_connections() {
        let address = SocketAddr::from(([127, 0, 0, 1], 0));
        let mut server = Server::new(address, 1).unwrap();
    }
}
