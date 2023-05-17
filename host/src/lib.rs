//! Public interface of the tortuga compiler and runtime.

extern crate core;

pub mod executor;
pub mod stream;
pub mod wasm;

// TODO: Introduce a host server using hyper and http3
// TODO: Use the new model instead of the guest library.
// TODO: Re-implement guest using the new model.
