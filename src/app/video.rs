//! Video API.

use crate::emu::video::{Frame, Pixel};

/// Video interface.
pub trait Video {
    type Pixel: Pixel;

    /// Draws a video frame.
    fn draw(&mut self, frame: Frame<Self::Pixel>);
}
