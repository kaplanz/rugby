//! Cartridge RAM.

use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result};
use log::{debug, error, info, trace};
use rugby::core::dmg::Cartridge;
use rugby::extra::cfg::opt;
use rugby::extra::cfg::opt::emu::When;

/// Loads the cartridge RAM from a save file.
pub fn load(args: &opt::emu::Cart, cart: &mut Cartridge) -> Result<()> {
    let Some(path) = args.ram() else {
        return Ok(());
    };
    match args.save.unwrap_or_default() {
        When::Never => {
            trace!("load RAM disabled by user");
            return Ok(());
        }
        When::Auto => {
            let board = &cart.header().board;
            if board.has_ram() && board.has_battery() {
                trace!("load RAM automatically enabled");
            } else {
                trace!("load RAM automatically disabled");
                return Ok(());
            }
        }
        When::Always => {
            trace!("load RAM enabled by user");
        }
    }
    if !cart.header().board.has_ram() {
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
    info!("loaded cart RAM");

    Ok(())
}

/// Dumps the cartridge RAM to a save file.
pub fn dump(args: &opt::emu::Cart, cart: &Cartridge) -> Result<()> {
    let Some(path) = args.ram() else {
        return Ok(());
    };
    match args.save.unwrap_or_default() {
        When::Never => {
            trace!("dump RAM disabled by user");
            return Ok(());
        }
        When::Auto => {
            let board = &cart.header().board;
            if board.has_ram() && board.has_battery() {
                trace!("dump RAM automatically enabled");
            } else {
                trace!("dump RAM automatically disabled");
                return Ok(());
            }
        }
        When::Always => {
            trace!("dump RAM enabled by user");
        }
    }
    if !cart.header().board.has_ram() {
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
    info!("dumped cart RAM");

    Ok(())
}
