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
        debug!("frontend headless");
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
                title = app::util::title(&args.cfg.data.emu.cart),
                speed = freq / f64::from(ppu::RATE)
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
