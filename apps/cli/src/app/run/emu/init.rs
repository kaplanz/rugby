//! Emulator assembly.

use anyhow::{Context, Result, bail, ensure};
use log::{debug, info, warn};
use rugby::core::cart::Cartridge;
use rugby::core::dmg::GameBoy;
use rugby::core::dmg::boot::Boot;
use rugby::extra::cfg::{self, Config};

use super::save;
use crate::app::init;
use crate::dir;

/// Builds an emulator instance.
pub fn emu(cfg: &Config) -> Result<GameBoy> {
    // Load cart ROM
    let mut cart = self::cart(&cfg.cart)
        .context("invalid cartridge")?
        .inspect(|cart| debug!("cartridge header:\n{}", cart.header()));
    // Load cart RAM
    if let Some(cart) = cart.as_mut() {
        save::load(&cfg.cart, cart).context("error flashing save RAM")?;
    }
    // Load boot ROM
    let boot = self::boot(&cfg.boot).context("invalid boot ROM")?;

    // Instantiate emulator
    let mut emu = boot.map_or_else(GameBoy::new, GameBoy::with);
    // Insert cartridge
    if let Some(cart) = cart {
        emu.insert(cart);
    } else {
        // Handle missing cartridge
        ensure!(
            cfg.cart.force,
            "missing cartridge; did not specify `--force`"
        );
        warn!("missing cartridge");
    }

    // Return emulator
    Ok(emu)
}

/// Builds a boot ROM instance.
pub fn boot(args: &cfg::Boot) -> Result<Option<Boot>> {
    // Allow none if skipped
    if args.skip || args.rom.is_none() {
        return Ok(None);
    }
    // Otherwise, extract path
    let Some(path) = &args.rom else {
        bail!("missing path to ROM image");
    };
    // Rebase relative paths
    let path = std::path::absolute(dir::data().join(path))
        .context(format!("invalid path: `{}`", path.display()))?;

    // Read ROM file
    let rom = init::util::load_exact::<0x0100>(&path).context("unable to load ROM image")?;

    // Initialize boot ROM
    let boot = Boot::from(rom);
    info!("loaded boot ROM");

    // Return success
    Ok(Some(boot))
}

/// Builds a cartridge instance.
pub fn cart(args: &cfg::Cart) -> Result<Option<Cartridge>> {
    // Allow none if forced
    if args.force && args.rom.is_none() {
        return Ok(None);
    }
    // Otherwise, extract path
    let Some(path) = &args.rom else {
        bail!("missing path to ROM image");
    };

    // Read ROM file
    //
    // NOTE: Game Paks manufactured by Nintendo have a maximum 8 MiB ROM.
    let rom = init::util::load_until(path, 0x0080_0000).context("unable to load ROM image")?;

    // Initialize cartridge
    let cart = if args.force {
        // If both force and check are supplied, default to force
        if args.check {
            warn!("use of `--force` overrides `--check`");
        }
        // Force cartridge construction
        Cartridge::unchecked
    } else if args.check {
        // Check cartridge integrity
        Cartridge::checked
    } else {
        // Construct a cartridge
        Cartridge::new
    }(&rom)
    .context("unable to construct cartridge")?;
    info!("loaded cart ROM");

    // Return success
    Ok(Some(cart))
}
