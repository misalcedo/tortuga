use crate::wasm;
use std::num::NonZeroUsize;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Connection<Factory, Stream> {
    factory: Factory,
    primary: Stream,
    rest: Vec<Stream>,
}

impl<Factory, Stream> Connection<Factory, Stream>
where
    Factory: wasm::Factory<Stream = Stream>,
    Stream: wasm::Stream,
{
    pub fn new(primary: Stream, factory: Factory) -> Self {
        Connection {
            factory,
            primary,
            rest: Vec::new(),
        }
    }
}

impl<Factory, Stream> Connection<Factory, Stream> {
    pub fn primary_mut(&mut self) -> &mut Stream {
        &mut self.primary
    }

    pub fn get_mut(&mut self, stream: u64) -> Option<&mut Stream> {
        match NonZeroUsize::new(stream as usize) {
            None => Some(&mut self.primary),
            Some(position) => self.rest.get_mut(position.get() - 1),
        }
    }

    pub fn into_primary(self) -> Stream {
        self.primary
    }
}

impl<Factory, Stream> Connection<Factory, Stream>
where
    Factory: wasm::Factory<Stream = Stream>,
    Stream: wasm::Stream,
{
    pub fn start_stream(&mut self) -> u64 {
        self.rest.push(self.factory.create());
        self.rest.len() as u64
    }
}
