#![no_main]

#[no_mangle]
pub fn add_one(value: i32) -> i32 {
    value + 1
}