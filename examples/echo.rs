#![no_main]

// Define the functions that this module will use from the outside world.
// In general, the set of this functions is what we define as an ABI.
// Here we define the "host" namespace for the imports,
// Otherwise it will be "env" by default
#[link(wasm_import_module = "system")]
extern "C" {
    /// Sends a message to the system by passing the memory address of the start of the message.
    fn send(address: u32, length: u32);
}

const MAX_REQUEST_SIZE: usize = 4096;
static mut REQUES_BUFFER: [u8; MAX_REQUEST_SIZE] = [0; MAX_REQUEST_SIZE];

#[no_mangle]
pub unsafe fn request_buffer() -> *const u8 {
    REQUES_BUFFER.as_ptr()
}

#[no_mangle]
pub unsafe fn receive(address: u32, length: u32) {
    send(address, length);
}