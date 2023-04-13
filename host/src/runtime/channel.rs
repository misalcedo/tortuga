use std::io::{Cursor, Read, Write};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tortuga_guest::Body;

#[derive(Debug)]
pub struct Sender<T>(UnboundedSender<T>);

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

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
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
        let (sender, receiver) = channel();

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
}
