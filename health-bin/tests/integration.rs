use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn healthcheck_exits_on_invalid_config() {
    let output = Command::new(env!("CARGO_BIN_EXE_healthcheck"))
        .arg("/nonexistent/path/config.conf")
        .output()
        .expect("failed to execute healthcheck");

    assert!(!output.status.success());
}

#[test]
fn healthcheck_requires_config_arg() {
    let output = Command::new(env!("CARGO_BIN_EXE_healthcheck"))
        .output()
        .expect("failed to execute healthcheck");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage"));
}

#[test]
fn healthcheck_parses_valid_config() {
    let mut config_path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    config_path.push("test_config.conf");

    let config_content = "tcp:host=127.0.0.1,port=22,timeout_ms=1000\n";
    fs::write(&config_path, config_content)
        .expect("failed to write config");

    let output = Command::new(env!("CARGO_BIN_EXE_healthcheck"))
        .arg(&config_path)
        .output()
        .expect("failed to execute healthcheck");

    fs::remove_file(&config_path).ok();

    assert!(output.status.success());
}
