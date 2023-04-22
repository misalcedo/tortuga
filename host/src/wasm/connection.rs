use crate::wasm;
use std::num::NonZeroUsize;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Connection<Factory>
where
    Factory: wasm::Factory,
{
    factory: Factory,
    primary: Factory::Stream,
    rest: Vec<Factory::Stream>,
}

impl<Factory> Connection<Factory>
where
    Factory: wasm::Factory,
{
    pub fn new(primary: Factory::Stream, factory: Factory) -> Self {
        Connection {
            factory,
            primary,
            rest: Vec::new(),
        }
    }

    pub fn primary_mut(&mut self) -> &mut Factory::Stream {
        &mut self.primary
    }

    pub fn into_primary(self) -> Factory::Stream {
        self.primary
    }

    pub fn start_stream(&mut self) -> u64 {
        self.rest.push(self.factory.create());
        self.rest.len() as u64
    }

    pub fn get_mut(&mut self, stream: u64) -> Option<&mut Factory::Stream> {
        match NonZeroUsize::new(stream as usize) {
            None => Some(&mut self.primary),
            Some(position) => self.rest.get_mut(position.get() - 1),
        }
    }
}
