//! Application utilities.

/// Cartridge utilities.
pub mod rom {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use anyhow::{Context, Result};
    use log::{error, info};
    use rugby::core::dmg::cart::mbc::Mbc;
    use rugby::core::dmg::Cartridge;
    use rugby_cfg::opt::emu::When;

    /// Flashes the cartridge RAM from a save file.
    pub fn flash(path: Option<&Path>, cart: Option<&mut Cartridge>, save: When) -> Result<()> {
        let Some(path) = path else {
            return Ok(());
        };
        let Some(cart) = cart else {
            return Ok(());
        };
        if save == When::Never {
            return Ok(());
        }
        if save == When::Auto && !cart.header().info.has_battery() {
            return Ok(());
        }
        if !path.exists() {
            return Ok(());
        }

        // Open RAM file
        let mut file = File::open(path)
            .with_context(|| format!("failed to open: `{}`", path.display()))?
            .take(0x0002_0000); // cartridge ROM has a maximum of 128 KiB

        // Load into cartridge
        let nbytes = cart
            .body_mut()
            .flash(&mut file)
            .with_context(|| format!("failed to read: `{}", path.display()))?;
        info!("flashed {nbytes} bytes: `{}`", path.display());

        Ok(())
    }

    /// Dumps the cartridge RAM to a save file.
    pub fn dump(cart: Option<&Cartridge>, path: Option<&Path>, save: When) -> Result<()> {
        let Some(cart) = cart else {
            return Ok(());
        };
        let Some(path) = path else {
            return Ok(());
        };
        if save == When::Never {
            return Ok(());
        }
        if save == When::Auto && !cart.header().info.has_battery() {
            return Ok(());
        }
        if !cart.header().info.has_ram() {
            error!("cannot dump: cartridge does not support RAM");
            return Ok(());
        }

        // Open RAM file
        let mut file =
            File::create(path).with_context(|| format!("failed to open: `{}`", path.display()))?;

        // Save from cartridge
        let nbytes = cart
            .body()
            .dump(&mut file)
            .with_context(|| format!("failed to write: `{}", path.display()))?;
        info!("dumped {nbytes} bytes: `{}`", path.display());

        Ok(())
    }
}
