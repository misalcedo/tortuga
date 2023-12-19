use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, SocketAddr};
use std::path::PathBuf;

use httparse::{Request, Response};

use mio::{Events, Poll, Interest, Token};
use mio::net::TcpStream;

use crate::cgi;

pub fn serve(script: PathBuf) -> io::Result<()> {
    // Bind a server socket to connect to.
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = TcpListener::bind(addr)?;

    // Construct a new `Poll` handle as well as the `Events` we'll store into
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);

    // cgi::run(&self.script, vec![""], HashMap::from([("", "")]))
    
    // Register the server with `Poll`
    poll.registry().register(&mut server, Token(0), Interest::READABLE | Interest::WRITABLE)?;

    // Wait for the socket to become ready. This has to happens in a loop to
    // handle spurious wakeups.
    loop {
        // Register the stream with `Poll`
        poll.registry().register(&mut stream, Token(0), Interest::READABLE | Interest::WRITABLE)?;
    
        poll.poll(&mut events, None)?;

        for event in &events {
            if event.token() == Token(0) && event.is_writable() {
                // The socket connected (probably, it could still be a spurious
                // wakeup)
                return Ok(());
            }
        }
    }
}
