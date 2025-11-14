mod common;

use std::process::Command;

use common::get_healthcheck_bin;

#[test]
fn test_help_flag() {
    let output = Command::new(get_healthcheck_bin())
        .arg("--help")
        .output()
        .expect("failed to execute healthcheck --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("generate-bin"));
    assert!(stdout.contains("generate-conf"));
}

#[test]
fn test_version_flag() {
    let output = Command::new(get_healthcheck_bin())
        .arg("--version")
        .output()
        .expect("failed to execute healthcheck --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("healthcheck v"));
}

#[test]
fn test_unknown_flag_error() {
    let output = Command::new(get_healthcheck_bin())
        .arg("--unknown-flag")
        .output()
        .expect("failed to execute healthcheck --unknown-flag");

    assert!(!output.status.success(), "Should fail with unknown flag");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown flag"));
}

#[test]
fn test_serve_command_unimplemented() {
    let output = Command::new(get_healthcheck_bin())
        .arg("serve")
        .output()
        .expect("failed to execute healthcheck serve");

    assert!(!output.status.success(), "Should exit with error code");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not yet implemented") || stderr.contains("Coming soon"),
        "serve command should indicate it's not implemented yet. Got: {}",
        stderr
    );
}

#[test]
fn test_watch_command_unimplemented() {
    let output = Command::new(get_healthcheck_bin())
        .arg("watch")
        .output()
        .expect("failed to execute healthcheck watch");

    assert!(!output.status.success(), "Should exit with error code");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not yet implemented") || stderr.contains("Coming soon"),
        "watch command should indicate it's not implemented yet. Got: {}",
        stderr
    );
}
