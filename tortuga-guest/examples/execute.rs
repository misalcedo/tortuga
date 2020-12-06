use tortuga_runtime::System;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main () {
    let mut system = System::new(9);
    let (sender, receiver) = channel();
    let timeout = Duration::from_millis(10);

    let add = system
        .register("add", include_bytes!("./target/wasm32-unknown-unknown/debug/examples/add.wasm"))
        .unwrap();
    let echo = system
        .register("echo", include_bytes!("./target/wasm32-unknown-unknown/debug/examples/echo.wasm"))
        .unwrap();
    let ping = system
        .register("ping", include_bytes!("./target/wasm32-unknown-unknown/debug/examples/ping.wasm"))
        .unwrap();
    let pong = system
        .register("pong", include_bytes!("./target/wasm32-unknown-unknown/debug/examples/pong.wasm"))
        .unwrap();

    let external = system.register_external(sender);

    system.distribute(echo, external, b"Hello, World!");
    system.distribute(add, external, b"Hello, World!");
    system.distribute(ping, pong, b"Hello, World!");

    // Run 2 steps for each message distributed.
    for i in 0..2 {
        system.run_step();
        system.run_step();
    }
}