use std::future::Future;
use std::io::{Read, Write};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;

use crate::network::ring::Buffer;
use tortuga_model::asynchronous;

#[derive(Clone, Debug)]
pub struct Pipe {
    buffer: Arc<RwLock<Buffer>>,
    closed: Arc<AtomicBool>,
    blocking: bool,
    read_waker: Option<Waker>,
    write_waker: Option<Waker>,
}

impl Pipe {
    pub fn new(capacity: usize) -> Self {
        Pipe {
            buffer: Arc::new(RwLock::new(Buffer::new(capacity))),
            closed: Arc::new(AtomicBool::new(false)),
            blocking: true,
            read_waker: None,
            write_waker: None,
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        self.blocking = blocking;
    }

    pub fn shutdown(&mut self) {
        self.closed.store(true, Ordering::SeqCst);
    }

    fn is_empty(&self) -> bool {
        let guard = self.read_lock();
        guard.is_empty()
    }

    fn is_full(&self) -> bool {
        let guard = self.read_lock();
        guard.is_full()
    }

    fn write_lock(&mut self) -> RwLockWriteGuard<'_, Buffer> {
        match RwLock::write(&self.buffer) {
            Ok(guard) => guard,
            Err(e) => {
                let mut guard = e.into_inner();
                guard.clear();
                guard
            }
        }
    }

    fn read_lock(&self) -> RwLockReadGuard<'_, Buffer> {
        match RwLock::read(&self.buffer) {
            Ok(guard) => guard,
            Err(e) => {
                self.closed.store(true, Ordering::SeqCst);
                e.into_inner()
            }
        }
    }

    fn readable(&mut self) -> ReadFuture<'_> {
        ReadFuture(self)
    }

    fn writable(&mut self) -> WriteFuture<'_> {
        WriteFuture(self)
    }
}

impl Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.blocking && self.is_empty() {
            Err(std::io::ErrorKind::WouldBlock.into())
        } else if self.closed.load(Ordering::SeqCst) && self.is_empty() {
            Ok(0)
        } else {
            loop {
                let mut guard = self.write_lock();

                if !guard.is_empty() {
                    return Ok(guard.read(buf));
                }
            }
        }
    }
}

impl Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.blocking && self.is_full() {
            Err(std::io::ErrorKind::WouldBlock.into())
        } else if self.closed.load(Ordering::SeqCst) {
            Err(std::io::ErrorKind::BrokenPipe.into())
        } else {
            loop {
                let mut guard = self.write_lock();

                if !guard.is_full() {
                    return Ok(guard.write(buf));
                }
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct ReadFuture<'a>(&'a mut Pipe);

#[async_trait]
impl asynchronous::Read for Pipe {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

pub struct WriteFuture<'a>(&'a mut Pipe);

#[async_trait]
impl asynchronous::Write for Pipe {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn read_write() {
        let input = [42];
        let mut output = [0];
        let mut pipe = Pipe::new(1);

        pipe.set_blocking(false);

        assert_eq!(pipe.write(&input).unwrap(), 1);
        assert_eq!(
            pipe.write(&input).unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(pipe.write(&input).unwrap(), 1);

        assert_eq!(input, output);
    }

    #[test]
    fn multithreaded() {
        let input = [42];
        let mut output = [0];
        let mut pipe = Pipe::new(1);
        let mut clone = pipe.clone();

        assert_eq!(pipe.write(&input).unwrap(), 1);

        let handle = thread::Builder::new()
            .name("reader".to_string())
            .spawn(move || {
                let mut bytes = [0];

                assert_eq!(clone.read(&mut bytes).unwrap(), 1);
                assert_eq!([42], bytes);
            })
            .unwrap();

        assert_eq!(pipe.write(&input).unwrap(), 1);
        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(pipe.write(&input).unwrap(), 1);

        assert_eq!(input, output);

        handle.join().unwrap();
    }

    #[test]
    fn looped() {
        let input = [42; 2];
        let mut output = [0; 1];
        let mut pipe = Pipe::new(3);

        pipe.set_blocking(false);

        assert_eq!(pipe.write(&input).unwrap(), 2);
        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(output, [42]);
        assert_eq!(pipe.write(&input).unwrap(), 2);
        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(output, [42]);
        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(output, [42]);
        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(output, [42]);
        assert_eq!(
            pipe.read(&mut output).unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        assert_eq!(output, [42]);
    }

    #[test]
    fn closed() {
        let input = [42];
        let mut output = [0];
        let mut pipe = Pipe::new(1);

        assert_eq!(pipe.write(&input).unwrap(), 1);

        pipe.shutdown();

        assert_eq!(pipe.read(&mut output).unwrap(), 1);
        assert_eq!(
            pipe.write(&input).unwrap_err().kind(),
            std::io::ErrorKind::BrokenPipe
        );
        assert_eq!(pipe.read(&mut output).unwrap(), 0);
    }

    async fn read_pipe(mut pipe: Pipe) {
        let mut bytes = [0];

        assert_eq!(
            asynchronous::Read::read(&mut pipe, &mut bytes)
                .await
                .unwrap(),
            1
        );
        assert_eq!([42], bytes);
    }

    #[tokio::test]
    async fn asynchronous() {
        let input = [42];
        let mut output = [0];
        let mut pipe = Pipe::new(1);
        let mut clone = pipe.clone();

        assert_eq!(
            asynchronous::Write::write(&mut pipe, &input).await.unwrap(),
            1
        );

        let mut handle = tokio::spawn(read_pipe(clone));

        assert_eq!(
            asynchronous::Write::write(&mut pipe, &input).await.unwrap(),
            1
        );
        assert_eq!(
            asynchronous::Read::read(&mut pipe, &mut output)
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            asynchronous::Write::write(&mut pipe, &input).await.unwrap(),
            1
        );

        assert_eq!(input, output);

        handle.await.unwrap();
    }
}
