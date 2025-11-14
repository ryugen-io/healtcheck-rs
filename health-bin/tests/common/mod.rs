use std::env;
use std::path::PathBuf;

pub fn get_healthcheck_bin() -> PathBuf {
    env::var("CARGO_BIN_EXE_healthcheck")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.pop();
            path.push("target");
            path.push("debug");
            path.push("healthcheck");
            path
        })
}
