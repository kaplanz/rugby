//! Embedded memory.

use rugby_arch::Shared;
use rugby_arch::mem::Ram;

pub use super::apu::Wave;
pub use super::cpu::{Hram, Wram};
pub use super::ppu::{Oam, Vram};

/// Sharp LH5164N (64K SRAM).
pub type Sram = Ram<[u8; 0x2000]>;

/// Memory bank.
///
/// |     Address     |  Size  | Name | Description   |
/// |:---------------:|--------|------|---------------|
/// | `$8000..=$9FFF` |  8 KiB | VRAM | Video RAM     |
/// | `$C000..=$DFFF` |  8 KiB | WRAM | Work RAM      |
/// | `$FE00..=$FEA0` |  160 B | OAM  | Object memory |
/// | `$FF30..=$FF3F` |   16 B | WAVE | Wave RAM      |
/// | `$FF80..=$FFFE` |  127 B | HRAM | High RAM      |
#[derive(Clone, Debug)]
pub struct Bank {
    /// Video RAM.
    pub vram: Shared<Vram>,
    /// Work RAM.
    pub wram: Shared<Wram>,
    /// Object memory.
    pub oam: Shared<Oam>,
    /// Wave memory.
    pub wave: Shared<Wave>,
    /// High RAM.
    pub hram: Shared<Hram>,
}

#[rustfmt::skip]
impl Default for Bank {
    fn default() -> Self {
        Self {
            vram: Shared::new(Vram::from([u8::default(); 0x2000])),
            wram: Shared::new(Wram::from([u8::default(); 0x2000])),
            oam:  Shared::new( Oam::from([u8::default(); 0x00a0])),
            wave: Shared::new(Wave::from([u8::default(); 0x0010])),
            hram: Shared::new(Hram::from([u8::default(); 0x007f])),
        }
    }
}

impl Bank {
    /// Constructs a new `Bank`.
    #[must_use]
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self::default()
    }
}
