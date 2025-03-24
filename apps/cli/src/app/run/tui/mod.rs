//! Terminal thread.

use anyhow::Result;

use crate::app;
use crate::exe::run::Cli;

/// Terminal main.
#[expect(unused)]
#[expect(clippy::unnecessary_wraps)]
pub fn main(args: &Cli) -> Result<()> {
    // Terminal loop
    while app::running() {}

    Ok(())
}
