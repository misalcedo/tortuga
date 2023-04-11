use std::io::{Seek, SeekFrom};
use std::num::NonZeroUsize;

use tortuga_guest::{
    Bidirectional, Destination, FrameIo, MemoryStream, ReadOnly, Request, Response, Source,
};

pub type ForGuest = MemoryStream<Bidirectional>;
pub type FromGuest = FrameIo<MemoryStream<ReadOnly>>;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Connection {
    primary: MemoryStream<Bidirectional>,
    streams: Vec<MemoryStream<Bidirectional>>,
}

impl Connection {
    pub fn new(request: Request<ForGuest>) -> Self {
        let mut primary = MemoryStream::default();

        primary.write_message(request).unwrap();
        primary.seek(SeekFrom::Start(0)).unwrap();

        Connection {
            primary,
            streams: Default::default(),
        }
    }

    pub fn stream(&mut self, stream: u64) -> Option<&mut MemoryStream<Bidirectional>> {
        match stream {
            0 => Some(&mut self.primary),
            _ => {
                NonZeroUsize::new(stream as usize).and_then(|id| self.streams.get_mut(id.get() - 1))
            }
        }
    }

    pub fn start_stream(&mut self) -> u64 {
        self.streams.push(Default::default());
        self.streams.len() as u64
    }

    pub fn response(self) -> Response<FromGuest> {
        self.primary.readable().read_message().unwrap()
    }
}
