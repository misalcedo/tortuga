use core::mem::{forget, size_of};
use std::error::Error;
use std::fs::read;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tortuga",
    about = "An actor-based system built on top of WASM technologies."
)]
struct Options {
    #[structopt(short, long, parse(from_os_str))]
    text_actors: Vec<PathBuf>,
    #[structopt(short, long, parse(from_os_str))]
    binary_actors: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Options::from_args();

    println!("{:?}", opt);

    let path = Path::new("./examples/target/wasm32-unknown-unknown/debug/");
    let system = tortuga::System::new();

    system.run(&read(path.join("echo.wasm"))?, "Hello, World!".as_bytes())?;

    let mut numbers = [42, 7, 1, 5];
    let ptr = numbers.as_mut_ptr() as *mut u8;
    let length = numbers.len() * size_of::<u32>();

    forget(numbers);

    let bytes = unsafe { Vec::from_raw_parts(ptr, length, length) };

    system.run(&read(path.join("add.wasm"))?, &bytes)?;

    Ok(())
}
