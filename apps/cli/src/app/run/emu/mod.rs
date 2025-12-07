//! Emulator thread.

use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use log::{debug, info};
use rugby::arch::Block;
use rugby::core::dmg::{self, ppu};
use rugby::emu::part::audio::Audio;
use rugby::emu::part::joypad::Joypad;
use rugby::emu::part::video::Video;
use rugby::prelude::Core;

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
    let mut emu = init::emu(&args.cfg.data)?;
    // Instantiate context
    let mut ctx = Context::default();
    // Prepare clocking
    ctx.clock.frq = args.cfg.data.app.spd.clone().unwrap_or_default().freq();
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
        if !args.feat.mute {
            app::data::audio::push(emu.inside().audio().sample().mix());
        }

        // Sample video
        //
        // Video is sampled only once per vsync, then the emulator indicates it
        // has completed drawing the frame.
        if !args.feat.headless && emu.inside().video().vsync() {
            // Render video frame
            app::data::video::draw(emu.inside().video().frame().into());
            // Render debug frame
            //
            // This contains a graphical representation of the contents of VRAM.
            #[cfg(feature = "gfx")]
            if args.dbg.vram {
                app::data::debug::gfx::draw(dmg::dbg::ppu(&emu));
            }
        }

        // Sample trace
        //
        // Processor tracing only occurs on the the first T-cycle of the CPU's
        // fetch/done stage.
        #[cfg(feature = "trace")]
        if let Some(trace) = trace.as_mut()
            && matches!(
                emu.main.soc.cpu.stage(),
                dmg::cpu::Stage::Fetch | dmg::cpu::Stage::Done
            )
            && ctx.total % 4 == 0
        {
            match trace.emit(&emu) {
                // Exit on completion
                Err(trace::Error::Finished) => {
                    info!("trace comparison successful");
                    app::exit(app::Exit::Tracecmp);
                    break;
                }
                res => res.context("failed to emit trace entry")?,
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
                emu.inside_mut().joypad().recv(keys);
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
    drop::emu(emu, &args.cfg.data).context("shutdown sequence failed")?;

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
        rate = freq / f64::from(ppu::VIDEO)
    )
}
