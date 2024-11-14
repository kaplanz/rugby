//! Emulator thread.

use std::mem;
use std::time::Instant;

use anyhow::{Context as _, Result};
use log::{debug, trace};
use rugby::arch::Block;
#[cfg(feature = "win")]
use rugby::core::dmg;
#[cfg(feature = "log")]
use rugby::core::dmg::cpu;
use rugby::emu::part::joypad::Joypad;
use rugby::emu::part::video::Video;
use rugby::prelude::Core;

use self::ctx::Context;
use self::msg::{Data, Sync};
use crate::exe::run::Cli;
use crate::talk::{self, Channel};
use crate::{app, drop, init};

pub mod ctx;
pub mod msg;

pub use self::msg::Message;

/// Clock divider.
///
/// As an optimization for more efficient synchronization, divide the target
/// frequency by this number, but clock this number as many cycles each time.
pub const DIVIDER: u32 = 0x100;

// Emulator main.
#[allow(clippy::too_many_lines)]
pub fn main(args: &Cli, mut talk: Channel<Message, app::Message>) -> Result<()> {
    // Instantiate emulator
    let mut emu = init::emu(&args.cfg.data).context("startup sequence failed")?;
    // Initialize debugger
    #[cfg(feature = "gbd")]
    let mut gbd = args
        .dbg
        .gbd
        .then(init::gbd)
        .transpose()
        .context("debugger initialization failed")?;
    // Initialize tracing instance
    #[cfg(feature = "log")]
    let mut log = args
        .dbg
        .trace
        .as_ref()
        .map(init::log)
        .transpose()
        .context("tracelog initialization failed")?;

    // Emulator loop
    let mut res = (|| -> Result<()> {
        // Initialize context
        let mut ctx = Context::new(&args.cfg.data);
        // Await initial start
        ctx.pause();

        loop {
            // Read messages
            let msg = if ctx.pause {
                // Blocking
                talk.wait().map(Some)?
            } else {
                // Non-blocking
                talk.recv()?
            };
            // Process messages
            if let Some(msg) = msg {
                trace!("recv: {msg}");
                match msg {
                    Message::Play => {
                        // Play emulator
                        debug!("{msg}");
                        ctx.resume();
                    }
                    Message::Stop => {
                        // Stop emulator
                        debug!("{msg}");
                        ctx.pause();
                    }
                    #[cfg(feature = "gbd")]
                    Message::Break => {
                        // Enable debugger
                        debug!("{msg}");
                        gbd.as_mut()
                            .context("cannot break; debugger not in use")?
                            .enable();
                    }
                    Message::Data(Data::Joypad(keys)) => {
                        // Receive joypad events
                        debug!("joypad input: {keys:?}");
                        emu.inside_mut().joypad().recv(keys);
                    }
                    Message::Data(Data::Serial(_)) => {
                        // Receive serial data
                        debug!("{msg}");
                        todo!()
                    }
                    Message::Sync(Sync::Video) => {
                        // Acknowledge video
                        //
                        // # Note
                        //
                        // By performing a handshake acknowledging video frame
                        // receipt, we prevent the emulator from continually
                        // sending frames when the application may not have
                        // processed the previous one. If the application is
                        // hanging, this has the effect of dropping missed
                        // frames. It also prevents unwanted traffic on the
                        // channel.
                        ctx.video = true;
                    }
                    Message::Exit => {
                        // Exit requested
                        debug!("{msg}");
                        return Ok(());
                    }
                }
                // Process next message
                continue;
            }
            // Skip if paused
            if ctx.pause {
                continue;
            }
            // Process debugger
            #[cfg(feature = "gbd")]
            if let Some(gbd) = gbd.as_mut() {
                // Sync with emulator
                gbd.sync(&emu);
                // Check if enabled
                if gbd.ready() {
                    // Stop emulator
                    ctx.pause();
                    // Run debugger
                    let res = gbd.run(&mut emu);
                    // Play emulator
                    ctx.resume();
                    // Quit if requested
                    if let Err(rugby_gbd::Error::Quit) = res {
                        // Exit frontend
                        talk.send(app::Message::Exit(app::Exit::Debugger))?;
                        // Exit emulator
                        return Ok(());
                    }
                }
                // Cycle debugger
                gbd.cycle();
            }
            // Sync wall-clock
            if ctx.count.cycle() % DIVIDER == 0 {
                ctx.clock.as_mut().map(Iterator::next);
            }
            // Cycle emulator
            emu.cycle();
            // Send video frame
            let video = emu.inside().video();
            if video.vsync() && {
                // To prevent overwhelming the frontend, we disable the video
                // before we send a frame. It will be re-enabled after receipt
                // if acknowledged.
                mem::take(&mut ctx.video)
            } {
                // Collect and send completed frame
                let frame = video.frame().to_owned().into_boxed_slice();
                talk.send(app::Message::Video(frame))?;
            }
            // Send VRAM debug info
            #[cfg(feature = "win")]
            if args.dbg.win && ctx.count.delta == 0 {
                // Collect debug info
                let info = dmg::dbg::ppu(&emu);
                talk.send(app::Message::Debug(app::msg::Debug::Vram(info)))?;
            }
            // Write trace entries.
            #[cfg(feature = "log")]
            if let Some(trace) = log.as_mut() {
                if matches!(
                    emu.main.soc.cpu.stage(),
                    cpu::Stage::Fetch | cpu::Stage::Done
                ) && ctx.count.delta % 4 == 0
                {
                    trace.log(&emu).context("could not write trace entry")?;
                }
            }
            // Reset timing stats
            let period = ctx.timer.elapsed();
            if ctx.timer.elapsed().as_secs() > 0 {
                // Calculate stats
                let stats = ctx.count.stats(period);
                debug!("{stats}");
                // Send stats
                talk.send(app::Message::Stats(stats))?;
                // Flush count, reset timer
                ctx.count.flush();
                ctx.timer = Instant::now();
            } else {
                // Clock another cycle
                ctx.count.tick();
            }
        }
    })(); // NOTE: This weird syntax is in lieu of using unstable try blocks.

    // Destroy emulator
    drop::emu(emu, &args.cfg.data).context("shutdown sequence failed")?;

    // Inspect error
    if let Some(err) = res
        .as_ref()
        .err()
        .and_then(|err| err.downcast_ref::<talk::Error>())
    {
        // Ignore disconnect
        debug!("{err}");
        res = Ok(());
    }
    // Propagate errors
    res.context("emulator error occurred")
}
