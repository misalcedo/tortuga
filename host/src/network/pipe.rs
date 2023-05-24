use std::future::Future;
use std::io::{Read, Write};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;

use crate::network::ring::Buffer;
use tortuga_model::asynchronous;

#[derive(Clone, Debug)]
pub struct Pipe {
    blocking: bool,
    closed: Arc<AtomicBool>,
    buffer: Arc<RwLock<Buffer>>,
    read_waker: Arc<Mutex<Option<Waker>>>,
    write_waker: Arc<Mutex<Option<Waker>>>,
}

impl Pipe {
    pub fn new(capacity: usize) -> Self {
        Pipe {
            blocking: true,
            closed: Arc::new(AtomicBool::new(false)),
            buffer: Arc::new(RwLock::new(Buffer::new(capacity))),
            read_waker: Arc::new(Mutex::new(None)),
            write_waker: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_blocking(&mut self, blocking: bool) {
        self.blocking = blocking;
    }

    pub fn shutdown(&mut self) {
        self.closed.store(true, Ordering::SeqCst);
    }

    fn is_empty(&self) -> bool {
        let guard = Self::read_lock(&self.buffer, &self.closed);
        guard.is_empty()
    }

    fn is_full(&self) -> bool {
        let guard = Self::read_lock(&self.buffer, &self.closed);
        guard.is_full()
    }

    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    fn take_waker(mutex: &Mutex<Option<Waker>>) -> Option<Waker> {
        match mutex.lock() {
            Ok(mut guard) => guard.take(),
            Err(e) => {
                let mut guard = e.into_inner();
                guard.take();
                None
            }
        }
    }

    fn set_waker(mutex: &Mutex<Option<Waker>>, cx: &mut Context) {
        let mut waker_guard = match mutex.lock() {
            Ok(guard) => guard,
            Err(e) => e.into_inner(),
        };

        *waker_guard = Some(cx.waker().clone());
    }

    fn write_lock(buffer: &RwLock<Buffer>) -> RwLockWriteGuard<'_, Buffer> {
        match RwLock::write(buffer) {
            Ok(guard) => guard,
            Err(e) => {
                let mut guard = e.into_inner();
                guard.clear();
                guard
            }
        }
    }

    fn read_lock<'a, 'b>(
        buffer: &'a RwLock<Buffer>,
        closed: &'b AtomicBool,
    ) -> RwLockReadGuard<'a, Buffer> {
        match RwLock::read(buffer) {
            Ok(guard) => guard,
            Err(e) => {
                closed.store(true, Ordering::SeqCst);
                e.into_inner()
            }
        }
    }

    fn readable(&mut self) -> ReadFuture<'_> {
        self.set_blocking(false);
        ReadFuture(self)
    }

    fn writable(&mut self) -> WriteFuture<'_> {
        self.set_blocking(false);
        WriteFuture(self)
    }
}

impl Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if !self.blocking && self.is_empty() {
            Err(std::io::ErrorKind::WouldBlock.into())
        } else if self.is_closed() && self.is_empty() {
            Ok(0)
        } else {
            loop {
                let mut guard = Self::write_lock(&self.buffer);

                if !guard.is_empty() {
                    if let Some(waker) = Self::take_waker(&self.write_waker) {
                        waker.wake();
                    }

                    return Ok(guard.read(buf));
                }
            }
        }
    }
}

impl Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.is_closed() {
            Err(std::io::ErrorKind::BrokenPipe.into())
        } else if !self.blocking && self.is_full() {
            Err(std::io::ErrorKind::WouldBlock.into())
        } else {
            loop {
                let mut guard = Self::write_lock(&self.buffer);

                if !guard.is_full() {
                    if let Some(waker) = Self::take_waker(&self.read_waker) {
                        waker.wake();
                    }

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

impl<'a> Future for ReadFuture<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let guard = Pipe::read_lock(&this.0.buffer, &this.0.closed);

        if this.0.is_closed() || !guard.is_empty() {
            Poll::Ready(())
        } else {
            Pipe::set_waker(&this.0.read_waker, cx);
            Poll::Pending
        }
    }
}

#[async_trait]
impl asynchronous::Read for Pipe {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            self.readable().await;

            match Read::read(self, buf) {
                Ok(bytes) => return Ok(bytes),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
    }
}

pub struct WriteFuture<'a>(&'a mut Pipe);

impl<'a> Future for WriteFuture<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let guard = Pipe::read_lock(&this.0.buffer, &this.0.closed);

        if this.0.is_closed() || !guard.is_full() {
            Poll::Ready(())
        } else {
            Pipe::set_waker(&this.0.write_waker, cx);
            Poll::Pending
        }
    }
}

#[async_trait]
impl asynchronous::Write for Pipe {
    async fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        loop {
            self.writable().await;

            match Write::write(self, buf) {
                Ok(bytes) => return Ok(bytes),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
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
        let clone = pipe.clone();

        assert_eq!(
            asynchronous::Write::write(&mut pipe, &input).await.unwrap(),
            1
        );

        let handle = tokio::spawn(read_pipe(clone));

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
