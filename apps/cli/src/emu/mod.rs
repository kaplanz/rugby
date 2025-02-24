//! Emulator thread.

use std::mem;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::{Context as _, Result};
use log::{debug, trace};
use ringbuf::LocalRb as Ring;
use ringbuf::traits::{Consumer, RingBuffer};
use rugby::arch::Block;
#[cfg(feature = "win")]
use rugby::core::dmg;
use rugby::core::dmg::FREQ;
#[cfg(feature = "log")]
use rugby::core::dmg::cpu;
use rugby::emu::part::audio::{Audio, Sample};
use rugby::emu::part::joypad::Joypad;
use rugby::emu::part::video::Video;
use rugby::prelude::Core;
use rugby_cfg::opt::app::Speed;

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

/// Audio sample rate.
pub const SAMPLE: usize = 96_000;

/// Audio buffer length.
pub const BUFLEN: usize = 0x1000;

// Emulator main.
#[expect(clippy::too_many_lines)]
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

    // Audio loop
    let (mut audio, wave) = {
        // Define output device parameters
        let params = tinyaudio::OutputDeviceParameters {
            channels_count: 2,
            sample_rate: SAMPLE,
            channel_sample_count: BUFLEN,
        };

        // Define audio wave source and sink
        let wave = Arc::new(Mutex::new(Ring::new(BUFLEN)));
        let sink = wave.clone();

        // Run output device loop
        let audio = tinyaudio::run_output_device(params, move |data| {
            // Store previous (stale) sample
            let mut last = Sample::default();

            // Play received samples
            for samples in data.chunks_mut(params.channels_count) {
                // Receive next sample from the audio buffer
                let sample: Sample = sink.lock().unwrap().try_pop().unwrap_or(last);
                // Update the sample value for each output channel
                let [lt, rt] = samples else {
                    panic!("incorrect number of audio channels")
                };
                *lt = sample.lt;
                *rt = sample.rt;
                // Slowly decay stale sample
                //
                // This will prevent pops as the volume is adjusted while
                // paused, as samples which would otherwise be reused could have
                // a DC offset. By tending them towards zero, that offset is no
                // longer multiplied as volume is adjusted.
                last = sample * 0.99;
            }
        })
        .unwrap();

        (audio, wave)
    };
    // Disable audio when headless
    if args.feat.headless {
        audio.close();
    }

    // Emulator loop
    let mut res = (|| -> Result<()> {
        // Initialize context
        let mut ctx = Context::new(&args.cfg.data);
        let mut apu = Vec::<Sample>::new();
        let freq = args
            .cfg
            .data
            .app
            .spd
            .as_ref()
            .and_then(Speed::freq)
            .unwrap_or(FREQ) as usize;
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
            // Sample audio output
            #[expect(irrefutable_let_patterns)]
            if let audio = emu.inside().audio() {
                // Fetch next sample
                apu.push(audio.sample().mix());
                // Filter to lower sample rate
                if apu.len() >= freq / SAMPLE {
                    // Filter collected samples
                    let len = apu.len();
                    // Average using mean
                    let sum = apu.drain(..).sum::<Sample>();
                    #[expect(clippy::cast_precision_loss)]
                    let avg = sum / len as f32;
                    // Send sample to device
                    wave.lock().unwrap().push_overwrite(avg);
                }
            }
            // Send video frame
            let video = emu.inside().video();
            if video.vsync() && {
                // To prevent overwhelming the frontend, we disable the video
                // before we send a frame. It will be re-enabled after receipt
                // is acknowledged.
                mem::take(&mut ctx.video)
            } {
                // Collect and send completed frame
                let frame = video.frame().to_owned().into_boxed_slice();
                talk.send(app::Message::Video(frame))?;
            }
            // Send VRAM debug info
            #[cfg(feature = "win")]
            if args.dbg.win && video.vsync() {
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

    // Close audio device
    audio.close();

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
