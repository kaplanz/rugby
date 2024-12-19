//! Command-line interface.

use clap::Parser;
use rugby_cfg::opt::emu::Cart;

/// [Info](super::exec) options.
#[derive(Debug, Parser)]
#[clap(arg_required_else_help = true)]
#[group(id = "Info")]
pub struct Cli {
    /// Cartridge options.
    #[clap(flatten)]
    pub cart: Cart,
}
