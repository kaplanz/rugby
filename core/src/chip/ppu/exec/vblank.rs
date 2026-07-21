use log::debug;
use rugby_arch::reg::Register;

use super::hblank::HBlank;
use super::{LCD, Mode, Ppu};

/// Mode 1: Vertical blank.
#[derive(Clone, Debug, Default)]
pub struct VBlank;

impl VBlank {
    /// Number of scanlines of vblank.
    pub const LAST: u16 = LCD.ht + 10;

    pub fn exec(self, ppu: &mut Ppu) -> Mode {
        // Handle line 153 quirk
        //
        // `LY` reads 153 for only its first 4 dots, then 0 until the
        // frame ends.
        if u16::from(ppu.etc.line) == Self::LAST - 1 && ppu.etc.dot >= 4 {
            ppu.reg.ly.store(0);
        }

        // Transition state machine
        if ppu.etc.dot + 1 < HBlank::DOTS {
            // Continue vblank
            Mode::VBlank(self)
        } else {
            // Increment scanline
            let line = ppu.etc.line + 1;

            // Determine scanline type
            if u16::from(line) < Self::LAST {
                // Update scanline
                ppu.reg.ly.store(line);
                ppu.etc.line = line;
                // Continue vblank
                Mode::VBlank(self)
            } else {
                // Reset scanline
                ppu.reg.ly.store(0);
                ppu.etc.line = 0;
                // Enter scan
                debug!("entered mode 2: scan OAM");
                Mode::Scan(self.into())
            }
        }
    }
}

impl From<HBlank> for VBlank {
    fn from(HBlank { .. }: HBlank) -> Self {
        Self
    }
}
