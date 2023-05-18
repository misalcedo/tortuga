use std::env;

fn main() {
    println!("Hello, world!");
    eprintln!("Hello, error!");

    for (index, arg) in env::args().enumerate() {
        eprintln!("{}) {}", index, arg);
    }
}
