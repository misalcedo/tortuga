# Tortuga

Tortuga is a functionally-oriented concurrent programming language. The runtime is a Rust program to provide performance and memory safety; the language compiles to WebAssembly. Using WebAssembly allows developers to utilize their favorite programming language to write actors for the runtime. Targeting WebAssembly as the compilation architecture allows us to test the runtime itself without a dependency on the programming language, so the two can be developed independently.

## Badges
[![Build](https://github.com/misalcedo/tortuga/actions/workflows/build.yml/badge.svg)](https://github.com/misalcedo/tortuga/actions/workflows/build.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellowgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Version](https://img.shields.io/crates/v/tortuga.svg)](https://crates.io/crates/tortuga)
[![Docs.rs Version](https://docs.rs/tortuga/badge.svg)](https://docs.rs/tortuga)

## Book
For design goals, non-goals, grammar, and more see the [Tortuga Programming Language Book](https://tortuga.salcedo.cc).


## Testing
### Local Install
To test the command-line interface, instal the crate locally from the root of the repository with:

```bash
cargo install tortuga --path ./
```

### Cargo Tests
To run the unit and documentation tests, use `cargo test`.

## Endianness
While the system sends all numbers in network byte order (i.e., big endian), WebAssembly uses little-endian for its numbers. Therefore, the system will handle mapping the integers between the types of endianness. See <https://tools.ietf.org/html/draft-newman-network-byte-order-01>

## Examples
More concrete examples are pending finalizing the grammar. However, some basic examples can be found in the [/examples](https://github.com/misalcedo/tortuga/tree/main/examples) directory.

## Versioning
Tortuga adheres to [Semantic Versioning](https://semver.org/). You can use `tortuga version` or `tortuga -V` to determine the version of a Tortuga installation.
