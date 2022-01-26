use std::path::PathBuf;

use clap::{Parser, ValueHint};
use gameboy::GameBoy;

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

    // Create emulator instance
    let mut gb = GameBoy::new();

    // Load cartridge image
    gb.load(&args.rom)?;

    // Start emulation
    gb.start();

    Ok(())
}
