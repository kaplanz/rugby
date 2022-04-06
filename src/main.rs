use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, ValueHint};
use color_eyre::eyre::{Result, WrapErr};
use gameboy::{Cartridge, Emulator, GameBoy, SCREEN};
use log::info;
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use remus::Machine;

/// Game Boy emulator written in Rust.
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Cartridge ROM image file
    #[clap(parse(from_os_str))]
    #[clap(value_hint = ValueHint::FilePath)]
    rom: PathBuf,
}

fn main() -> Result<()> {
    // Install panic and error report handlers
    color_eyre::install()?;
    // Initialize logger
    env_logger::init();
    // Parse args
    let args = Args::parse();

    // Read the ROM
    let rom = {
        // Open ROM file
        let f = File::open(&args.rom)
            .with_context(|| format!("failed to open ROM: `{}`", args.rom.display()))?;
        // Read ROM into a buffer
        let mut buf = Vec::new();
        // NOTE: Game Paks manufactured by Nintendo have a maximum 8 MiB ROM.
        f.take(0x800000)
            .read_to_end(&mut buf)
            .with_context(|| format!("failed to open ROM: `{}`", args.rom.display()))?;

        buf
    };
    // Initialize the cartridge
    let cart = Cartridge::new(&rom)
        .with_context(|| format!("failed to load cartridge: `{}`", args.rom.display()))?;
    // Extract ROM title from cartridge
    let title = match cart.header().title.replace('\0', " ").trim() {
        "" => "Game Boy",
        title => title,
    }
    .to_string();
    // Create emulator instance
    let mut gb = GameBoy::new(cart);

    // Set up emulator for running
    gb.setup();
    // Create a framebuffer window
    let mut win = Window::new(
        &title,
        SCREEN.width,
        SCREEN.height,
        WindowOptions {
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    )
    .unwrap();

    // Mark the starting time
    let mut now = std::time::Instant::now();
    let mut active = 0;
    // Run emulator on a 4 MiHz clock
    for _ in std::iter::repeat(()) {
        // Perform a single cycle
        gb.cycle();

        // Redraw the screen (if needed)
        gb.redraw(|buf| {
            win.update_with_buffer(buf, SCREEN.width, SCREEN.height)
                .unwrap()
        });

        // Calculate real-time clock frequency
        if now.elapsed().as_secs() > 0 {
            info!("Frequency: {active}");
            active = 0;
            now = std::time::Instant::now();
        }
        active += 1;
    }

    Ok(())
}
