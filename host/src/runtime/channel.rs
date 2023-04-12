use std::io::{Cursor, Read, Write};
use std::num::NonZeroU64;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};

#[derive(Debug)]
pub struct ChannelStream {
    receiver: StreamReceiver,
    sender: StreamSender,
}

#[derive(Debug)]
pub struct StreamReceiver {
    identifier: u64,
    channel: Receiver<Vec<u8>>,
    reader: Cursor<Vec<u8>>,
}

#[derive(Debug)]
pub struct StreamSender {
    identifier: u64,
    channel: Sender<Vec<u8>>,
}

impl Default for ChannelStream {
    fn default() -> Self {
        Self::primary()
    }
}

impl From<Option<NonZeroU64>> for ChannelStream {
    fn from(identifier: Option<NonZeroU64>) -> Self {
        let (sender, receiver) = channel();
        let identifier = identifier.map(NonZeroU64::get).unwrap_or_default();

        ChannelStream {
            receiver: StreamReceiver {
                identifier,
                channel: receiver,
                reader: Default::default(),
            },
            sender: StreamSender {
                identifier,
                channel: sender,
            },
        }
    }
}

impl ChannelStream {
    pub fn primary() -> Self {
        Self::from(NonZeroU64::new(0))
    }

    pub fn new(identifier: NonZeroU64) -> Self {
        Self::from(Some(identifier))
    }

    pub fn split(self) -> (StreamSender, StreamReceiver) {
        (self.sender, self.receiver)
    }
}

impl Read for StreamReceiver {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.reader.position() == self.reader.get_ref().len() {
            if let Ok(bytes) = self.channel.1.recv() {
                self.reader = Cursor::new(bytes);
            };
        }

        self.reader.read(buf)
    }
}

impl Write for StreamSender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.channel.0.send(Vec::from(buf)).unwrap();

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for ChannelStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.receiver.read(buf)
    }
}

impl Write for ChannelStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sender.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.sender.flush()
    }
}
