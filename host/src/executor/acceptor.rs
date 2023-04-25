use crate::executor::{Acceptor, Identifier, Message, Router};
use crate::{executor, wasm};
use async_trait::async_trait;
use std::io::{Read, Write};
use tortuga_guest::{Header, Request, Source};

pub struct RoutingAcceptor<Provider, Target> {
    provider: Provider,
    router: Router<Target>,
}

impl<Provider> RoutingAcceptor<Provider, Identifier>
where
    Provider: executor::Provider,
{
    pub fn new(provider: Provider, router: Router<Identifier>) -> Self {
        RoutingAcceptor { provider, router }
    }
}

#[async_trait]
impl<Provider, Stream> Acceptor for RoutingAcceptor<Provider, Identifier>
where
    Provider: executor::Provider<Stream = Stream>,
    Stream: wasm::Stream<Error = std::io::Error> + Read + Write,
{
    type Stream = Header<Provider::Stream>;

    fn try_accept(&mut self) -> Option<Message<Self::Stream>> {
        let stream = self.provider.try_next()?;
        let header: Header<_> = stream.read_message().unwrap();
        let request: Request<_> = header.read_message().unwrap();
        let identifier = self.router.route(request.uri().clone()).unwrap();

        Some(Message::new(identifier, request.into_body().finish()))
    }

    async fn accept(&mut self) -> Message<Self::Stream> {
        let stream = self.provider.next().await;
        let header: Header<_> = stream.read_message().unwrap();
        let request: Request<_> = header.read_message().unwrap();
        let identifier = self.router.route(request.uri().clone()).unwrap();

        Message::new(identifier, request.into_body().finish())
    }
}
