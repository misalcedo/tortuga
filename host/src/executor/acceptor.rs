use crate::executor;
use crate::executor::{Acceptor, Identifier, Message, Router};
use async_trait::async_trait;

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
impl<Provider> Acceptor for RoutingAcceptor<Provider, Identifier>
where
    Provider: executor::Provider,
{
    type Stream = Provider::Stream;

    fn try_accept(&mut self) -> Option<Message<Self::Stream>> {
        let stream = self.provider.try_next()?;

        Some(Message::new(Identifier::default(), stream))
    }

    async fn accept(&mut self) -> Message<Self::Stream> {
        let stream = self.provider.next().await;

        Message::new(Identifier::default(), stream)
    }
}
