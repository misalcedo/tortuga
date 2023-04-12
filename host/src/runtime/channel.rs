use std::io::{Cursor, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use tortuga_guest::Body;

#[derive(Debug)]
pub struct ChannelStream {
    receiver: StreamReceiver,
    sender: StreamSender,
}

#[derive(Debug)]
pub struct StreamReceiver {
    channel: Receiver<Vec<u8>>,
    reader: Cursor<Vec<u8>>,
}

#[derive(Debug)]
pub struct StreamSender {
    channel: Sender<Vec<u8>>,
}

impl Default for ChannelStream {
    fn default() -> Self {
        let (sender, receiver) = channel();

        ChannelStream {
            receiver: StreamReceiver {
                channel: receiver,
                reader: Default::default(),
            },
            sender: StreamSender { channel: sender },
        }
    }
}

impl Body for ChannelStream {
    fn len(&mut self) -> Option<usize> {
        None
    }
}

impl ChannelStream {
    pub fn split(self) -> (StreamSender, StreamReceiver) {
        (self.sender, self.receiver)
    }
}

impl Read for StreamReceiver {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.reader.position() == self.reader.get_ref().len() as u64 {
            if let Ok(bytes) = self.channel.recv() {
                self.reader = Cursor::new(bytes);
            };
        }

        self.reader.read(buf)
    }
}

impl Write for StreamSender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.channel.send(Vec::from(buf)).unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_read() {
        let content = b"Hello, World!";
        let mut stream = ChannelStream::default();
        let mut buffer = Cursor::new(Vec::new());

        stream.write_all(content).unwrap();
        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), content)
    }
}
