use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll, Waker};

#[derive(Clone, Debug, Default)]
pub struct State<T> {
    inner: Option<T>,
    waker: Option<Waker>,
}

#[derive(Debug, Default)]
pub struct Promise<T>(Arc<RwLock<State<T>>>);

impl<T> Clone for Promise<T> {
    fn clone(&self) -> Self {
        Promise(Arc::clone(&self.0))
    }
}

impl<T> Promise<T> {
    pub fn complete(&mut self, value: T) {
        let mut guard = self.0.write().unwrap();

        guard.inner = Some(value);

        if let Some(waker) = guard.waker.take() {
            waker.wake();
        }
    }
}

impl<T> Future for Promise<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.write().unwrap();

        match guard.inner.take() {
            Some(value) => Poll::Ready(value),
            None => {
                guard.waker = Some(cx.waker().clone());

                Poll::Pending
            }
        }
    }
}
