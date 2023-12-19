use std::collections::LinkedList;
use std::io;
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut};

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const LISTENER: Token = Token(0);

enum Socket {
    Request(Vec<u8>, TcpStream),
    Response(usize, TcpStream),
}

impl From<TcpStream> for Socket {
    fn from(value: TcpStream) -> Self {
        Self::Request(Vec::with_capacity(1024 * 16), value)
    }
}

// A naturally indexed (starting at 1) slab of client sockets.
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

    pub fn set(&mut self, slot: NonZeroUsize, client: TcpStream) {
        if let Some(s) = self.slots.get_mut(slot.get() - 1) {
            *s = Some(Socket::Request(Vec::with_capacity(1024 * 16), client));
        }
    }

    pub fn remove(&mut self, slot: usize) -> Option<bool> {
        let length = self.slots.len();

        if slot > length {
            None
        } else if slot == length {
            Some(self.slots.pop().is_some())
        } else {
            let index = slot.checked_sub(1)?;

            self.available.push_back(index);

            Some(self.slots[index].take().is_some())
        }
    }

    pub fn claim_slot(&mut self) -> NonZeroUsize {
        self.available
            .pop_front()
            .and_then(NonZeroUsize::new)
            .unwrap_or_else(|| {
                let index = self.slots.len();

                self.slots.reserve(1);
                self.slots.push(None);

                NonZeroUsize::new(index + 1).unwrap_or(NonZeroUsize::MIN)
            })
    }
}

impl Index<usize> for SwitchBoard {
    type Output = Socket;

    fn index(&self, index: usize) -> &Self::Output {
        self.slots
            .index(index)
            .as_ref()
            .expect("Indexed an empty slot.")
    }
}

impl IndexMut<usize> for SwitchBoard {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.slots
            .index_mut(index)
            .as_mut()
            .expect("Indexed an empty slot.")
    }
}

pub struct Server {
    listener: TcpListener,
    poll: Poll,
    switch_board: SwitchBoard,
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

    pub fn serve(mut self) -> io::Result<()> {
        loop {
            self.poll.poll(&mut self.events, None)?;

            for event in &self.events {
                match event.token() {
                    LISTENER => loop {
                        match self.listener.accept() {
                            Ok((mut client, _)) => {
                                let slot = self.switch_board.claim_slot();

                                self.poll.registry().register(
                                    &mut client,
                                    Token(slot.get()),
                                    Interest::READABLE,
                                )?;
                                self.switch_board.set(slot, client);
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    Token(_slot) => {
                        loop {
                            // match client.read_to_end(buffer) {
                            //     Ok(0) => {
                            //         self.switch_board.remove(index);
                            //         break;
                            //     }
                            //     Ok(_) => {
                            //         let mut headers = Vec::with_capacity(16);
                            //         let mut request = httparse::Request::new(&mut headers);

                            //         match request.parse(buffer) {
                            //             Ok(result) if result.is_complete() => {

                            //             }
                            //             Ok(result) => {

                            //             }
                            //             Err(e) => {
                            //                 eprintln!("HTTP parse error {e}");
                            //                 self.switch_board.remove(index);
                            //                 break;
                            //             }
                            //         }
                            //     },
                            //     Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            //         break;
                            //     }
                            //     Err(e) => {
                            //         eprintln!("Unexpected socket error {e}");
                            //         self.switch_board.remove(index);
                            //         break;
                            //     }
                            // }
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
