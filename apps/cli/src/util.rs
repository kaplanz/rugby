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

/// Cartridge RAM.
pub mod ram {
    use std::fs::File;
    use std::io::Read;

    use anyhow::{Context, Result};
    use log::{debug, error};
    use rugby::core::dmg::Cartridge;
    use rugby::core::dmg::cart::mbc::Mbc;
    use rugby_cfg::opt;
    use rugby_cfg::opt::emu::When;

    /// Loads the cartridge RAM from a save file.
    pub fn load(args: &opt::emu::Cart, cart: &mut Cartridge) -> Result<()> {
        let Some(path) = args.ram() else {
            return Ok(());
        };
        match args.save.unwrap_or_default() {
            When::Never => {
                debug!("load RAM disabled by user");
                return Ok(());
            }
            When::Auto => {
                let info = &cart.header().info;
                if info.has_ram() && info.has_battery() {
                    debug!("load RAM automatically enabled");
                } else {
                    debug!("load RAM automatically disabled");
                    return Ok(());
                }
            }
            When::Always => {
                debug!("load RAM enabled by user");
            }
        }
        if !cart.header().info.has_ram() {
            error!("cannot load: cartridge does not support RAM");
            return Ok(());
        }
        if !path.exists() {
            return Ok(());
        }

        // Open RAM file
        let mut file = File::open(&path)
            .with_context(|| format!("failed to open: `{}`", path.display()))?
            .take(0x0002_0000); // cartridge ROM has a maximum of 128 KiB
        // Load into cartridge
        let nbytes = cart
            .body_mut()
            .flash(&mut file)
            .with_context(|| format!("failed to read: `{}", path.display()))?;
        debug!(
            "read {size}: `{path}`",
            size = bfmt::Size::from(nbytes),
            path = path.display(),
        );

        Ok(())
    }

    /// Dumps the cartridge RAM to a save file.
    pub fn dump(args: &opt::emu::Cart, cart: &Cartridge) -> Result<()> {
        let Some(path) = args.ram() else {
            return Ok(());
        };
        match args.save.unwrap_or_default() {
            When::Never => {
                debug!("dump RAM disabled by user");
                return Ok(());
            }
            When::Auto => {
                let info = &cart.header().info;
                if info.has_ram() && info.has_battery() {
                    debug!("dump RAM automatically enabled");
                } else {
                    debug!("dump RAM automatically disabled");
                    return Ok(());
                }
            }
            When::Always => {
                debug!("dump RAM enabled by user");
            }
        }
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
        // Save from cartridge
        let nbytes = cart
            .body()
            .dump(&mut file)
            .with_context(|| format!("failed to write: `{}", path.display()))?;
        debug!(
            "read {size}: `{path}`",
            size = bfmt::Size::from(nbytes),
            path = path.display(),
        );

        Ok(())
    }
}
