use std::collections::HashMap;
use std::io::Cursor;
use std::num::NonZeroU64;
use tortuga_guest::{Destination, FrameIo, Request, Response, Source};

pub type ForGuest = Cursor<Vec<u8>>;
pub type FromGuest = FrameIo<Cursor<Vec<u8>>>;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BidirectionalStream {
    pub(crate) host_to_guest: Cursor<Vec<u8>>,
    pub(crate) guest_to_host: Cursor<Vec<u8>>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Connection {
    primary: BidirectionalStream,
    streams: HashMap<NonZeroU64, BidirectionalStream>,
}

impl Connection {
    pub fn new(request: Request<ForGuest>) -> Self {
        let mut primary = BidirectionalStream::default();

        primary.host_to_guest.write_message(request).unwrap();
        primary.host_to_guest.set_position(0);

        Connection {
            primary,
            streams: Default::default(),
        }
    }
    pub fn stream(&mut self, stream: u64) -> Option<&mut BidirectionalStream> {
        match stream {
            0 => Some(&mut self.primary),
            _ => NonZeroU64::new(stream).and_then(|id| self.streams.get_mut(&id)),
        }
    }

    pub fn start_stream(&mut self) -> u64 {
        let id = 1 + self.streams.len() as u64;

        self.streams
            .insert(NonZeroU64::new(id).unwrap(), Default::default());

        id
    }

    pub fn response(self) -> Response<FromGuest> {
        let message: std::io::Result<Response<FrameIo<Cursor<Vec<u8>>>>> =
            self.primary.guest_to_host.read_message();

        message.unwrap()
    }
}
