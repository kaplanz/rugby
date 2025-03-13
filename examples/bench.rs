use std::time::Instant;

use rugby::arch::Block;
use rugby::core::dmg::{Cartridge, FREQ, GameBoy};

const GAME: &[u8; 0x10000] = include_bytes!("../roms/games/porklike/porklike.gb");

fn main() {
    // Instantiate a cartridge
    let cart = Cartridge::new(GAME).unwrap();
    // Create an emulator instance
    let mut emu = GameBoy::new();
    // Load the cartridge
    emu.insert(cart);

    // Loop forever
    loop {
        // Timestamp iteration start
        let instant = Instant::now();
        for _ in 0..FREQ {
            emu.cycle()
        }
        // Compute elapsed time
        let runtime = instant.elapsed();
        let speedup = runtime.as_secs_f64().recip();
        // Print iteration statistics
        println!("runtime: {runtime:>8.2?}, speedup: {speedup:>4.2}x");
    }
}
