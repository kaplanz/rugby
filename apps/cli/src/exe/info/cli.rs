//! Command-line interface.

use clap::Args;
use rugby_cfg::opt::emu::Cart;

/// [Info](super::exec) options.
#[derive(Args, Debug)]
#[clap(arg_required_else_help = true)]
#[group(id = "Info")]
pub struct Cli {
    /// Cartridge options.
    #[clap(flatten)]
    pub cart: Cart,
}
