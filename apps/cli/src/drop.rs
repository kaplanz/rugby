use anyhow::{Context, Result};
use rugby::core::dmg::GameBoy;
use rugby_cfg::opt;

use crate::app::App;
use crate::exe::run::Cli;
use crate::util;

/// Destroys an `App`.
impl App {
    pub fn drop(self, args: &Cli) -> Result<()> {
        // Destroy emulator
        self::emu(self.emu, &args.cfg.data.emu.cart)?;

        Ok(())
    }
}

/// Destroys an emulator instance.
pub fn emu(mut this: GameBoy, args: &opt::emu::Cart) -> Result<()> {
    // Eject cartridge
    if let Some(cart) = this.eject() {
        // Save cart RAM
        util::rom::dump(args, &cart).context("failed to dump cartridge RAM")?;
    }

    Ok(())
}
