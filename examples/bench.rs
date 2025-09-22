use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use rugby::arch::Block;
use rugby::core::dmg::{CLOCK, Cartridge, GameBoy};

#[allow(unused)]
#[path = "../apps/cli/src/app/run/emu/perf.rs"]
mod perf;

/// Sample cart ROM.
const GAME: &[u8; 0x10000] = include_bytes!("../roms/games/porklike/porklike.gb");

/// Application exit flag.
///
/// This value, `false` at initialization, will change to `true` exactly
/// once during the lifetime of the program, signaling to all threads that
/// they should exit.
static EXIT: AtomicBool = AtomicBool::new(false);

fn main() {
    // Instantiate a cartridge
    let cart = Cartridge::new(GAME).unwrap();
    // Create an emulator instance
    let mut emu = GameBoy::new();
    // Load the cartridge
    emu.insert(cart);

    // Start exit timer
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        // Exit after timer
        EXIT.store(true, Ordering::Relaxed);
    });

    // Initialize profiler
    let mut prof = perf::Profiler::new();

    // Loop until exit
    while !EXIT.load(Ordering::Relaxed) {
        // Timestamp iteration start
        let instant = Instant::now();
        // Perform emulation work
        for _ in 0..CLOCK {
            emu.cycle();
        }
        // Compute elapsed time
        let runtime = instant.elapsed();
        let speedup = runtime.as_secs_f64().recip();
        // Print iteration statistics
        println!("runtime: {runtime:>8.2?}, speedup: {speedup:>4.2}x");
        // Update profiler
        prof.tick_by(CLOCK);
        if let Some(freq) = prof.report_delay() {
            println!("average: {:>4.2}x", freq / f64::from(CLOCK));
        }
    }
}
