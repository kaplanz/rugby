//! Emulator assembly.

use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result, ensure};
use log::{debug, info, warn};
use rugby::core::dmg::{Boot, Cartridge, GameBoy};
use rugby::extra::cfg::{Config, opt};

use super::save;

/// Builds an emulator instance.
pub fn emu(cfg: &Config) -> Result<GameBoy> {
    // Load cart ROM
    let mut cart = self::cart(&cfg.emu.cart)
        .context("invalid cartridge")?
        .inspect(|cart| debug!("cartridge header:\n{}", cart.header()));
    // Load cart RAM
    if let Some(cart) = cart.as_mut() {
        save::load(&cfg.emu.cart, cart).context("error flashing save RAM")?;
    }
    // Load boot ROM
    let boot = self::boot(&cfg.emu.boot).context("invalid boot ROM")?;

    // Instantiate emulator
    let mut emu = boot.map_or_else(GameBoy::new, GameBoy::with);
    // Insert cartridge
    if let Some(cart) = cart {
        emu.insert(cart);
    } else {
        // Handle missing cartridge
        ensure!(
            cfg.emu.cart.force,
            "missing cartridge; did not specify `--force`"
        );
        warn!("missing cartridge");
    }

    // Return emulator
    Ok(emu)
}

/// Builds a boot ROM instance.
pub fn boot(args: &opt::emu::Boot) -> Result<Option<Boot>> {
    // Allow none if skipped
    if args.skip || args.rom.is_none() {
        return Ok(None);
    }
    // Otherwise, extract path
    let path = args.rom.as_deref().context("missing path to ROM image")?;

    // Read ROM file
    let rom = {
        // Open ROM file
        let mut file =
            File::open(path).with_context(|| format!("failed to open: `{}`", path.display()))?;
        // Read ROM into a buffer (must be exactly 256 bytes)
        let mut buf = [0u8; 0x0100];
        file.read_exact(&mut buf)
            .with_context(|| format!("failed to read: `{}`", path.display()))?;
        let nbytes = buf.len();
        debug!(
            "read {size}: `{path}`",
            size = bfmt::Size::from(nbytes),
            path = path.display(),
        );
        // Use ROM contents
        buf
    };

    // Initialize boot ROM
    let boot = Boot::from(rom);
    info!("loaded boot ROM");

    // Return success
    Ok(Some(boot))
}

/// Builds a cartridge instance.
pub fn cart(args: &opt::emu::Cart) -> Result<Option<Cartridge>> {
    // Allow none if forced
    if args.force && args.rom.is_none() {
        return Ok(None);
    }
    // Otherwise, extract path
    let path = args.rom.as_deref().context("missing path to ROM image")?;

    // Read ROM file
    let rom = {
        // Open ROM file
        let file =
            File::open(path).with_context(|| format!("failed to open: `{}`", path.display()))?;
        // Read ROM into a buffer
        let mut buf = Vec::new();
        let nbytes = file
            // Game Paks manufactured by Nintendo have a maximum 8 MiB ROM
            .take(0x0080_0000)
            .read_to_end(&mut buf)
            .with_context(|| format!("failed to read: `{}`", path.display()))?;
        debug!(
            "read {size}: `{path}`",
            size = bfmt::Size::from(nbytes),
            path = path.display(),
        );
        // Use ROM contents
        buf
    };

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
    .with_context(|| format!("failed to load: `{}`", path.display()))?;
    info!("loaded cart ROM");

    // Return success
    Ok(Some(cart))
}
