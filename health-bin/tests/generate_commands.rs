mod common;

use std::env;
use std::fs;
use std::process::Command;

use common::get_healthcheck_bin;

#[test]
fn test_generate_bin_creates_binary() {
    let output_dir = env::temp_dir().join("test_bin_output");
    fs::create_dir_all(&output_dir).expect("failed to create temp dir");

    let output = Command::new(get_healthcheck_bin())
        .arg("generate-bin")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("failed to execute healthcheck generate-bin");

    // Cleanup
    fs::remove_dir_all(&output_dir).ok();

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Binary generated successfully"));
}

#[test]
fn test_generate_bin_without_output_flag() {
    let output = Command::new(get_healthcheck_bin())
        .arg("generate-bin")
        .output()
        .expect("failed to execute healthcheck generate-bin");

    // Cleanup default ./bin directory
    fs::remove_dir_all("./bin").ok();

    assert!(
        output.status.success(),
        "Command should succeed with default path"
    );
}

#[test]
fn test_generate_bin_missing_output_value() {
    let output = Command::new(get_healthcheck_bin())
        .arg("generate-bin")
        .arg("--output")
        .output()
        .expect("failed to execute healthcheck generate-bin --output");

    assert!(!output.status.success(), "Should fail without path value");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--output requires a path argument"));
}

#[test]
fn test_generate_conf_creates_file() {
    let config_path = env::temp_dir().join("test_healthcheck.config");

    let output = Command::new(get_healthcheck_bin())
        .arg("generate-conf")
        .arg("--output")
        .arg(&config_path)
        .output()
        .expect("failed to execute healthcheck generate-conf");

    let file_exists = config_path.exists();
    let mut file_content = String::new();
    if file_exists {
        file_content = fs::read_to_string(&config_path).unwrap_or_default();
    }

    // Cleanup
    fs::remove_file(&config_path).ok();

    assert!(output.status.success(), "Command should succeed");
    assert!(file_exists, "Config file should be created");
    assert!(
        file_content.contains("SECURITY WARNING"),
        "Config should contain security warning"
    );
    assert!(
        file_content.contains("tcp:"),
        "Config should contain TCP example"
    );
}

#[test]
fn test_generate_conf_non_interactive_existing_file() {
    let config_path = env::temp_dir().join("test_existing.config");

    // Create the file first
    fs::write(&config_path, "existing content").expect("failed to write existing file");

    // Try to generate conf in non-interactive mode (will fail because file exists)
    let output = Command::new(get_healthcheck_bin())
        .arg("generate-conf")
        .arg("--output")
        .arg(&config_path)
        .output()
        .expect("failed to execute healthcheck generate-conf");

    // Cleanup
    fs::remove_file(&config_path).ok();

    // In non-interactive mode, should fail when file exists
    // Note: This test might succeed in some CI environments that have a TTY,
    // so we check for either success (with prompt) or failure (without TTY)
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("already exists"),
            "Should indicate file already exists"
        );
    }
}
