use divan::Bencher;
use healthcheck_core::probes::database::DatabaseCheck;
use std::collections::HashMap;

fn main() {
    divan::main();
}

#[divan::bench]
fn database_config_parse(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert(
        "conn_str".to_string(),
        "postgresql://user:pass@localhost:5432/db".to_string(),
    );
    params.insert("timeout_ms".to_string(), "3000".to_string());

    bencher.bench(|| DatabaseCheck::from_params(&params).unwrap());
}

#[divan::bench]
fn database_config_parse_complex(bencher: Bencher) {
    let mut params = HashMap::new();
    params.insert(
        "conn_str".to_string(),
        "postgresql://meta_user:complex_pass@db.example.com:5432/meta_db?sslmode=require"
            .to_string(),
    );
    params.insert("timeout_ms".to_string(), "5000".to_string());

    bencher.bench(|| DatabaseCheck::from_params(&params).unwrap());
}
