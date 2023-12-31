use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::StatusCode;
use std::path::Component::CurDir;
use std::path::PathBuf;
use std::sync::OnceLock;
use tortuga::{Options, Server};

static URI: OnceLock<String> = OnceLock::new();

pub fn criterion_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Unable to start an async runtime");
    let options = Options {
        wasm_cache: None,
        document_root: PathBuf::from("examples/"),
        cgi_bin: PathBuf::from(CurDir.as_os_str()),
        hostname: "localhost".to_string(),
        port: 0,
    };
    let server = runtime.block_on(Server::bind(options)).unwrap();
    let address = server.address().unwrap();

    URI.set(format!("http://{}/cgi-bin/assert.cgi", address)).unwrap();

    runtime.spawn(async move { server.serve().await });

    c.bench_function("uncached wcgi", |b| {
        let client = reqwest::blocking::Client::new();

        b.iter(move || {
            let body = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
            let response = client.post(URI.get().unwrap())
                .body(body)
                .header(reqwest::header::CONTENT_TYPE, "text/html")
                .header(reqwest::header::CONTENT_LENGTH, body.len())
                .send()
                .unwrap();

            assert_eq!(StatusCode::OK, response.status());
            assert_eq!(Bytes::from(body.as_bytes()), response.bytes().unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
