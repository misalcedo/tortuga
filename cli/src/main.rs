use std::fs::read;
use std::path::Path;
use std::error::Error;
use core::mem::{forget, size_of};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./examples/target/wasm32-unknown-unknown/debug/");

    tortuga::run(&read(path.join("echo.wasm"))?, "Hello, World!".as_bytes())?;
    
    let mut numbers = [42, 7, 1, 5];
    let ptr = numbers.as_mut_ptr() as *mut u8;
    let length = numbers.len() * size_of::<u32>();
    
    forget(numbers);
    
    let bytes = unsafe { Vec::from_raw_parts(ptr, length, length) };

    tortuga::run(&read(path.join("add.wasm"))?, &bytes)?;

    Ok(())
}