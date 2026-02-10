//! Emulate provided ROM.

use anyhow::Context as _;
use constcat::concat;
use log::trace;
use merge::Merge;

use crate::err::Result;
use crate::{app, cfg};

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-run");

/// [`Run`](crate::cli::Command::Run) entrypoint.
pub fn main(mut args: Cli) -> Result<()> {
    // Load configuration
    args.cfg.data.merge({
        // Parse config from file
        cfg::load(&args.cfg.path)?
    });
    // Initialize logger
    crate::log::init(args.cfg.data.app.log.as_deref()).context("logger initialization failed")?;
    // Log arguments
    trace!("{args:#?}");

    // Run application
    app::run(&args).map_err(Into::into)
}
