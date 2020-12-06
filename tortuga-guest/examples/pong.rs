static MESSAGE: &str = "Pong!\n";

#[link(wasm_import_module = "system")]
extern "C" {
    fn send(address: *const u8, length: usize);
}

#[no_mangle]
pub unsafe fn receive(_address: *const u8, _length: usize) {
    send(MESSAGE.as_ptr(), MESSAGE.len());
}
