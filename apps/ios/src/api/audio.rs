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
    pub fn sample(&self) -> u64 {
        // Encode sample with a `u64`
        //
        // By encoding the sample's data within a `u64`, we can avoid UniFFI
        // overhead. This should not be necessary, but it seems `Record` isn't
        // very optimized.
        //
        // I understand this implementation may be considered a crime in some
        // jurisdictions.
        let Sample { lt, rt } = self.inner.read().inside().audio().sample().mix();
        // Convert `f32`s to little-endian bytes
        let lt = lt.to_le_bytes();
        let rt = rt.to_le_bytes();
        // Encode bytes in little-endian `u64`
        u64::from_le_bytes([lt[0], lt[1], lt[2], lt[3], rt[0], rt[1], rt[2], rt[3]])
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
