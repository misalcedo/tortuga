use std::io;
use std::os::fd::RawFd;

use mio::event::Event;
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};

const LISTENER: Token = Token(0);

pub struct Poller {
    poll: Poll,
    events: Events,
}

#[repr(transparent)]
pub struct Hint<'a>(&'a Event);

impl<'a> From<&'a Event> for Hint<'a> {
    fn from(value: &'a Event) -> Self {
        Hint(value)
    }
}

impl<'a> Hint<'a> {
    pub fn token(&self) -> usize {
        self.0.token().0
    }

    pub fn is_readable(&self) -> bool {
        self.0.is_readable()
    }

    pub fn is_writable(&self) -> bool {
        self.0.is_writable()
    }
}

impl Poller {
    pub fn new(capacity: usize) -> io::Result<Self> {
        let poll = Poll::new()?;
        let events = Events::with_capacity(capacity);

        Ok(Self { poll, events })
    }

    pub fn register(&self, source: RawFd, token: usize, writable: bool) -> io::Result<()> {
        let mut interest = Interest::READABLE;

        if writable {
            interest |= Interest::WRITABLE;
        }

        self.poll
            .registry()
            .register(&mut SourceFd(&source), Token(token), interest)
    }

    pub fn poll(&mut self) -> io::Result<()> {
        match self.poll.poll(&mut self.events, None) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => Ok(()),
            result => result,
        }
    }

    pub fn hints(&self) -> impl Iterator<Item = Hint> {
        self.events.iter().map(Hint::from)
    }
}
