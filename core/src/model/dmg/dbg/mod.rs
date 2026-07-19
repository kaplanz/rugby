//! Debugging the [DMG-01](super).

pub mod trace;

use super::GameBoy;
use super::soc::ppu;
use crate::rev::Revision;

/// Collect debug information from the PPU.
#[must_use]
pub fn ppu<R: Revision>(emu: &GameBoy<R>) -> ppu::dbg::Debug {
    ppu::dbg::info(&emu.main.soc.ppu)
}
