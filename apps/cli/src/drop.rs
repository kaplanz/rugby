use anyhow::{Context, Result};
use rugby::core::dmg::GameBoy;
use rugby_cfg::Config;

use crate::util;

/// Destroys the emulator instance.
pub fn emu(mut this: GameBoy, args: &Config) -> Result<()> {
    // Eject cartridge
    if let Some(cart) = this.eject() {
        // Save cart RAM
        util::rom::dump(&args.emu.cart, &cart).context("error dumping save RAM")?;
    }

    Ok(())
}