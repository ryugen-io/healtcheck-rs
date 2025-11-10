use healthcheck_core::probes::tcp::TcpCheck;
use std::collections::HashMap;

#[test]
fn tcp_check_from_params_valid() {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "22".to_string());
    params.insert("timeout_ms".to_string(), "1000".to_string());

    let check = TcpCheck::from_params(&params);
    assert!(check.is_ok());
}

#[test]
fn tcp_check_from_params_missing_host_uses_default() {
    let mut params = HashMap::new();
    params.insert("port".to_string(), "22".to_string());

    let check = TcpCheck::from_params(&params);
    assert!(check.is_ok());
}

#[test]
fn tcp_check_from_params_missing_port() {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());

    let check = TcpCheck::from_params(&params);
    assert!(check.is_err());
}

#[test]
fn tcp_check_name() {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "22".to_string());

    let check = TcpCheck::from_params(&params).unwrap();
    assert_eq!(check.name(), "tcp");
}

#[test]
fn tcp_check_localhost_ssh_succeeds() {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "22".to_string());
    params.insert("timeout_ms".to_string(), "1000".to_string());

    let check = TcpCheck::from_params(&params).unwrap();
    let result = check.check();
    assert!(result.ok);
}

#[test]
fn tcp_check_invalid_port_fails() {
    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "1".to_string());
    params.insert("timeout_ms".to_string(), "100".to_string());

    let check = TcpCheck::from_params(&params).unwrap();
    let result = check.check();
    assert!(!result.ok);
}
