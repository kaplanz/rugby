//! Debugging the [DMG-01](super).

pub mod trace;

use super::{cpu, ppu, GameBoy};

/// Collect debug information from the PPU.
#[must_use]
pub fn ppu(emu: &GameBoy) -> ppu::dbg::Debug {
    ppu::dbg::info(&emu.main.soc.ppu)
}
