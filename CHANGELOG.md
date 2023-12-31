# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.6.1] - 2023-12-31
### Changed
- Added a lib.rs in order to benchmark the server with Criterion.
- Created an Options type for the server separate from the CLI options.

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
