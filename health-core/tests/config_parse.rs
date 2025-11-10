use healthcheck_core::config::parse_config_str;

#[test]
fn parse_config_str_valid_tcp() {
    let config = "tcp:host=127.0.0.1,port=22,timeout_ms=1000";
    let result = parse_config_str(config);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].check_type, "tcp");
}

#[test]
fn parse_config_str_multiple_checks() {
    let config = "\
tcp:host=127.0.0.1,port=22
http:url=http://localhost:8080,timeout_ms=5000
";
    let result = parse_config_str(config);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 2);
}

#[test]
fn parse_config_str_with_comments() {
    let config = "\
# This is a comment
tcp:host=127.0.0.1,port=22
# Another comment
http:url=http://localhost:8080
";
    let result = parse_config_str(config);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 2);
}

#[test]
fn parse_config_str_empty_lines() {
    let config = "\
tcp:host=127.0.0.1,port=22

http:url=http://localhost:8080

";
    let result = parse_config_str(config);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 2);
}

#[test]
fn parse_config_str_invalid_format() {
    let config = "invalid format no colon";
    let result = parse_config_str(config);
    assert!(result.is_err());
}

#[test]
fn parse_config_str_database() {
    let config = "database:conn_str=postgresql://user:pass@localhost:5432/db,timeout_ms=3000";
    let result = parse_config_str(config);
    assert!(result.is_ok());
    let configs = result.unwrap();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs[0].check_type, "database");
}
