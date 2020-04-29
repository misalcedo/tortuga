use std::io::{self, Read};

mod parser;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).expect("Unable to read from stdin.");

    let actions = parser::parse(buffer.as_str()).expect("Unable to parse the file.");

    for (reference, message) in actions.iter() {
        match reference {
            &"add" => {
                let total: u64 = message.iter().sum();
                println!("{}", total);
            },
            &"subtract" => {
                let iterator = message.iter();

                match iterator.next() {
                    Some(value) => {
                        let total: u64 = iterator.sum();
                        println!("{}", value - total)
                    },
                    None => eprintln!("Must pass at least one value to subtract.")
                };
            },
            &"divide" => {
                let iterator = message.iter();

                match iterator.next() {
                    Some(value) => {
                        let total: u64 = iterator.product();
                        println!("{}", value / total)
                    },
                    None => eprintln!("Must pass at least one value to subtract.")
                };
            },
            &"multiply" => {
                let total: u64 = message.iter().product();
                println!("{}", total);
            }
        };
    }
}