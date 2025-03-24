//! Emulator thread.

use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use log::{debug, trace};
use rugby::arch::Block;
use rugby::core::dmg::{self, ppu};
use rugby::emu::part::audio::Audio;
use rugby::emu::part::joypad::Joypad;
use rugby::emu::part::video::Video;
use rugby::prelude::Core;

use self::perf::Profiler;
use crate::app;
use crate::exe::run::Cli;

pub mod drop;
pub mod init;
pub mod perf;
pub mod save;

/// Emulator context.
#[derive(Debug, Default)]
pub struct Context {
    /// Sync timestamp.
    pub awake: Option<Instant>,
    /// Cycle counter.
    pub count: Profiler,
    /// Pause emulator.
    pub pause: bool,
}

/// Emulator main.
pub fn main(args: &Cli) -> Result<()> {
    // Instantiate emulator
    let mut emu = init::emu(&args.cfg.data)?;
    // Instantiate context
    let mut ctx = Context::default();
    // Initialize tracing
    #[cfg(feature = "trace")]
    let mut trace = args
        .dbg
        .trace
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
            // Once woken, restart loop to determine state
            continue;
        }
        // Record emulation time
        //
        // This will be used for frequency synchronization, to ensure that
        let watch = Instant::now();
        // Synchronize frequency
        //
        // To ensure the emulator is clocked at the target frequency, if it is
        // running ahead of schedule, sleep until around when we expect the next
        // cycles to run.
        if let Some(awake) = ctx.awake {
            if awake > watch {
                // Not yet ready for work... sleep until ready.
                thread::sleep(awake - watch);
            } else {
                // When lagging behind, unset the wake-up time. This has the
                // effect of resetting synchronization against the current time,
                // rather than against the previous pace.
                trace!("emulator thread stalled");
                ctx.awake = None;
            }
            // At this point, emulator is ready to perform work...
        }

        // Forward joypad input
        let keys = app::data::input::take();
        if !keys.is_empty() {
            emu.inside_mut().joypad().recv(keys);
        }
        // Perform emulation work
        let count = ppu::RATE;
        for _ in 0..count {
            // Cycle emulator
            emu.cycle();

            // Sample audio
            app::data::audio::push(emu.inside().audio().sample().mix());
            // Sample video
            if !args.feat.headless && emu.inside().video().vsync() {
                // Video frame.
                app::data::video::draw(emu.inside().video().frame().into());
                // Debug frame.
                #[cfg(feature = "gfx")]
                if args.dbg.gfx {
                    app::data::debug::gfx::draw(dmg::dbg::ppu(&emu));
                }
            }

            // Trace execution
            #[cfg(feature = "trace")]
            if let Some(trace) = trace.as_mut() {
                if matches!(
                    emu.main.soc.cpu.stage(),
                    dmg::cpu::Stage::Fetch | dmg::cpu::Stage::Done
                ) && ctx.count.cycle % 4 == 0
                {
                    trace.log(&emu).context("could not write trace entry")?;
                }
            }
        }

        // Update synchronization delay
        ctx.awake = if let Some(freq) = args.cfg.data.app.spd.clone().unwrap_or_default().freq() {
            // Calculate sync delay for executed cycles
            let delay = Duration::from_secs(1) * count / freq;
            // Subtract emulation time from delay
            Some(ctx.awake.unwrap_or(watch) + delay)
        } else {
            // When running without a target frequency (i.e. at maximum speed),
            // no synchronization delay is needed.
            None
        };

        // Synchronize profiler
        ctx.count.tick_by(count);
        if let Some(freq) = ctx.count.report_delay() {
            // Log performance
            debug!(
                "frequency: {freq:>7.4} MHz, speedup: {pace:>4.2}x, frames: {rate:>6.2} FPS",
                freq = freq / 1e6,
                pace = freq / f64::from(dmg::FREQ),
                rate = freq / f64::from(ppu::RATE)
            );
            // Set performance
            app::data::bench::update(freq);
        }
    }

    // Destroy emulator
    drop::emu(emu, &args.cfg.data).context("shutdown sequence failed")?;

    Ok(())
}
