//! Graphics assembly.

use anyhow::{Context as _, Result};
use rugby::core::dmg::LCD;
use rugby::extra::pal::Palette;

use super::util;
use crate::app::gui::Frontend;
use crate::exe::run::Cli;

/// Builds a frontend instance.
pub fn gui(args: &Cli) -> Result<Frontend> {
    // Construct frontend
    let mut gui = Frontend::new(args).context("could not open frontend")?;

    // Initialize main window
    let pal = Palette::from(args.cfg.data.app.pal.clone().unwrap_or_default());
    gui.lcd.redraw(&vec![pal[0].into(); LCD.depth()])?;
    gui.lcd.title(util::title(&args.cfg.data.emu.cart));
    // Open debug windows
    #[cfg(feature = "gfx")]
    if args.dbg.gfx {
        gui.dbg.open().context("could not open debug windows")?;
    }

    Ok(gui)
}
