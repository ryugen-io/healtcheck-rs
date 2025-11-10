use healthcheck_core::probes::process::ProcessCheck;
use std::collections::HashMap;

#[test]
fn process_check_from_params_valid() {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "systemd".to_string());

    let check = ProcessCheck::from_params(&params);
    assert!(check.is_ok());
}

#[test]
fn process_check_from_params_missing_name() {
    let params = HashMap::new();
    let check = ProcessCheck::from_params(&params);
    assert!(check.is_err());
}

#[test]
fn process_check_name() {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "systemd".to_string());

    let check = ProcessCheck::from_params(&params).unwrap();
    assert_eq!(check.name(), "process");
}

#[test]
fn process_check_systemd_running() {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "systemd".to_string());

    let check = ProcessCheck::from_params(&params).unwrap();
    let result = check.check();
    assert!(result.ok);
}

#[test]
fn process_check_nonexistent_fails() {
    let mut params = HashMap::new();
    params.insert(
        "name".to_string(),
        "definitely_not_running_process".to_string(),
    );

    let check = ProcessCheck::from_params(&params).unwrap();
    let result = check.check();
    assert!(!result.ok);
}
