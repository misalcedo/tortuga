use std::io::{Cursor, Read, Write};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tortuga_guest::Body;

#[derive(Debug)]
pub struct Sender<T>(UnboundedSender<T>);

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Sender<T> {
    pub fn try_send(&self, item: T) -> Option<T> {
        let error = self.0.send(item).err()?;

        Some(error.0)
    }

    pub async fn send(&self, item: T) -> Option<T> {
        self.try_send(item)
    }
}

#[derive(Debug)]
pub enum ReceiveError {
    Empty,
    Disconnected,
}

#[derive(Debug)]
pub struct Receiver<T>(UnboundedReceiver<T>);

impl<T> Receiver<T> {
    pub fn try_recv(&mut self) -> Result<T, ReceiveError> {
        self.0.try_recv().map_err(|e| match e {
            TryRecvError::Empty => ReceiveError::Empty,
            TryRecvError::Disconnected => ReceiveError::Disconnected,
        })
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.0.recv().await
    }
}

pub fn new_channel<T>() -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = unbounded_channel();

    (Sender(sender), Receiver(receiver))
}

#[derive(Debug)]
pub struct ChannelStream {
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
    reader: Cursor<Vec<u8>>,
}

impl Default for ChannelStream {
    fn default() -> Self {
        let (sender, receiver) = new_channel();

        ChannelStream {
            sender,
            receiver,
            reader: Default::default(),
        }
    }
}

impl Body for ChannelStream {
    fn len(&mut self) -> Option<usize> {
        None
    }
}

impl ChannelStream {
    pub fn new() -> (Self, Self) {
        let mut client = Self::default();
        let mut server = Self::default();

        std::mem::swap(&mut client.sender, &mut server.sender);

        (client, server)
    }

    pub async fn read_async(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.reader.position() == self.reader.get_ref().len() as u64 {
            if let Some(bytes) = self.receiver.recv().await {
                self.reader = Cursor::new(bytes);
            };
        }

        self.reader.read(buf)
    }

    pub async fn write_async(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.sender.send(Vec::from(buf)).await {
            None => Ok(buf.len()),
            Some(_) => Ok(0),
        }
    }

    pub fn peek(&mut self) -> PeekingReader {
        PeekingReader(self, self.reader.position())
    }
}

#[derive(Debug)]
pub struct PeekingReader<'a>(&'a mut ChannelStream, u64);

impl<'a> Drop for PeekingReader<'a> {
    fn drop(&mut self) {
        self.0.reader.set_position(self.1);
    }
}

impl<'a> Read for PeekingReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.0.reader.position() == self.0.reader.get_ref().len() as u64 {
            if let Ok(bytes) = self.0.receiver.try_recv() {
                self.0.reader.get_mut().extend_from_slice(bytes.as_slice());
            };
        }

        self.0.reader.read(buf)
    }
}

impl Read for ChannelStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.reader.position() == self.reader.get_ref().len() as u64 {
            if let Ok(bytes) = self.receiver.try_recv() {
                self.reader = Cursor::new(bytes);
            };
        }

        self.reader.read(buf)
    }
}

impl Write for ChannelStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.sender.try_send(Vec::from(buf)) {
            None => Ok(buf.len()),
            Some(_) => Ok(0),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
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

        assert_eq!(buffer.get_ref().as_slice(), content);
    }

    #[test]
    fn peek_partial() {
        let content = b"Hello, World!";
        let mut stream = ChannelStream::default();
        let mut buffer = [0u8; 7];

        stream.write_all(content).unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let partial = &content[bytes..];

        assert_eq!(&buffer[..bytes], &content[..bytes]);

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream.peek(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), partial);

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), partial);
    }

    #[test]
    fn write_peek_read() {
        let content = b"Hello, World!";
        let mut stream = ChannelStream::default();
        let mut buffer = Cursor::new(Vec::new());

        stream.write_all(content).unwrap();
        std::io::copy(&mut stream.peek(), &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), content);

        let mut buffer = Cursor::new(Vec::new());

        std::io::copy(&mut stream, &mut buffer).unwrap();
        assert_eq!(buffer.get_ref().as_slice(), content);
    }

    #[test]
    fn crossed() {
        let content = b"Hello, World!";
        let (mut client, mut server) = ChannelStream::new();
        let mut buffer = Cursor::new(Vec::new());

        client.write_all(content).unwrap();
        std::io::copy(&mut server, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), content);

        let mut buffer = Cursor::new(Vec::new());

        server.write_all(content).unwrap();
        std::io::copy(&mut client, &mut buffer).unwrap();

        assert_eq!(buffer.get_ref().as_slice(), content);
    }

    #[tokio::test]
    async fn crossed_async() {
        let content = b"Hello, World!";
        let (mut client, mut server) = ChannelStream::new();
        let mut buffer = vec![0; content.len()];

        client.write_all(content).unwrap();
        server.read_async(&mut buffer).await.unwrap();

        assert_eq!(buffer.as_slice(), content);
    }
}
