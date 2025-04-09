//! Command-line interface.

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

use super::NAME;

/// Generate static files.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[command(flatten_help = true)]
#[group(id = "Gen")]
pub struct Cli {
    /// Document type.
    #[command(subcommand)]
    pub document: Document,
}

/// Generated document.
#[derive(Debug, Subcommand)]
#[command(disable_help_subcommand = true)]
#[non_exhaustive]
pub enum Document {
    /// Configuration file.
    #[command(disable_help_flag = true)]
    Cfg,
    /// Shell completions.
    #[command(arg_required_else_help = true)]
    #[command(disable_help_flag = true)]
    Cmp { shell: Shell },
    /// Manual pages.
    #[command(disable_help_flag = true)]
    Man {
        #[arg(value_name = "COMMAND")]
        cmd: Option<Command>,
    },
}

/// Execution mode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
pub enum Command {
    /// Analyze provided ROM.
    Check,
    /// Emulate provided ROM.
    Run,
    /// Generate static files.
    Gen,
    /// Show help information.
    Help,
}
