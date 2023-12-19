use std::io;
use std::net::SocketAddr;

use mio::net::{TcpListener, TcpStream};

use crate::board::SwitchBoard;

const LISTENER: Token = Token(0);

struct Client {
    stream: TcpStream,
    buffer: Vec<u8>
}

impl From<TcpStream> for Client {
    fn from(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::with_capacity(1024 * 16)
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

    pub fn serve(mut self) -> io::Result<()> {
        loop {
            self.poll.poll(&mut self.events, None)?;

            for event in &self.events {
                match event.token() {
                    LISTENER => {
                        loop {
                            match self.listener.accept() {
                                Ok((mut client, _)) => {
                                    let slot = self.switch_board.reserve();
                                    self.poll.registry().register(&mut client, Token(slot.get()), Interest::READABLE)?;
                                    self.switch_board[slot] = Some(Client::from(client));
                                }
                                Err(e) if e.kind() == io::ErrorKind::WouldBlock => { break; }
                                Err(e) => { return Err(e); }
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
