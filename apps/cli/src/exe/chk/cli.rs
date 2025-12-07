//! Command-line interface.

use clap::{Parser, ValueEnum};
use rugby::extra::cfg::opt::emu::Cart;

use super::NAME;

/// Check header for ROM.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[group(id = "Check")]
pub struct Cli {
    /// Check header only.
    #[arg(short = 'H', long = "header")]
    #[arg(help_heading = "Features")]
    pub head: bool,

    /// Output header format.
    #[arg(long = "format")]
    #[arg(visible_alias = "fmt")]
    #[arg(value_name = "FORMAT")]
    #[arg(help_heading = "Features")]
    pub fmt: Option<Format>,

    /// Cartridge options.
    #[command(flatten)]
    pub cart: Cart,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
pub enum Format {
    /// Pretty, human readable.
    #[default]
    Pretty,
    /// JavaScript Object Notation (JSON).
    Json,
}
