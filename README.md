# Tortuga

Tortuga is an HTTP CGI-specific server written in Rust.

## Badges
[![Build](https://github.com/misalcedo/tortuga/actions/workflows/compatibility.yml/badge.svg)](https://github.com/misalcedo/tortuga/actions/workflows/compatibility.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-yellowgreen.svg)](https://opensource.org/licenses/Apache-2.0)
[![Crates.io Version](https://img.shields.io/crates/v/tortuga.svg)](https://crates.io/crates/tortuga)
[![Docs.rs Version](https://docs.rs/tortuga/badge.svg)](https://docs.rs/tortuga)

## Usage
### Git HTTP Backend
An example of using the server for an HTTP git server.

#### Start the server
```bash
mkdir -p repos/test.git
pushd repos/test.git
git init --bare
git config http.getanyfile true
git config http.receivepack true
popd

tortuga serve $PWD -P 3000
```

#### Use remote

```bash
git init
git touch file.txt
git commit -m 'this is a test'
git remote add origin http://misalcedo@localhost:3000/cgi-bin/git.cgi/repos/test.git
git push --set-upstream origin main
```

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
The initial compilation slows down the first request.

```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar'
Running 1s test @ http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.19ms    4.79ms  44.02ms   96.79%
    Req/Sec     2.55k   462.45     3.01k    90.91%
  2791 requests in 1.10s, 272.56KB read
Requests/sec:   2535.83
Transfer/sec:    247.64KB
```

#### WCGI with Compilation Cache
The cache is enabled by pointing the server at a file created via `wasmtime config new`.

```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar'      
Running 1s test @ http://localhost:3000/cgi-bin/echo.wcgi/extra/path?--foo+bar
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   408.90us  266.19us   3.76ms   92.34%
    Req/Sec     2.59k   260.40     2.84k    81.82%
  2828 requests in 1.10s, 276.17KB read
Requests/sec:   2570.45
Transfer/sec:    251.02KB
```

#### CGI
```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/cgi-bin/echo.cgi/extra/path?--foo+bar'
Running 1s test @ http://localhost:3000/cgi-bin/echo.cgi/extra/path?--foo+bar
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     6.68ms    1.08ms   9.74ms   69.33%
    Req/Sec   150.10     13.25   170.00     80.00%
  150 requests in 1.00s, 14.65KB read
Requests/sec:    149.35
Transfer/sec:     14.58KB
```

#### Static File
```bash
$ wrk -c 1 -t 1 -d 1s 'http://localhost:3000/index.html'
Running 1s test @ http://localhost:3000/index.html
  1 threads and 1 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   221.81us  118.00us   1.45ms   90.75%
    Req/Sec     4.62k   497.28     5.10k    81.82%
  5058 requests in 1.10s, 503.82KB read
Requests/sec:   4598.99
Transfer/sec:    458.10KB
```
