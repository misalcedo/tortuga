# Tortuga
## Abstract
Tortuga is a functionally-oriented concurrent programming language. The runtime is a Rust program to provide performance and memory safety; the language compiles to WebAssembly. Using WebAssembly allows developers to utilize their favorite programming language to write actors for the runtime. Targeting WebAssembly as the compilation architecture allows us to test the runtime itself without a dependency on the programming language, so the two can be developed independently.

## Badges
[![Build](https://github.com/misalcedo/tortuga/actions/workflows/build.yml/badge.svg)](https://github.com/misalcedo/tortuga/actions/workflows/build.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellowgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Version](https://img.shields.io/crates/v/tortuga.svg)](https://crates.io/crates/tortuga)
[![Docs.rs Version](https://docs.rs/tortuga/badge.svg)](https://docs.rs/tortuga)

# Design
For design goals, non-goals, grammar, and more see [docs/Design.md](https://github.com/misalcedo/tortuga/blob/main/docs/design.md).

# Usage
## Command-Line
To run the system locally, perform the following steps:

1. Run `cargo install tortuga`.
1. Run `tortuga` to start the interpreter.
1. Type in some code, such as `10 - 011.01#2 = 6.75`.

## Embedded
To embed the language in Rust, add `tortuga` as a dependency in your `Cargo.toml`:
```toml
tortuga = { version = "0.4", default-features = false }
```

## Docker
To test the language in a container, run the `ghcr.io/misalcedo/tortuga` image:
```bash
docker run -it --rm ghcr.io/misalcedo/tortuga
```

# Testing
## Local Install
To test the command-line interface, instal the crate locally from the root of the repository with:

```bash
cargo install tortuga --path ./
```

## Cargo Tests
To run the unit and documentation tests, use `cargo test`.

# Endianness
While the system sends all numbers in network byte order (i.e., big endian), WebAssembly uses little-endian for its numbers. Therefore, the system will handle mapping the integers between the types of endianness. See <https://tools.ietf.org/html/draft-newman-network-byte-order-01>

# Examples
More concrete examples are pending finalizing the grammar. However, some basic examples can be found in the [/examples](https://github.com/misalcedo/tortuga/tree/main/examples) directory.
