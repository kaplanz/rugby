//! Frontend thread.

use anyhow::Result;
use log::debug;
use rugby::app::joypad::Joypad;
use rugby::app::video::Video;
use rugby::core::dmg::ppu;

use crate::app::{self, Exit};
use crate::exe::run::Cli;

mod init;

/// Frontend main.
pub fn main(args: &Cli) -> Result<()> {
    // No-op if headless
    if args.feat.headless {
        debug!("graphics disabled");
        return Ok(());
    }

    // Instantiate frontend
    let mut gui = init::gui(args)?;

    // Frontend loop
    //
    // Until the main window is closed, this loops is responsible for handling
    // all graphics logic.
    while app::running() {
        // Check liveness
        if !gui.alive() {
            // Exit when closed
            app::exit(Exit::Frontend);
            continue;
        }

        // Poll key events
        let keys = gui.events();
        if !keys.is_empty() {
            debug!("keys: {keys:?}");
            app::data::input::send(keys);
        }

        // Draw main window
        if let Some(frame) = app::data::video::take() {
            // Redraw frame
            gui.draw(frame);
        } else {
            // Sync window
            gui.lcd.update();
        }

        // Report benchmark
        if let Some(freq) = app::data::bench::report() {
            // Update window title
            gui.lcd.title(&format!(
                "{title} ({speed:.1} FPS)",
                title = util::title(&args.cfg.data.emu.cart),
                speed = freq / f64::from(ppu::VIDEO)
            ));
        }

        // Draw debug windows
        #[cfg(feature = "gfx")]
        if let Some(frame) = app::data::debug::gfx::take() {
            gui.gfx(frame)?;
        }
    }

    Ok(())
}

/// Frontend utilities.
mod util {
    use std::ffi::OsStr;
    use std::path::Path;

    use rugby::extra::cfg::opt;

    /// Resolves the application title.
    pub fn title(args: &opt::emu::Cart) -> &str {
        args.rom
            .as_deref()
            .and_then(Path::file_stem)
            .and_then(OsStr::to_str)
            .unwrap_or(crate::NAME)
    }
}
