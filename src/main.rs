use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueHint};
use gameboy::{Cartridge, GameBoy};

/// Game Boy emulator written in Rust.
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Cartridge ROM image file
    #[clap(parse(from_os_str))]
    #[clap(value_hint = ValueHint::FilePath)]
    rom: PathBuf,
}

fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse args
    let args = Args::parse();

    // Read the ROM
    let rom = fs::read(&args.rom)?;

    // Initialize the cartridge
    let cart = Cartridge::new(&rom)?;

    // Create emulator instance
    let mut gb = GameBoy::new(cart);

    // Run emulator
    gb.run();

    Ok(())
}
