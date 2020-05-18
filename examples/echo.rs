// Declaring our library as `no-std` unconditionally lets us be consistent
// in how we `use` items from `std` or `core`
#![no_std]
#![no_main]

// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

// When we're building for a no-std target, we pull in `core`, but alias
// it as `std` so the `use` statements are the same between `std` and `core`.
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

// Define the functions that this module will use from the outside world.
// In general, the set of this functions is what we define as an ABI.
// Here we define the "host" namespace for the imports,
// Otherwise it will be "env" by default
#[link(wasm_import_module = "system")]
extern "C" {
    /// Sends a message to the system by passing the memory address of the start of the message.
    fn send(address: *const u8, length: usize);
}

#[no_mangle]
pub unsafe fn receive(address: *const u8, length: usize) {
    send(address, length);
}