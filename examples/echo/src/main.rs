mod request {
    #[link(wasm_import_module = "request")]
    extern "C" {
        pub fn send(address: *const u8, length: usize);
    }
}

mod response {
    #[link(wasm_import_module = "response")]
    extern "C" {
        pub fn status() -> u32;
        pub fn set_status(status: u32);
    }
}

fn main() {
    unsafe {
        response::set_status(201);
    }
}
