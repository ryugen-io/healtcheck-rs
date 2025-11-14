//! CLI command implementations: `generate-bin` and `generate-conf`.
//!
//! Security: Path validation, TOCTOU prevention, TTY detection, credential warnings.

mod generate_bin;
mod generate_conf;

pub use generate_bin::execute as generate_bin;
pub use generate_conf::execute as generate_conf;
