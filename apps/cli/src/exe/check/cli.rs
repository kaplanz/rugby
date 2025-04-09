//! Command-line interface.

use clap::Parser;
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
}
