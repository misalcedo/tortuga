# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.7.2] - 2024-01-08
### Added
- Support for extracting a `REMOTE_USER` and `AUTH_TYPE` from the `Authorization` header when using the `Basic` authentication scheme.
 
## [0.7.1] - 2024-01-02
### Fixed
- Bug where preload would not be respected when turned off during periodic scans.

## [0.7.0] - 2024-01-02
### Removed
- The server no longer relies on wasmtime's cache config.

### Added
- The server periodically scans the CGI bin directory to load WASM modules into memory and purge old cached entries no longer on the file system.

### Fixed
- Bug where wcgi scripts would not return a 404 correctly for script not found.
  
## [0.6.3] - 2024-01-02
### Changed
- The server now has a single instance of the Process and WASM invokers.

### Added
- Update the WASM invoker to cache compiled modules to speed up runtimes. The file-based WASM cache is now mostly useful to help with cold starts.

## [0.6.2] - 2023-12-31
### Changed
- Update the WASM cache config path documentation to refer to it as a file path.

## [0.6.1] - 2023-12-31
### Changed
- Added a lib.rs in order to benchmark the server with Criterion.
- Created an Options type for the server separate from the CLI options.

### Added
- A benchmark for uncached WCGI to the assert script.

### Removed
- The "full" features from dependencies in order to speed up compile times.

## [0.6.0] - 2023-12-30

### Removed
- All of the old Virtual Machine code.
- All of the language implementation.
- The custom in-memory WASI-compatible network to connect actors.

### Added
- A Hyper-based HTTP/1.1 server.
- A process-based CGI implementation.
- A WASI-based CGI implementation.
- A custom-percent-encoded String decoder.

## Before 0.6.0
Tortuga was a WebAssembly Virtual Machine for running a custom programming language. 
