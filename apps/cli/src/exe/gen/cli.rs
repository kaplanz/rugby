//! Command-line interface.

use clap::{Args, Subcommand};
use clap_complete::Shell;

/// [Gen](super::exec) options.
#[derive(Args, Debug)]
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
    Man,
}
