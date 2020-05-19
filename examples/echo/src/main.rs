#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link(wasm_import_module = "system")]
extern "C" {
    fn send(address: *const u8, length: usize);
}

#[no_mangle]
pub unsafe fn receive(address: *const u8, length: usize) {
    send(address, length);
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
