//! Component support.

use crate::api::part;

/// Audio support.
pub trait Audio {
    /// Audio interface.
    type Audio: part::audio::Audio;

    /// Borrows the core's audio.
    #[must_use]
    fn audio(&self) -> &Self::Audio;

    /// Mutably borrows the core's audio.
    #[must_use]
    fn audio_mut(&mut self) -> &mut Self::Audio;
}

/// Input support.
pub trait Input {
    /// Input interface.
    type Input: part::input::Input;

    /// Borrows the core's input.
    #[must_use]
    fn input(&self) -> &Self::Input;

    /// Mutably borrows the core's input.
    #[must_use]
    fn input_mut(&mut self) -> &mut Self::Input;
}

/// Processor support.
pub trait Processor {
    /// Compute interface.
    type Proc: part::proc::Processor;

    /// Borrows the core's processor.
    #[must_use]
    fn proc(&self) -> &Self::Proc;

    /// Mutably borrows the core's processor.
    #[must_use]
    fn proc_mut(&mut self) -> &mut Self::Proc;
}

/// Cable support.
pub trait Cable {
    /// Cable interface.
    type Cable: part::cable::Cable;

    /// Borrows the core's cable.
    #[must_use]
    fn cable(&self) -> &Self::Cable;

    /// Mutably borrows the core's cable.
    #[must_use]
    fn cable_mut(&mut self) -> &mut Self::Cable;
}

/// Video support.
pub trait Video {
    /// Video interface.
    type Video: part::video::Video;

    /// Borrows the core's video.
    #[must_use]
    fn video(&self) -> &Self::Video;

    /// Mutably borrows the core's video.
    #[must_use]
    fn video_mut(&mut self) -> &mut Self::Video;
}
