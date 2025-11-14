//! HealthCheck RS - Lightweight health check CLI and API server
//!
//! A minimal, secure health checking tool with multiple probe types and
//! deployment-focused CLI commands.
//!
//! ## Features
//!
//! - **Multiple probe types**: TCP, HTTP, database (PostgreSQL), process
//! - **Deployment tools**: `generate-bin` and `generate-conf` for easy setup
//! - **Security-first**: Path validation, TOCTOU prevention, input sanitization
//! - **Minimal dependencies**: Only `log` and `env_logger` for runtime
//! - **Small binary**: ~530KB compressed with UPX
//! - **Fast**: Parallel check execution with minimal overhead
//!
//! ## Usage
//!
//! ```sh
//! # Run health checks from config
//! healthcheck healthcheck.config
//!
//! # Generate deployment binary
//! healthcheck generate-bin --output ./bin
//!
//! # Generate example config
//! healthcheck generate-conf
//! ```

mod cli;
mod commands;
mod path_validation;
mod runner;
mod status;

use cli::{CliAction, parse_args, print_help, print_version};

fn main() {
    match parse_args() {
        CliAction::Help => print_help(),
        CliAction::Version => print_version(),
        CliAction::GenerateBin { output_dir } => match commands::generate_bin(output_dir) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        CliAction::GenerateConf { output_path } => match commands::generate_conf(output_path) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        CliAction::Serve => {
            eprintln!("Error: 'serve' command not yet implemented");
            eprintln!("Coming soon: HTTP API server mode");
            std::process::exit(1);
        }
        CliAction::Watch => {
            eprintln!("Error: 'watch' command not yet implemented");
            eprintln!("Coming soon: Continuous monitoring mode");
            std::process::exit(1);
        }
        CliAction::RunChecks { config_path } => {
            runner::run_health_checks(&config_path);
        }
    }
}
