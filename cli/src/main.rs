use std::fs::read;
use std::path::Path;

fn main() {
    let filename = Path::new("./examples/target/wasm32-unknown-unknown/debug/echo.wasm");
    let contents = read(filename).expect("buffer overflow");

    tortuga::run(&contents).unwrap()
}