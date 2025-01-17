//! Video API.

use crate::emu::part::video::{Frame, Pixel};

/// Video interface.
pub trait Video {
    type Pixel: Pixel;

    /// Draws the current video frame.
    ///
    /// The video output is updated using the framebuffer provided by the
    /// emulator.
    fn draw(&mut self, frame: Frame<Self::Pixel>);
}
