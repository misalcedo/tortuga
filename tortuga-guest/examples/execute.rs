use tortuga_runtime::System;

fn main () {
    let mut system = System::new(9);
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

    system.distribute(echo, 0, b"Hello, World!");
    system.distribute(add, 0, b"Hello, World!");
    system.distribute(ping, pong, b"Hello, World!");

    system.run_step();
    system.run_step();
    system.run_step();
    system.run_step();
}