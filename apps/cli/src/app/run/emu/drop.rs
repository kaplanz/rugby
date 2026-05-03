//! Emulator teardown.

use anyhow::{Context, Result};
use rugby::core::dmg::GameBoy;

use super::save;
use crate::exe::run::Cli;

/// Destroys the emulator instance.
pub fn emu(mut this: GameBoy, args: &Cli) -> Result<()> {
    // Eject cartridge
    if let Some(cart) = this.eject() {
        // Save cart RAM
        save::dump(args.cli.cart.rom.as_ref(), &args.cfg.data.cart, &cart)
            .context("error dumping save RAM")?;
    }

    Ok(())
}
