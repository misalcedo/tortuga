static MESSAGE: &str = "Ping!\n";

#[link(wasm_import_module = "pong")]
extern "C" {
    fn send(address: *const u8, length: usize);
}

#[no_mangle]
pub unsafe fn receive(sender: u128, _address: *const u8, _length: usize) {
    send(MESSAGE.as_ptr(), MESSAGE.len());
}
