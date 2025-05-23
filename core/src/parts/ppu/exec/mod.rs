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
        // Update status register
        let ly = ppu.reg.ly.load();
        let lyc = ppu.reg.lyc.load();
        let stat = {
            let mut stat = ppu.reg.stat.load();
            stat ^= (0x03 & stat) ^ self.value();
            stat ^= (0x04 & stat) ^ (u8::from(ly == lyc) << 2);
            stat
        };
        ppu.reg.stat.store(stat);

        // Trigger interrupts
        if ppu.etc.dot == 0 {
            let mut int = 0;
            // LYC=LY
            int |= u8::from(lyc == ly) << 6;
            // Mode 2
            int |= u8::from(matches!(self, Mode::Scan(_))) << 5;
            // Mode 1
            int |= u8::from(matches!(self, Mode::VBlank(_))) << 4;
            // Mode 0
            int |= u8::from(matches!(self, Mode::HBlank(_))) << 3;
            // Check for interrupts
            if int & (stat & 0x78) != 0 {
                // Request an interrupt
                ppu.int.raise(Interrupt::LcdStat);
            }
        }

        // Execute the current mode
        let next = match self {
            Mode::Scan(scan) => scan.exec(ppu),
            Mode::Draw(draw) => draw.exec(ppu),
            Mode::HBlank(hblank) => hblank.exec(ppu),
            Mode::VBlank(vblank) => vblank.exec(ppu),
        };

        // Increment dot count
        ppu.etc.dot += 1;
        ppu.etc.dot %= HBlank::DOTS;

        // Return next state
        next
    }
}
