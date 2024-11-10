//! Emulate provided ROM.

use std::path::Path;

use anyhow::Context;
use log::trace;
use rugby_cfg::Join;

use crate::app::App;
use crate::err::Result;
use crate::{cfg, init};

pub mod cli;

pub use self::cli::Cli;

/// [`Run`](crate::cli::Command::Run) entrypoint.
pub fn main(mut args: Cli) -> Result<()> {
    // Load configuration
    args.cfg.data.merge({
        // Parse config from file
        let mut cfg = cfg::load(&args.cfg.path)?;
        // Rebase paths to parent
        cfg.rebase(args.cfg.path.parent().unwrap_or(Path::new("")));
        // Merge with parsed args
        cfg
    });
    // Initialize logger
    #[cfg_attr(not(feature = "gbd"), allow(unused))]
    let filter = init::log(args.cfg.data.app.log.as_deref().unwrap_or_default())
        .context("could not initialize logger")?;
    // Log previous steps
    trace!("{args:#?}");

    // Initialize application
    let mut app = App::new(&args).context("startup sequence failed")?;
    // Install reload handle
    #[cfg(feature = "gbd")]
    app.logger(filter);

    // Run application
    let res = (|| -> Result<()> {
        loop {
            // Check termination
            if app.done() {
                return Ok(());
            }
            // Cycle application
            app.main()?;
        }
    })(); // NOTE: This weird syntax is in lieu of using unstable try blocks.

    // Destroy emulator
    app.drop(&args).context("shutdown sequence failed")?;
    // Forward app errors
    res.context("an application error occurred")?;
    // Terminate normally
    Ok(())
}
