//! Command-line interface.

use clap::Parser;

use super::NAME;
pub use crate::exe::r#gen::cli::Command;

/// Show help information.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(flatten_help = true)]
#[group(id = "Help")]
pub struct Cli {
    /// Rugby subcommand.
    #[arg(value_name = "COMMAND")]
    pub cmd: Option<Command>,
}
