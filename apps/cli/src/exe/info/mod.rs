//! Print ROM information.

use crate::err::Result;
use crate::init;

pub mod cli;

pub use self::cli::Cli;

/// [`Info`](crate::cli::Command::Info) entrypoint.
#[allow(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Load cartridge
    let cart = init::cart(&args.cart)?;
    // Print header
    println!("{}", cart.header());

    Ok(())
}
