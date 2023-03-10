mod response {
    #[link(wasm_import_module = "response")]
    extern "C" {
        pub fn set_status(status: u32);
    }
}

mod message {
    pub enum ResponseReference {}

    #[link(wasm_import_module = "message")]
    extern "C" {
        pub fn call() -> *const ResponseReference;
        pub fn status(reference: *const ResponseReference) -> u32;
    }
}

fn main() {
    let reference = unsafe { message::call() };

    unsafe { response::set_status(message::status(reference)) }
}
