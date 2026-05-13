//! Emulator thread.

use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use log::{debug, info};
use rugby::api::audio::Audio;
use rugby::api::input::Input;
use rugby::api::video::Video;
use rugby::arch::Block;
use rugby::core::chip::{cpu, ppu};
use rugby::core::dmg;

use crate::app;
#[cfg(feature = "trace")]
use crate::app::dbg::trace;
use crate::exe::run::Cli;

pub mod drop;
pub mod init;
pub mod perf;
pub mod save;
pub mod sync;

use self::perf::Profiler;
use self::sync::Clocking;

/// Emulator context.
#[derive(Debug)]
pub struct Context {
    /// Pause signal.
    pub pause: bool,
    /// Clock timings.
    pub clock: Clocking,
    /// Batch counter.
    pub batch: Profiler,
    /// Start instant.
    pub start: Instant,
    /// Total counter.
    pub total: u64,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            pause: false,
            clock: Clocking::default(),
            batch: Profiler::default(),
            start: Instant::now(),
            total: u64::default(),
        }
    }
}

/// Emulator main.
pub fn main(args: &Cli) -> Result<()> {
    // Instantiate emulator
    let mut emu = init::emu(args)?;
    // Instantiate context
    let mut ctx = Context::default();
    // Prepare clocking
    ctx.clock.frq = args.cli.spd.clone().unwrap_or_default().freq();
    // Initialize tracing
    #[cfg(feature = "trace")]
    let mut trace = args
        .dbg
        .tracer
        .as_ref()
        .map(app::dbg::trace::init)
        .transpose()
        .context("tracelog initialization failed")?;

    // Emulator loop
    //
    // Until the emulator exits, this loops is responsible for handling all
    // emulation logic. This includes cycling the system core, synchronizing
    // emulation frames to the wall-clock, and processing core input/output.
    //
    // If enabled, the debugger is also cycled here inline to manage the core.
    while app::running() {
        // Sleep while paused
        //
        // This preserves host compute cycles that would otherwise be needlessly
        // wasted spinning.
        if ctx.pause {
            // Use delay that is negligible in human time
            thread::sleep(Duration::from_millis(10));
            // Once woken, restart loop to re-synchronize
            continue;
        }

        // Synchronize thread
        //
        // Ensures the thread doesn't exceed nominal frequency as configured.
        if ctx.clock.sync() {
            continue;
        }

        // Cycle emulator
        //
        // Advances the emulator by a single virtual clock cycle.
        emu.cycle();

        // Sample audio
        //
        // Audio is sampled each cycle in order to ensure the audio system
        // remain busy, otherwise, audible "pops" will sound.
        if !args.cli.mute {
            app::data::audio::push(emu.sample().mix());
        }

        // Sample video
        //
        // Video is sampled only once per vsync, then the emulator indicates it
        // has completed drawing the frame.
        if !args.cli.headless && emu.vsync() {
            // Render video frame
            app::data::video::draw(emu.frame().into());
            // Render debug frame
            //
            // This contains a graphical representation of the contents of VRAM.
            #[cfg(feature = "gfx")]
            if args.dbg.vram {
                let debug = match &emu {
                    rugby::GameBoy::Dmg0(dmg) => dmg::dbg::ppu(dmg),
                    rugby::GameBoy::DmgA(dmg)
                    | rugby::GameBoy::DmgB(dmg)
                    | rugby::GameBoy::DmgC(dmg) => dmg::dbg::ppu(dmg),
                    _ => unreachable!(),
                };
                app::data::debug::gfx::draw(debug);
            }
        }

        // Sample trace
        //
        // Processor tracing only occurs on the the first T-cycle of the CPU's
        // fetch/done stage.
        #[cfg(feature = "trace")]
        if let Some(trace) = trace.as_mut()
            && ctx.total % 4 == 0
        {
            let stage = match &emu {
                rugby::GameBoy::Dmg0(dmg) => dmg.main.soc.cpu.stage(),
                rugby::GameBoy::DmgA(dmg)
                | rugby::GameBoy::DmgB(dmg)
                | rugby::GameBoy::DmgC(dmg) => dmg.main.soc.cpu.stage(),
                _ => unreachable!(),
            };
            if matches!(stage, cpu::Stage::Fetch | cpu::Stage::Done) {
                match trace.emit(&emu) {
                    // Exit on completion
                    Err(trace::Error::Finished) => {
                        info!("trace comparison successful");
                        app::exit(app::Exit::Tracelog);
                        break;
                    }
                    res => res.context("failed to emit trace entry")?,
                }
            }
        }

        // Perform lower-frequency actions
        if ctx.total % u64::from(ctx.clock.frq.unwrap_or(dmg::CLOCK) / 64) == 0 {
            // Sample input
            //
            // Joypad input is sampled to the emulator ~64 times per second, as
            // doing so more often impacts performance and shouldn't be
            // noticeable to users. This improves overall emulation efficiency.
            let keys = app::data::input::take();
            if !keys.is_empty() {
                emu.recv(keys);
            }

            // Report performance
            //
            // Approximately once per second, we should generate a performance
            // report. This will be logged and updated in the window's title.
            if let Some(freq) = ctx.batch.report_delay() {
                // Log performance
                debug!("{}", self::frequency(freq));
                // Set performance
                app::data::bench::update(freq);
            }
        }

        // Count clocked cycle
        ctx.clock.tick();
        ctx.batch.tick();
        ctx.total += 1;
    }

    // Report benchmark
    let tick = ctx.total;
    let time = ctx.start.elapsed();
    info!("{}", self::benchmark(tick, time));
    // Report frequency
    let mean = ctx.clock.perf().report();
    info!("{}", self::frequency(mean));

    // Destroy emulator
    drop::emu(emu, args).context("shutdown sequence failed")?;

    Ok(())
}

/// Generates a benchmark report.
fn benchmark(tick: u64, time: Duration) -> String {
    format!(
        "benchmark: {div:>3}.{rem:06} MCy, elapsed: {time:>4.2?}",
        div = tick / 1_000_000,
        rem = tick % 1_000_000,
    )
}

/// Generates a frequency report.
fn frequency(freq: f64) -> String {
    format!(
        "frequency: {freq:>10.6} MHz, speedup: {pace:>4.2}x, frames: {rate:>6.2} FPS",
        freq = freq / 1e6,
        pace = freq / f64::from(dmg::CLOCK),
        rate = freq / f64::from(ppu::FRAME)
    )
}
