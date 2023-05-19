use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::os::fd::{FromRawFd, RawFd};

#[link(wasm_import_module = "guest")]
extern "C" {
    pub fn connect(buffer: *const u8, length: u64) -> u32;
}

fn main() {
    println!("Hello, world!");
    eprintln!("Hello, error!");

    for (index, arg) in env::args().enumerate() {
        eprintln!("{}) {}", index, arg);
    }

    let origin = "http://localhost:9780";

    unsafe {
        let fd = connect(origin.as_ptr(), origin.len() as u64);

        eprintln!("{:?}", fd);

        let mut pipe = TcpStream::from_raw_fd(fd as RawFd);
        let mut message = String::new();

        pipe.read_to_string(&mut message).unwrap();

        eprintln!("{}", message);

        pipe.write("!".as_bytes()).unwrap();
    }
}
