#![feature(io_slice_advance)]
//! Public interface of the tortuga compiler and runtime.

// pub mod executor;
// pub mod stream;
pub mod wasm;

// See https://docs.rs/wasi-cap-std-sync/8.0.1/src/wasi_cap_std_sync/net.rs.html#332
// TODO: Introduce a host server using hyper and http3
// TODO: Use the new model instead of the guest library.
// TODO: Re-implement guest using the new model.
