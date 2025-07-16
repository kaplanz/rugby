//! Command-line interface.

use clap::{Parser, ValueEnum};
use rugby::extra::cfg::opt::emu::Cart;

use super::NAME;

/// Analyze provided ROM.
#[derive(Debug, Parser)]
#[command(name = NAME)]
#[command(arg_required_else_help = true)]
#[group(id = "Check")]
pub struct Cli {
    /// Cartridge options.
    #[command(flatten)]
    pub cart: Cart,

    /// Check header only.
    #[arg(short = 'H', long = "header")]
    pub head: bool,

    /// Output header format.
    #[arg(long = "format")]
    #[arg(visible_alias = "fmt")]
    #[arg(value_name = "FORMAT")]
    pub fmt: Option<Format>,
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
