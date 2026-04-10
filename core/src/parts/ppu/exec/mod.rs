use ppu::{LCD, Lcdc, Ppu};
use rugby_arch::reg::Register;

use self::draw::Draw;
use self::hblank::HBlank;
use self::scan::Scan;
use self::vblank::VBlank;
use super::super::ppu;
use crate::dmg::pic::Interrupt;

pub mod draw;
pub mod hblank;
pub mod scan;
pub mod vblank;

/// Graphics mode.
#[derive(Clone, Debug)]
pub enum Mode {
    /// Mode 2: Scan OAM.
    Scan(Scan),
    /// Mode 3: Draw pixels.
    Draw(Draw),
    /// Mode 0: Horizontal blank.
    HBlank(HBlank),
    /// Mode 1: Vertical blank.
    VBlank(VBlank),
}

impl Default for Mode {
    fn default() -> Self {
        Self::HBlank(HBlank)
    }
}

impl Mode {
    /// Returns the internal mode value.
    #[must_use]
    #[rustfmt::skip]
    pub fn value(&self) -> u8 {
        match self {
            Mode::Scan(_)   => 0b10,
            Mode::Draw(_)   => 0b11,
            Mode::HBlank(_) => 0b00,
            Mode::VBlank(_) => 0b01,
        }
    }

    #[must_use]
    pub(super) fn exec(self, ppu: &mut Ppu) -> Self {
        // Execute state machine
        let next = match self {
            // Mode 2
            Mode::Scan(scan) => scan.exec(ppu),
            // Mode 3
            Mode::Draw(draw) => draw.exec(ppu),
            // Mode 0
            Mode::HBlank(hblank) => hblank.exec(ppu),
            // Mode 1
            Mode::VBlank(vblank) => vblank.exec(ppu),
        };

        // Compute STAT register
        let ly = ppu.reg.ly.load();
        let lyc = ppu.reg.lyc.load();
        let stat = {
            let mut stat = ppu.reg.stat.load();
            // Mode bits [1:0]
            stat ^= (0x03 & stat) ^ next.value();
            // LYC=LY bit [2]
            stat ^= (0x04 & stat) ^ (u8::from(ly == lyc) << 2);
            stat
        };
        // Update STAT register
        ppu.reg.stat.store(stat);

        // Compute STAT interrupt
        let int = {
            // Mode bits [5:3]
            #[rustfmt::skip]
            let mode = match next {
                // Mode 0
                Mode::HBlank(_) => 1 << 3,
                // Mode 1
                Mode::VBlank(_) => 1 << 4,
                // Mode 3
                Mode::Draw(_)   => 0,
                // Mode 2
                Mode::Scan(_)   => 1 << 5,
            };
            // LYC=LY bit [6]
            let line = u8::from(lyc == ly) << 6;

            // Combine activation sources
            ((mode | line) & stat) != 0
        };
        // Trigger STAT interrupt
        if int && !ppu.etc.int {
            // Only trigger on rising edge
            ppu.int.raise(Interrupt::LcdStat);
        }
        // Update STAT interrupt
        ppu.etc.int = int;

        // Increment dot count
        ppu.etc.dot += 1;
        ppu.etc.dot %= HBlank::DOTS;

        // Return next state
        next
    }
}
