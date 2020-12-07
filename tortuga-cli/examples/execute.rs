use std::sync::mpsc::channel;
use std::time::Duration;
use tortuga_runtime::System;

fn main() {
    let mut system = System::new(9);
    let (sender, receiver) = channel();
    let timeout = Duration::from_millis(10);

    let add = system
        .register(
            "add",
            include_bytes!(
                "../../tortuga-guest/target/wasm32-unknown-unknown/debug/examples/add.wasm"
            ),
        )
        .unwrap();
    let echo = system
        .register(
            "echo",
            include_bytes!(
                "../../tortuga-guest/target/wasm32-unknown-unknown/debug/examples/echo.wasm"
            ),
        )
        .unwrap();
    let ping = system
        .register(
            "ping",
            include_bytes!(
                "../../tortuga-guest/target/wasm32-unknown-unknown/debug/examples/ping.wasm"
            ),
        )
        .unwrap();
    let pong = system
        .register(
            "pong",
            include_bytes!(
                "../../tortuga-guest/target/wasm32-unknown-unknown/debug/examples/pong.wasm"
            ),
        )
        .unwrap();

    let external = system.register_external(sender);
    let message = b"Hello, World!";

    system.distribute(echo, external, message).unwrap();
    system
        .distribute(add, external, &to_le(vec![2, 20, 3, 7, 6, 4])[..])
        .unwrap();
    system.distribute(ping, external, message).unwrap();
    system.distribute(pong, external, message).unwrap();

    for i in 0..7 {
        println!("Running step {}...", i);
        assert!(system.run_step().unwrap());
    }

    assert_eq!(
        receiver.recv_timeout(timeout).unwrap().message().unwrap(),
        message
    );
    assert_eq!(
        receiver.recv_timeout(timeout).unwrap().message().unwrap(),
        42u32.to_le_bytes()[..].as_ref()
    );
    assert_eq!(
        receiver.recv_timeout(timeout).unwrap().message().unwrap(),
        b"Ping!\n"
    );
    assert_eq!(
        receiver.recv_timeout(timeout).unwrap().message().unwrap(),
        b"Pong!\n"
    );
}

fn to_le(v: Vec<u32>) -> Vec<u8> {
    let temp: Vec<[u8; 4]> = v.iter().map(|x| x.to_le_bytes()).collect();
    temp.iter().flatten().cloned().collect()
}
