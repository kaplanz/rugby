//! Playback thread.

use std::thread;
use std::time::Duration;

use anyhow::{Result, anyhow};
use log::debug;
use rugby::core::dmg;
use rugby::emu::part::audio::Sample;

use crate::app;
use crate::exe::run::Cli;

mod buf;

pub use self::buf::Stream;

/// Audio channel count.
pub const CHANNELS: usize = 2;

/// Audio latency maximum (in milliseconds).
pub const LATENCY: usize = 100;

/// Audio main.
pub fn main(args: &Cli) -> Result<()> {
    // No-op if muted
    if args.feat.mute {
        debug!("playback disabled");
        return Ok(());
    }

    // Define sample rates
    let ifrq = args
        .cfg
        .data
        .app
        .spd
        .clone()
        .unwrap_or_default()
        .freq()
        .unwrap_or(dmg::FREQ);
    let ofrq = args.cfg.data.app.aux;

    // Initialize audio system
    app::data::audio::init(ifrq, ofrq);

    // Define output device parameters
    let params = tinyaudio::OutputDeviceParameters {
        channels_count: CHANNELS,
        sample_rate: ofrq as usize,
        channel_sample_count: ofrq as usize * LATENCY / 1000,
    };

    // Run output device loop with proper error handling
    let mut audio = tinyaudio::run_output_device(params, move |data| {
        // Retain previous sample if needed
        let mut sample = Sample::default();

        // Play received samples
        for samples in data.chunks_mut(params.channels_count) {
            // Receive next sample
            //
            // If the audio buffer can't produce a next sample, we instead reuse
            // the previous sample, preventing audio "pop."
            sample = app::data::audio::pull().unwrap_or(sample);

            // Update the sample value for each output channel
            //
            // Audio channels
            let [lt, rt] = samples else {
                panic!("incorrect number of audio channels")
            };
            *lt = sample.lt;
            *rt = sample.rt;
        }
    })
    .map_err(|err| anyhow!("{err}"))?;

    // Playback loop
    //
    // This exists just to keep the audio alive, and will exit with the
    // application.
    while app::running() {
        // No actual work is done here. Instead, we simply sleep to yield the
        // thread's cycles to more useful work.
        thread::sleep(Duration::from_millis(10));
    }

    // Stop audio
    audio.close();

    Ok(())
}
