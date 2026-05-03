//! Command-line interface.

use rugby::extra::cfg;

use super::NAME;
use crate::exe::run::cli;

/// Check header for ROM.
#[derive(Debug)]
#[derive(clap::Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[group(id = "chk::Cli")]
pub struct Cli {
    /// Check header only.
    #[arg(short = 'H', long = "header")]
    #[arg(help_heading = None)]
    pub head: bool,

    /// Output header format.
    #[arg(long = "format")]
    #[arg(visible_alias = "fmt")]
    #[arg(value_name = "FORMAT")]
    #[arg(help_heading = None)]
    pub fmt: Option<Format>,

    /// Command-line options.
    #[command(flatten)]
    pub cli: cli::Cart,

    /// Configurable options.
    #[command(flatten)]
    pub cfg: cfg::Cart,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[derive(clap::ValueEnum)]
#[non_exhaustive]
pub enum Format {
    /// Pretty, human readable.
    #[default]
    Pretty,
    /// JavaScript Object Notation (JSON).
    Json,
}
