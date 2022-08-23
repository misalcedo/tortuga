# Tortuga

Tortuga is a functionally-oriented concurrent programming language. The runtime is a Rust program to provide performance and memory safety; the language compiles to WebAssembly. Using WebAssembly allows developers to utilize their favorite programming language to write actors for the runtime. Targeting WebAssembly as the compilation architecture allows us to test the runtime itself without a dependency on the programming language, so the two can be developed independently.

## Badges
[![Build](https://github.com/misalcedo/tortuga/actions/workflows/compatibility.yml/badge.svg)](https://github.com/misalcedo/tortuga/actions/workflows/compatibility.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellowgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Version](https://img.shields.io/crates/v/tortuga.svg)](https://crates.io/crates/tortuga)
[![Docs.rs Version](https://docs.rs/tortuga/badge.svg)](https://docs.rs/tortuga)

## Book
For design goals, non-goals, grammar, and more see the [Tortuga Programming Language Book](https://tortuga.salcedo.cc).


## Testing
### Local Install
To test the command-line interface, install the crate locally from the root of the repository with:

```console
cargo install --path ./
```

### Cargo Tests
To run the unit and documentation tests, use:
```console
cargo test
```

## Endianness
While the system sends all numbers in network byte order (i.e., big endian), WebAssembly uses little-endian for its numbers. Therefore, the system will handle mapping the integers between the types of endianness. See <https://tools.ietf.org/html/draft-newman-network-byte-order-01>

## Examples
More concrete examples are pending finalizing the grammar. However, some basic examples can be found in the [/examples](https://github.com/misalcedo/tortuga/tree/main/examples) directory.

## Versioning
Tortuga adheres to [Semantic Versioning](https://semver.org/). You can use `tortuga version` or `tortuga -V` to determine the version of a Tortuga installation.

## Benchmarks
Run using `cargo bench`.

```bash
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(0)                                                                             
                        time:   [10.927 us 10.961 us 10.998 us]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(1)                                                                             
                        time:   [10.898 us 10.990 us 11.090 us]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  6 (6.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(2)                                                                             
                        time:   [22.979 us 23.090 us 23.188 us]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  7 (7.00%) low mild
  3 (3.00%) high mild
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(3)                                                                             
                        time:   [35.145 us 35.282 us 35.400 us]
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) low severe
  2 (2.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(4)                                                                            
                        time:   [58.662 us 58.937 us 59.185 us]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
  3 (3.00%) high mild
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(5)                                                                            
                        time:   [94.610 us 95.028 us 95.369 us]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(6)                                                                            
                        time:   [155.05 us 155.41 us 155.74 us]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(7)                                                                            
                        time:   [249.85 us 250.68 us 251.48 us]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(8)                                                                            
                        time:   [405.06 us 406.98 us 408.57 us]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) low severe
  9 (9.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
Tortuga Fibonnaci/tortuga::runtime::interpret::Interpreter/fibonacci(9)                                                                            
                        time:   [659.07 us 661.90 us 665.23 us]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low severe
  5 (5.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe
```