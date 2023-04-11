use crate::runtime::connection::FromGuest;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tortuga_guest::Response;

pub struct ResponseFuture;

impl Future for ResponseFuture {
    type Output = Response<FromGuest>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}
