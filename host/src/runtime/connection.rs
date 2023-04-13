use std::num::NonZeroUsize;

use crate::runtime::channel::ChannelStream;
use tortuga_guest::{FrameIo, Response, Source};

pub type FromGuest = FrameIo<ChannelStream>;

#[derive(Debug, Default)]
pub struct Connection {
    primary: ChannelStream,
    streams: Vec<ChannelStream>,
}

impl Connection {
    pub fn new(primary: ChannelStream) -> Self {
        Connection {
            primary,
            streams: Default::default(),
        }
    }

    pub fn stream(&mut self, stream: u64) -> Option<&mut ChannelStream> {
        match stream {
            0 => Some(&mut self.primary),
            _ => {
                NonZeroUsize::new(stream as usize).and_then(|id| self.streams.get_mut(id.get() - 1))
            }
        }
    }

    pub fn add_stream(&mut self, stream: ChannelStream) -> u64 {
        self.streams.push(stream);
        self.streams.len() as u64
    }

    pub fn response(self) -> Response<FromGuest> {
        self.primary.read_message().unwrap()
    }
}
