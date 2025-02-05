//! Analyze provided ROM.

use anyhow::Context;
use constcat::concat;
use log::trace;

use crate::err::Result;
use crate::init;

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-check");

/// [`Check`](crate::cli::Command::Check) entrypoint.
#[expect(clippy::needless_pass_by_value)]
pub fn main(args: Cli) -> Result<()> {
    // Initialize logger
    crate::log::init(None).context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Load cartridge
    let cart = init::cart(&args.cart)?.context("try again with a valid ROM")?;
    // Print header
    println!("{}", cart.header());

    Ok(())
}
