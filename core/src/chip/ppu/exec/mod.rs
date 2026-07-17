use ppu::{LCD, Ppu};
use rugby_arch::reg::Register;

use self::draw::Draw;
use self::hblank::HBlank;
use self::scan::Scan;
use self::vblank::VBlank;
use super::super::ppu;
use crate::chip::irq::Interrupt;

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
        // Check for vertical blank entry
        let entry = !matches!(&self, Mode::VBlank(_));
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

        // Update STAT register
        let ly = ppu.reg.ly.load();
        let lyc = ppu.reg.lyc.load();
        ppu.reg.stat.borrow_mut().set_mode(next.value());
        ppu.reg.stat.borrow_mut().set_lyc(ly == lyc);

        // Latch window trigger
        ppu.etc.ytrg |= ppu.reg.lcdc.borrow().win_enable() && ly == ppu.reg.wy.load();

        // Compute STAT interrupt
        let stat = *ppu.reg.stat.borrow();
        #[rustfmt::skip]
        let int = (match next {
            // Mode 0
            Mode::HBlank(_) => stat.hblank_int(),
            // Mode 1
            //
            // NOTE: Entering the vertical blank also raises the OAM source.
            //       This is not documented by Pan Docs, but is verified on
            //       hardware by mooneye's `vblank_stat_intr` test.
            Mode::VBlank(_) => stat.vblank_int() || (entry && stat.oam_int()),
            // Mode 3
            Mode::Draw(_)   => false,
            // Mode 2
            Mode::Scan(_)   => stat.oam_int(),
        }) || (ly == lyc && stat.lyc_int());
        // Trigger STAT interrupt
        if int && !ppu.etc.int {
            // Only trigger on rising edge
            ppu.irq.raise(Interrupt::LcdStat);
        }
        // Update STAT interrupt
        ppu.etc.int = int;

        // Increment dot count
        ppu.etc.dot += 1;
        ppu.etc.dot %= HBlank::DOTS;

        // Transition state machine
        next
    }
}
