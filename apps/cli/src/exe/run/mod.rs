//! Emulate provided ROM.

use std::path::Path;
use std::thread;

use anyhow::Context as _;
use constcat::concat;
use log::trace;
use rugby::extra::cfg::Join;

use crate::err::Result;
use crate::{app, cfg, emu, talk};

pub mod cli;

pub use self::cli::Cli;

/// Subcommand name.
pub const NAME: &str = concat!(crate::NAME, "-run");

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
    crate::log::init(args.cfg.data.app.log.as_deref()).context("logger initialization failed")?;
    // Log arguments
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
