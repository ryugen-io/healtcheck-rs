use healthcheck_core::probes::tcp::TcpCheck;
use healthcheck_core::registry::CheckRegistry;
use std::collections::HashMap;

#[test]
fn registry_register_and_create() {
    let mut registry = CheckRegistry::new();
    registry.register("tcp", TcpCheck::from_params);

    let mut params = HashMap::new();
    params.insert("host".to_string(), "127.0.0.1".to_string());
    params.insert("port".to_string(), "22".to_string());

    let result = registry.create_check("tcp", &params);
    assert!(result.is_ok());
}

#[test]
fn registry_unknown_check_type() {
    let registry = CheckRegistry::new();

    let params = HashMap::new();
    let result = registry.create_check("unknown", &params);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.contains("unknown check type"));
    }
}

#[test]
fn registry_multiple_types() {
    let mut registry = CheckRegistry::new();
    registry.register("tcp", TcpCheck::from_params);

    assert!(registry.create_check("tcp", &HashMap::new()).is_err());
}
