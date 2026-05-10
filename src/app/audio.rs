//! Audio API.

use crate::emu::audio::Sample;

/// Audio interface.
pub trait Audio {
    /// Plays an audio sample.
    fn play(&mut self, sample: Sample);
}
