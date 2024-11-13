//! Emulate provided ROM.

use std::path::Path;
use std::thread;

use anyhow::Context as _;
use log::trace;
use rugby_cfg::Join;

use crate::err::Result;
use crate::{app, cfg, emu, talk};

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
    crate::log::init(args.cfg.data.app.log.as_deref().unwrap_or_default())
        .context("logger initialization failed")?;
    // Log previous steps
    trace!("{args:#?}");

    // Run application
    let res = thread::scope(|s| {
        // Initialize channels
        let channel = talk::pair::<emu::Message, app::Message>();

        // Run emulator on another thread
        let emu = s.spawn(|| emu::main(&args, channel.0));
        // Run frontend on main thread
        let app = app::main(&args, channel.1);

        // Forward result
        emu.join().expect("emulator thread panicked").and(app)
    });

    // Return runtime errors
    res.map_err(Into::into)
}
