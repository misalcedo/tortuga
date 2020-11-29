use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tortuga::System;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut system = System::new(1);

    let echo = system.register("echo", include_bytes!("../examples/echo.wat")).unwrap();

    system.distribute(echo, echo, b"Hello, World!");

    c.bench_function("echo", |b| b.iter(|| black_box(system.run_step())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);