//! A benchmark of the tree-walking interpreter for Tortuga.
//! The benchmark generates Fibonnaci numbers.

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};

#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

use std::any::type_name;
use tortuga::Interpreter;

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("PPP Binary");

    let inputs: Vec<String> = (0..10).into_iter().map(|v| format!(r###"
        @fibonacci(@n <= 1) = n
        @fibonacci(@n) = fibonacci(n - 2) + fibonacci(n - 1)
        
        fibonacci({})
    "###, v)).collect();

    for input in inputs {
        group.bench_with_input(
            BenchmarkId::new(type_name::<Interpreter>(), input.as_str()),
            input.as_str(),
            |b, i| {
                b.iter(|| Interpreter::build_then_run(i));
            },
        );
    }

    group.finish();
}

#[cfg(unix)]
criterion_group! {
    name = benches;
    config = {
        Criterion::default().with_profiler(PProfProfiler::new(100, Output::Protobuf))
    };
    targets = benchmarks
}

#[cfg(not(unix))]
criterion_group!(benches, benchmarks);

criterion_main!(benches);