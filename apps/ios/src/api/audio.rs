//! Audio API.

use rugby::emu::part::audio::{Chiptune, Sample};
use rugby::prelude::*;

use super::GameBoy;

#[uniffi::export]
impl GameBoy {
    /// Samples audio output.
    ///
    /// Returns an audio sample with internal channels already mixed, ready for
    /// emulation.
    #[uniffi::method]
    pub fn sample(&self) -> Sample {
        self.inner.read().inside().audio().sample().mix()
    }
}

/// Rich audio sample.
///
/// See [`rugby::emu::part::audio::Chiptune`]
#[uniffi::remote(Record)]
pub struct Chiptune {
    /// Master volume.
    ///
    /// Generally used as a multiplier across all channels.
    pub vol: Sample,
    /// Channel 1 output.
    pub ch1: Sample,
    /// Channel 2 output.
    pub ch2: Sample,
    /// Channel 3 output.
    pub ch3: Sample,
    /// Channel 4 output.
    pub ch4: Sample,
}

/// Audio sample.
///
/// See [`rugby::emu::part::audio::Sample`]
#[uniffi::remote(Record)]
pub struct Sample {
    /// Left channel.
    pub lt: f32,
    /// Right channel.
    pub rt: f32,
}
