//! Emulate provided ROM.

use std::path::Path;

use anyhow::Context;
use log::trace;
use rugby_cfg::Join;

use crate::err::Result;
use crate::{cfg, init, util};

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
    #[cfg_attr(not(feature = "gbd"), allow(unused, clippy::let_unit_value))]
    let log = init::log(args.cfg.data.app.log.as_deref().unwrap_or_default())
        .context("could not initialize logger")?;
    // Log previous steps
    trace!("{args:#?}");

    // Prepare emulator
    let emu = init::emu(&args.cfg.data)?;
    // Perform early exit
    if args.feat.exit {
        return Ok(());
    }
    // Prepare application
    let mut app = init::app(&args, emu, log)?;
    // Run application
    app.run()?;
    // Dump cartridge RAM
    util::rom::dump(
        app.emu.eject().as_ref(),
        args.cfg.data.emu.cart.ram().as_deref(),
        args.cfg.data.emu.cart.save.unwrap_or_default(),
    )
    .context("could not dump RAM")?;

    // Terminate normally
    Ok(())
}
