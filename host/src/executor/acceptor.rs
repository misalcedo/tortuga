use crate::executor::{Acceptor, Identifier, Message, Router};
use crate::{executor, wasm};
use async_trait::async_trait;
use std::io::{Read, Write};
use tortuga_guest::{Header, Request, Source};

#[derive(Clone)]
pub struct RoutingAcceptor<Provider, Target> {
    provider: Provider,
    router: Router<Target>,
}

impl<Provider> RoutingAcceptor<Provider, Identifier> {
    pub fn provider_mut(&mut self) -> &mut Provider {
        &mut self.provider
    }

    pub fn router_mut(&mut self) -> &mut Router<Identifier> {
        &mut self.router
    }
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
        let mut stream = request.into_body().finish();

        stream.reset();

        Some(Message::new(identifier, stream))
    }

    async fn accept(&mut self) -> Message<Self::Stream> {
        let stream = self.provider.next().await;
        let header: Header<_> = stream.read_message().unwrap();
        let request: Request<_> = header.read_message().unwrap();
        let identifier = self.router.route(request.uri().clone()).unwrap();
        let mut stream = request.into_body().finish();

        stream.reset();

        Message::new(identifier, stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::memory;
    use crate::wasm::Factory;
    use std::io::Cursor;
    use tortuga_guest::{Destination, Method};

    #[test]
    fn route() {
        let mut factory = memory::Bridge::default();
        let mut router = Router::default();
        let mut acceptor = RoutingAcceptor::new(factory.clone(), router.clone());
        let mut client = factory.create();
        let mut buffer = Cursor::new(Vec::new());

        let body = b"Hello, World!";
        let identifier = Identifier::default();
        let request = Request::new(
            Method::Get,
            "/".into(),
            body.len(),
            Cursor::new(body.to_vec()),
        );

        client.write_message(request.clone()).unwrap();
        router.define("/".into(), identifier);

        let message = acceptor.try_accept().unwrap();

        assert_eq!(message.to(), identifier);

        let mut actual: Request<_> = message.into_inner().read_message().unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(request, actual);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    #[tokio::test]
    async fn async_route() {
        let mut factory = memory::Bridge::default();
        let mut router = Router::default();
        let mut acceptor = RoutingAcceptor::new(factory.clone(), router.clone());
        let mut client = factory.create();
        let mut buffer = Cursor::new(Vec::new());

        let body = b"Hello, World!";
        let identifier = Identifier::default();
        let request = Request::new(
            Method::Get,
            "/".into(),
            body.len(),
            Cursor::new(body.to_vec()),
        );

        client.write_message(request.clone()).unwrap();
        router.define("/".into(), identifier);

        let message = acceptor.accept().await;

        assert_eq!(message.to(), identifier);

        let mut actual: Request<_> = message.into_inner().read_message().unwrap();

        std::io::copy(actual.body(), &mut buffer).unwrap();

        assert_eq!(request, actual);
        assert_eq!(buffer.get_ref().as_slice(), body);
    }

    #[test]
    fn route_unknown() {
        let mut factory = memory::Bridge::default();
        let router = Router::default();
        let mut acceptor = RoutingAcceptor::new(factory.clone(), router.clone());
        let mut client = factory.create();

        let body = b"Hello, World!";
        let request = Request::new(
            Method::Get,
            "/".into(),
            body.len(),
            Cursor::new(body.to_vec()),
        );

        client.write_message(request).unwrap();

        assert!(acceptor.try_accept().is_none());
    }
}
