# Installation

Tortuga can be used in various ways. The most common is to download the compiled binaries for your system from the [latest release](https://github.com/misalcedo/tortuga/releases) on the [repository](https://github.com/misalcedo/tortuga).

## Download
Each new version of Tortuga creates a [release](https://github.com/misalcedo/tortuga/releases) in the [repository](https://github.com/misalcedo/tortuga). A release contains compressed (`.tar.gz` and `.zip`) assets for Linux, macOS, and Windows.

After you download the asset for your system, use the tools for your operating system to decompress the assets into an executable. Then, place the executable in a directory that is part of your `PATH` environment variable.

## Cargo
Alternatively, you can install any version of tortuga from source using `cargo` with: 

```console
cargo install tortuga
```

## Embedded
To embed the language in a Rust program, add the `tortuga` crate as a dependency in your `Cargo.toml`:

```toml
tortuga = { version = "*", default-features = false }
```

## Docker
To use the language in a container, use the `ghcr.io/misalcedo/tortuga` image:

```console
docker run -it --rm ghcr.io/misalcedo/tortuga
```