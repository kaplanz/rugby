//! Command-line interface.

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

use super::NAME;

/// Generate static files.
#[derive(Debug, Parser)]
#[clap(name = NAME)]
#[clap(arg_required_else_help = true)]
#[clap(flatten_help = true)]
#[group(id = "Gen")]
pub struct Cli {
    /// Document type.
    #[clap(subcommand)]
    pub document: Document,
}

/// Generated document.
#[derive(Debug, Subcommand)]
#[clap(disable_help_subcommand = true)]
#[non_exhaustive]
pub enum Document {
    /// Configuration file.
    #[clap(disable_help_flag = true)]
    Cfg,
    /// Shell completions.
    #[clap(arg_required_else_help = true)]
    #[clap(disable_help_flag = true)]
    Cmp { shell: Shell },
    /// Manual pages.
    #[clap(disable_help_flag = true)]
    Man {
        #[clap(value_name = "COMMAND")]
        cmd: Option<Command>,
    },
}

/// Execution mode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
pub enum Command {
    /// Emulate provided ROM.
    Run,
    /// Print ROM information.
    Info,
    /// Generate static files.
    Gen,
}
