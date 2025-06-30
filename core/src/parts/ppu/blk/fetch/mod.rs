use rugby_arch::reg::Register;

use super::fifo::Fifo;
use super::meta::{self, Layer};
use super::{Lcdc, Ppu};

mod bgw;
mod obj;

pub use self::bgw::Fetcher as Background;
pub use self::obj::Fetcher as Sprite;

/// Fetcher step.
#[derive(Clone, Debug, Default)]
pub enum Step {
    /// Fetch tile number.
    #[default]
    Fetch,
    /// Read tile data (low).
    Read0 { tdat: u16 },
    /// Read tile data (high).
    Read1 { tdat: u16, data: u8 },
    /// Push pixels to FIFO.
    Push { data: [u8; 2] },
}

impl Ppu {
    /// Gets the tile address base for the configured addressing mode.
    #[inline]
    pub(crate) fn base(&self, layer: Layer) -> u16 {
        match layer {
            Layer::Background | Layer::Window => {
                if self.lcdc(Lcdc::BgWinData) {
                    0x0000
                } else {
                    0x1000
                }
            }
            Layer::Sprite => 0x0000,
        }
    }

    /// Calculates a tile index for the configured addressing mode.
    #[inline]
    pub(crate) fn tidx(&self, layer: Layer, tnum: u8) -> u16 {
        let base = self.base(layer);
        let tnum = match layer {
            Layer::Background | Layer::Window => {
                if self.lcdc(Lcdc::BgWinData) {
                    u16::from(tnum)
                } else {
                    tnum as i8 as u16
                }
            }
            Layer::Sprite => u16::from(tnum),
        };
        base.wrapping_add(tnum << 4)
    }

    /// Calculates the tile offset for a given layer.
    #[inline]
    pub(crate) fn toff(&self, layer: Layer, yoff: u8) -> u16 {
        u16::from(match layer {
            Layer::Background => self.reg.ly.load().wrapping_add(yoff),
            Layer::Window => self.etc.ywin,
            Layer::Sprite => self.reg.ly.load().wrapping_sub(yoff),
        }) % 8
    }

    /// Calculates the tile data address from a tile number using the configured
    /// addressing mode.
    #[inline]
    pub(crate) fn tdat(&self, layer: Layer, tnum: u8, yoff: u8) -> u16 {
        let tidx = self.tidx(layer, tnum);
        let toff = self.toff(layer, yoff) * 2;
        tidx | toff
    }
}
