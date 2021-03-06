# Tortuga

---
**NOTE:**
The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and
"OPTIONAL" in this document are to be interpreted as described in
[RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).
---

## Abstract
Tortuga is an actor-based programming language and runtime. The runtime is written in Rust to provide performance and memory safety, while the language is compiled to WebAssembly. Using WebAssembly allows developers to utilize their favorite programming language to write actors for the runtime. Targeting WebAssembly as the compilation architecture allows us to test the runtime itself without a dependency on the programming language, so the two can be developed independently.

## Badges

[![License](https://img.shields.io/badge/License-Apache%202.0-yellowgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Version](https://img.shields.io/crates/v/tortuga.svg)](https://crates.io/crates/tortuga)
[![Docs.rs Version](https://docs.rs/tortuga/badge.svg)](https://docs.rs/tortuga)

# Glossary

## Actors

> An actor is a computational entity that, in response to a message it receives, can concurrently:
>
> 1.  Send a finite number of messages to other actors.
>
> 2.  Create a finite number of new actors.
>
> 3.  Designate the behavior to be used for the next message it receives.
>
> — 
> Wikipedia
> https://en.wikipedia.org/wiki/Actor_model

## Intent
The intent is the set of behaviors that an actor may execute in response to receiving a message. An intent must have at least one behavior that is the default behavior.

## Behavior
A behavior is a set of logical instructions (i.e. messages sent to actors) executed in the context of an actor’s continuation for a specific message.

## Continuation
An instantiation of an actor’s intent in the actor system to process a message. Continuations may be re-used between multiple messages as long as the re-use is not perceivable (i.e. does not affect the outcome of the intent).

## Supervision
Actors in an actor system may form a supervision tree. The tree is always rooted in a system-provided root actor. The root actor has children for system actors and user actors. When an actor creates a new actor, the creator becomes the supervisor of the created. On failure to process a message, the supervisor is queried to determine the appropriate action (e.g., create a new continuation and re-process the message, discard the message, etc.).

## Host
The WebAssembly runtime that instantiates guests for each actor continuation and routes messages.

## Guest
The guest is an instance of a WebAssembly module with access to the Application Programming Language (API) of the Tortuga runtime. A guest maps to a continuation of an actor.

# Design

## Compiler

-   Lexer
-   Parser
-   Transformer
-   Emitter

## WebAssembly (WASM)

Small wrapper around the WASM library.

## Runtime

Defines the interface between the guest and host. Relied upon by the system to run instances.

# Usage

To run the system locally, perform the following steps:

1.  Clone the repository to your local machine.

2.  Run `cargo build`.

3.  Build the examples as described in the [Examples](#Examples) section.

4.  <span id="System"></span> Run `cargo run start`.

5.  <span id="ping"></span> Run `cargo run -- deploy --intent examples/target/wasm32-unknown-unknown/release/ping.wasm`

6.  <span id="pong"></span> Run `cargo run -- deploy --intent examples/target/wasm32-unknown-unknown/release/pong.wasm`

7.  In the terminal that is running the [System](#System), you should see the "Ping!" and "Pong!" messages flowing back and forth.

# Limitations

# System

The initial versions of the actor system will have the following limitations:

1.  Only a single thread will process incoming messages, create continuations, and send outgoing messages.

2.  No supervision tree, any failed messages will be discarded.

3.  Actors can only send messages, they cannot create new actors or change the behavior.

4.  New continuation for every message.

# Endianness

While the system sends all numbers in network byte order (i.e. big endian), WebAssembly uses little-endian for its numbers. Therefore, the system will handle mapping the integers between the types of endianness. See <https://tools.ietf.org/html/draft-newman-network-byte-order-01>

# Examples

# Rust

Some examples are Rust-based Tortuga actors that compile to WASM. To build the examples, change to the `examples` workspace directory. Then, run `cargo build --release`. Built examples can be found in: `examples/target/wasm32-unknown-unknown/release/*.wasm`.
