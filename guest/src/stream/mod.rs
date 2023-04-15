#[cfg(not(feature = "mock"))]
mod host;

#[cfg(feature = "mock")]
mod memory;

mod direction;

pub use direction::{Bidirectional, ReadOnly, Readable, Writable, WriteOnly};

#[cfg(not(feature = "mock"))]
pub use host::Stream;

#[cfg(feature = "mock")]
pub use memory::Stream;
