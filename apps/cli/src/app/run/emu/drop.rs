//! Emulator teardown.

use anyhow::{Context, Result};
use rugby::core::dmg::GameBoy;
use rugby::extra::cfg::Config;

use super::save;

/// Destroys the emulator instance.
pub fn emu(mut this: GameBoy, args: &Config) -> Result<()> {
    // Eject cartridge
    if let Some(cart) = this.eject() {
        // Save cart RAM
        save::dump(&args.emu.cart, &cart).context("error dumping save RAM")?;
    }

    Ok(())
}
