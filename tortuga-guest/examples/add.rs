#![feature(extern_types)]

extern "C" {
    type Reference;
}

type Sender = *const Reference;

#[link(wasm_import_module = "system")]
extern "C" {
    fn send(recipient: Sender, address: *const u8, length: usize);
}

#[no_mangle]
pub unsafe fn receive(sender: Sender, address: *const u8, length: usize) {
    let pointer = address as *mut u32;
    let mut total: u32 = 0;

    for i in 0..length {
        total += *pointer.add(i);
    }

    let result = pointer.add(length);

    *result = total;

    send(sender, result as *const u8, 1);
}
