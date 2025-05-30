//! Video API.

use rugby::prelude::*;

use super::GameBoy;

#[uniffi::export]
impl GameBoy {
    /// Checks if video output is ready.
    ///
    /// Assuming the PPU is enabled (by the cartridge), this should be true
    /// exactly once every 70,224 cycles.
    pub fn vsync(&self) -> bool {
        self.emu.read().unwrap().inside().video().vsync()
    }

    /// Gets the current frame state.
    ///
    /// If this is called any time [`Self::vsync`] returns `false`, the result
    /// may be an unfinished frame.
    ///
    /// # Format
    ///
    /// Each frame is represented as a flattened array of 23,040 bytes, each
    /// with values ranging from 0 to 3. These values each represent the 2-bit
    /// color of its corresponding pixel.
    ///
    /// To render a frame as an image, map each pixel to the 160x144 screen. For
    /// example, to get the 20th pixel of the 16th line, you would access the
    /// `(20 * 160) + 16  = 3216`th element of the frame (zero indexed) as
    /// `frame[3215]`.
    pub fn frame(&self) -> Vec<u8> {
        self.emu
            .read()
            .unwrap()
            .inside()
            .video()
            .frame()
            .iter()
            .map(|&pix| pix as u8)
            .collect::<Vec<u8>>()
    }
}
