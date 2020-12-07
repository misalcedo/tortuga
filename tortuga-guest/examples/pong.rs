static MESSAGE: &str = "Pong!\n";

#[link(wasm_import_module = "system")]
extern "C" {
    fn send(recipient: u128, address: *const u8, length: usize);
}

#[no_mangle]
pub unsafe fn receive(sender: u128, _address: *const u8, _length: usize) {
    send(sender, MESSAGE.as_ptr(), MESSAGE.len());
}
