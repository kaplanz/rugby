//! Application utilities.

use std::ffi::OsStr;
use std::path::Path;

use rugby_cfg::opt;

/// Resolves the application title.
pub fn title(args: &opt::emu::Cart) -> &str {
    args.rom
        .as_deref()
        .and_then(Path::file_stem)
        .and_then(OsStr::to_str)
        .unwrap_or(crate::NAME)
}

/// Cartridge utilities.
pub mod rom {
    use std::fs::File;
    use std::io::Read;

    use anyhow::{Context, Result};
    use log::{debug, error};
    use rugby::core::dmg::Cartridge;
    use rugby::core::dmg::cart::mbc::Mbc;
    use rugby_cfg::opt;
    use rugby_cfg::opt::emu::When;

    /// Flashes the cartridge RAM from a save file.
    pub fn flash(args: &opt::emu::Cart, cart: &mut Cartridge) -> Result<()> {
        let Some(path) = args.ram() else {
            return Ok(());
        };
        if let Some(When::Never) = args.save {
            return Ok(());
        }
        if matches!(args.save, Some(When::Auto)) && !cart.header().info.has_battery() {
            return Ok(());
        }
        if !cart.header().info.has_ram() {
            error!("cannot flash: cartridge does not support RAM");
            return Ok(());
        }
        if !path.exists() {
            return Ok(());
        }

        // Open RAM file
        let mut file = File::open(&path)
            .with_context(|| format!("failed to open: `{}`", path.display()))?
            .take(0x0002_0000); // cartridge ROM has a maximum of 128 KiB
        debug!("reading RAM image: `{}`", path.display());

        // Load into cartridge
        cart.body_mut()
            .flash(&mut file)
            .with_context(|| format!("failed to read: `{}", path.display()))?;

        Ok(())
    }

    /// Dumps the cartridge RAM to a save file.
    pub fn dump(args: &opt::emu::Cart, cart: &Cartridge) -> Result<()> {
        let Some(path) = args.ram() else {
            return Ok(());
        };
        if let Some(When::Never) = args.save {
            return Ok(());
        }
        if matches!(args.save, Some(When::Auto)) && !cart.header().info.has_battery() {
            return Ok(());
        }
        if !cart.header().info.has_ram() {
            error!("cannot dump: cartridge does not support RAM");
            return Ok(());
        }

        // Open RAM file
        let mut file =
            File::create(&path).with_context(|| format!("failed to open: `{}`", path.display()))?;
        debug!("writing RAM image: `{}`", path.display());

        // Save from cartridge
        cart.body()
            .dump(&mut file)
            .with_context(|| format!("failed to write: `{}", path.display()))?;

        Ok(())
    }
}
