mod request {
    #[link(wasm_import_module = "request")]
    extern "C" {
        pub fn read(buffer: *mut u8, length: u32, start: u32) -> u32;
    }
}

mod response {
    #[link(wasm_import_module = "response")]
    extern "C" {
        pub fn status() -> u32;
        pub fn set_status(status: u32);
        pub fn write(buffer: *const u8, length: u32, end: u32) -> u32;
    }
}

fn main() {
    let buffer = vec![0; 4098];

    unsafe {
        let bytes = request::read(buffer.as_ptr(), buffer.len() as u32, 0);

        response::set_status(201);
        response::write(buffer.as_ptr(), buffer.len() as u32, bytes);
    }
}
