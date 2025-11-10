use divan::Bencher;
use healthcheck_core::probes::http::HttpCheck;
use std::collections::HashMap;

fn main() {
    divan::main();
}

#[divan::bench]
fn http_config_parse(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert(
        "url".to_string(),
        "http://localhost:8080/health".to_string(),
    );
    params.insert("timeout_ms".to_string(), "5000".to_string());

    bencher.bench(|| HttpCheck::from_params(&params).unwrap());
}

#[divan::bench]
fn http_config_parse_https(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert("url".to_string(), "https://example.com/health".to_string());
    params.insert("timeout_ms".to_string(), "5000".to_string());

    bencher.bench(|| HttpCheck::from_params(&params).unwrap());
}
