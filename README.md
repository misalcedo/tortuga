# Tortuga

Tortuga is an HTTP CGI-specific server written in Rust.

## Badges
[![Build](https://github.com/misalcedo/tortuga/actions/workflows/compatibility.yml/badge.svg)](https://github.com/misalcedo/tortuga/actions/workflows/compatibility.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellowgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Version](https://img.shields.io/crates/v/tortuga.svg)](https://crates.io/crates/tortuga)
[![Docs.rs Version](https://docs.rs/tortuga/badge.svg)](https://docs.rs/tortuga)

## Book
For design goals, non-goals and more see the [Tortuga Web Server Book](https://tortuga.salcedo.cc).

## RFC
This project attempts to implement the [CGI RFC](https://www.rfc-editor.org/rfc/rfc3875.html).

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
Some basic CGI programs can be found in the [/examples](./examples) directory.

## Versioning
Tortuga adheres to [Semantic Versioning](https://semver.org/). You can use `tortuga version` or `tortuga -V` to determine the version of a Tortuga installation.

## Benchmark
All benchmarking code was run on a 16-core Codespace with the default image. Apache was installed using the `apt` package and `wrk` was installed by building from source.

### Apache
Configured Apache to serve the [debug.cgi](examples/debug.cgi) script using the default configuration to get a baseline for the performance we want to aim for without much load.

Script:
```bash
sudo cp examples/*.cgi /usr/lib/cgi-bin
wrk -t5 -c5 -d30s -R20 --latency 'http://localhost/cgi-bin/debug.cgi/extra/path?foo+bar+--me%202'
wrk -t5 -c5 -d30s -R340 --latency 'http://localhost/cgi-bin/empty.cgi/extra/path?foo+bar+--me%202'
```

Output:
```bash
Running 30s test @ http://localhost/cgi-bin/debug.cgi/extra/path?foo+bar+--me%202
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    78.12ms    2.29ms  97.98ms   81.50%
    Req/Sec     3.83      2.88     6.00    100.00%
  Latency Distribution (HdrHistogram - Recorded Latency)
 50.000%   77.63ms
 75.000%   78.97ms
 90.000%   80.51ms
 99.000%   86.14ms
 99.900%   98.05ms
 99.990%   98.05ms
 99.999%   98.05ms
100.000%   98.05ms

#[Mean    =       78.119, StdDeviation   =        2.294]
#[Max     =       97.984, Total count    =          400]
#[Buckets =           27, SubBuckets     =         2048]
----------------------------------------------------------
  600 requests in 30.01s, 682.72KB read
Requests/sec:     20.00
Transfer/sec:     22.75KB

Running 30s test @ http://localhost/cgi-bin/empty.cgi/extra/path?foo+bar+--me%202
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.89ms  632.16us   5.86ms   60.74%
    Req/Sec    71.72     48.95   111.00     68.41%
  Latency Distribution (HdrHistogram - Recorded Latency)
 50.000%    2.87ms
 75.000%    3.40ms
 90.000%    3.70ms
 99.000%    4.22ms
 99.900%    5.22ms
 99.990%    5.81ms
 99.999%    5.86ms
100.000%    5.86ms
#[Mean    =        2.886, StdDeviation   =        0.632]
#[Max     =        5.856, Total count    =         6795]
#[Buckets =           27, SubBuckets     =         2048]
----------------------------------------------------------
  10205 requests in 30.00s, 1.18MB read
Requests/sec:    340.13
Transfer/sec:     40.20KB
```

### Test without server
Running the [empty.cgi](examples/empty.cgi) script without an HTTP server takes about 5 milliseconds. Therefore, we will not be able to do much better than half that time (since the test command starts 2 processes). Apache takes approximately 3 milliseconds on an empty CGI script, so there is very little overhead.

```
cargo build --release
time target/release/tortuga test -s examples/debug.cgi
```

### CGI versus WCGI
On a 4-core Intel CPU, I ran the following quick benchmarks:

#### WCGI without Compilation Cache
```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar'
Running 1s test @ http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.63ms    2.13ms  34.56ms   92.86%
    Req/Sec    41.40      6.04    50.00     60.00%
  42 requests in 1.01s, 4.10KB read
Requests/sec:     41.60
Transfer/sec:      4.06KB
```

#### WCGI with Compilation Cache
The cache is enabled by pointing the server at a file created via `wasmtime config new`.

```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar'      
Running 1s test @ http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.66ms  669.17us  10.06ms   95.96%
    Req/Sec   380.36     34.42   414.00     72.73%
  417 requests in 1.10s, 40.72KB read
Requests/sec:    378.36
Transfer/sec:     36.95KB
```

#### CGI
```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/cgi-bin/echo.cgi/extra/path?--foo+bar'
Running 1s test @ http://localhost:3000/cgi-bin/echo.cgi/extra/path?--foo+bar
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.27ms  469.06us   7.84ms   85.21%
    Req/Sec   234.36      8.29   242.00     81.82%
  257 requests in 1.10s, 25.10KB read
Requests/sec:    233.14
Transfer/sec:     22.77KB
```
