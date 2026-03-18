//! Command-line interface.

use super::NAME;
pub use crate::exe::r#gen::cli::Command;

/// Display docs for a command.
#[derive(Debug)]
#[derive(clap::Parser)]
#[command(name = NAME)]
#[command(flatten_help = true)]
#[group(id = "Help")]
pub struct Cli {
    /// Rugby subcommand.
    #[arg(value_name = "COMMAND")]
    pub cmd: Option<Command>,
}
