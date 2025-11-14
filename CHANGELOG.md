# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New CLI commands: `generate-bin` and `generate-conf` for deployment workflows
- TTY detection for non-interactive environments (CI/CD compatibility)
- Comprehensive path validation with system directory protection
- Support for extensionless files (Makefile, Dockerfile, healthcheck.config)
- 21 unit tests + 13 integration tests (34 total)
- cargo-auditable integration for supply chain security

### Changed
- **BREAKING**: Binary renamed from `healthcheckrs` to `healthcheck`
  - Migration: Update your Docker COPY/deployment scripts to use new binary name
  - Update systemd service files, cron jobs, and CI/CD pipelines
- Reorganized modules to idiomatic Rust structure (src/<module>/mod.rs)
- Improved JSON escaping for RFC 8259 compliance (solidus, control chars)
- Enhanced path canonicalization to prevent TOCTOU vulnerabilities
- Expanded system directory protection (/bin, /sbin, /lib, /usr/*, /root, /var/run, /var/lock)

### Fixed
- TOCTOU vulnerability in file creation using atomic OpenOptions::create_new()
- CLI index bug in --output flag parsing (off-by-2 error)
- Unsafe env::set_var replaced with safe env_logger configuration
- Inverted clippy logic in rebuild.sh script

### Security
- Path traversal attack prevention with canonicalization
- JSON injection protection with proper character escaping
- Credential warnings in generated configuration files
- File overwrite protection with user confirmation in interactive mode

## [1.0.0] - 2025-11-10

### Added
- Initial release of modular healthcheck system
- TCP port connectivity check with configurable timeout
- HTTP/HTTPS endpoint check with configurable timeout
- PostgreSQL database connection check with authentication
- Process existence check for Linux systems
- Config-based system using simple key=value format
- Custom parsers with minimal dependencies (no serde/TOML)
- Size-optimized binary (~517KB compressed with UPX)
- Cross-compilation support for ARM64 (aarch64-unknown-linux-musl)
- Comprehensive test suite (21 tests)
- Performance benchmarks using divan (4 benchmark suites)
- Code quality enforcement:
  - Rust Edition 2024
  - Min Rust version 1.91
  - Max 150 LOC per file
  - Clippy with zero warnings
  - MIT OR Apache-2.0 dual licensing

[Unreleased]: https://github.com/ryugen-io/healthcheck-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/ryugen-io/healthcheck-rs/releases/tag/v1.0.0
