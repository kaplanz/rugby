//! Analyze provided ROM.

use anyhow::Context;
use constcat::concat;
use log::trace;

use crate::app::{init, save};
use crate::err::Result;

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

    // Load cart ROM
    let mut cart = init::cart(&args.cart)?.context("try again with a valid ROM")?;
    // Load cart RAM
    save::ram::load(&args.cart, &mut cart).context("error flashing save RAM")?;

    // Print header
    println!("{}", cart.header());

    Ok(())
}
