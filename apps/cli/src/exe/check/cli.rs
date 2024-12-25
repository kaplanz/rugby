//! Command-line interface.

use clap::Parser;
use rugby_cfg::opt::emu::Cart;

use super::NAME;

/// Analyze provided ROM.
#[derive(Debug, Parser)]
#[clap(name = NAME)]
#[clap(arg_required_else_help = true)]
#[group(id = "Check")]
pub struct Cli {
    /// Cartridge options.
    #[clap(flatten)]
    pub cart: Cart,
}
