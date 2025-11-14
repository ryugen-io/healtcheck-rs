//! Configuration file generation command

use std::fs;
use std::io::{self, IsTerminal, Write};

use crate::path_validation::validate_output_path;

/// Template config embedded at compile time
const EXAMPLE_CONFIG: &str = include_str!("../../healthcheck.config.template");

pub fn execute(output_path: Option<String>) -> Result<(), String> {
    let config_path = output_path.as_deref().unwrap_or("healthcheck.config");
    let validated_path = validate_output_path(config_path)?;

    // Atomic file creation to prevent TOCTOU vulnerabilities
    let mut file = if io::stdin().is_terminal() {
        // Interactive mode: check if file exists and prompt user
        if validated_path.exists() {
            eprintln!(
                "Warning: File '{}' already exists and will be overwritten.",
                config_path
            );
            eprintln!("Press Ctrl+C to cancel, or Enter to continue...");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;

            // User confirmed, overwrite the file
            fs::File::create(&validated_path)
                .map_err(|e| format!("Failed to create config file '{}': {}", config_path, e))?
        } else {
            // File doesn't exist, create it atomically
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&validated_path)
                .map_err(|e| format!("Failed to create config file '{}': {}", config_path, e))?
        }
    } else {
        // Non-interactive mode: use create_new() to fail atomically if file exists
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&validated_path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    format!(
                        "File '{}' already exists. Remove it first or run in an interactive terminal to confirm overwrite.",
                        config_path
                    )
                } else {
                    format!("Failed to create config file '{}': {}", config_path, e)
                }
            })?
    };

    file.write_all(EXAMPLE_CONFIG.as_bytes())
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    println!("Generated example configuration:");
    println!("  File: {}", config_path);
    println!();
    println!("Edit the file and run:");
    println!("  healthcheck {}", config_path);

    Ok(())
}
