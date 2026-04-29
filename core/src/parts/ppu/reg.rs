//! Graphics registers.

#![expect(clippy::doc_markdown)]

use bitfield_struct::bitfield;
use rugby_arch::mem::Memory;
use rugby_arch::reg::Register;

/// `LCDC`: LCD control register.
///
/// | Bit | Name                        |
/// |-----|-----------------------------|
/// |   7 | LCD enable.                 |
/// |   6 | Window tile map area.       |
/// |   5 | Window enable.              |
/// |   4 | BG-Window tile data area.   |
/// |   3 | BG tile map area.           |
/// |   2 | OBJ size.                   |
/// |   1 | OBJ enable.                 |
/// |   0 | BG-Window enable.           |
///
/// See more details [here][lcdc].
///
/// [lcdc]: https://gbdev.io/pandocs/LCDC.html
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
pub struct Lcdc {
    /// `LCDC[7]`: LCD enable.
    #[bits(1)]
    pub enable: bool,
    /// `LCDC[6]`: Window tile map area.
    #[bits(1)]
    pub win_map: bool,
    /// `LCDC[5]`: Window enable.
    #[bits(1)]
    pub win_enable: bool,
    /// `LCDC[4]`: BG-Window tile data area.
    #[bits(1)]
    pub bg_win_data: bool,
    /// `LCDC[3]`: BG tile map area.
    #[bits(1)]
    pub bg_map: bool,
    /// `LCDC[2]`: OBJ size.
    #[bits(1)]
    pub obj_size: bool,
    /// `LCDC[1]`: OBJ enable.
    #[bits(1)]
    pub obj_enable: bool,
    /// `LCDC[0]`: BG-Window enable.
    #[bits(1)]
    pub bg_win_enable: bool,
}

impl Memory for Lcdc {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Lcdc {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.into_bits()
    }

    fn store(&mut self, value: Self::Value) {
        *self = Self::from_bits(value);
    }
}

/// `STAT`: LCD status register.
///
/// | Bit | Name                           | Use |
/// |-----|--------------------------------|-----|
/// |   6 | STAT interrupt source (LYC=LY) | R/W |
/// |   5 | OAM interrupt source           | R/W |
/// |   4 | VBlank interrupt source        | R/W |
/// |   3 | HBlank interrupt source        | R/W |
/// |   2 | LYC = LY compare flag          | R   |
/// | 1-0 | Mode flag (2-bit)              | R   |
///
/// See more details [here][stat].
///
/// [stat]: https://gbdev.io/pandocs/STAT.html#ff41--stat-lcd-status
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
pub struct Stat {
    /// `STAT[7]`: Unused.
    #[bits(1)]
    __: u8,
    /// `STAT[6]`: LYC=LY interrupt source.
    #[bits(1)]
    pub lyc_int: bool,
    /// `STAT[5]`: OAM interrupt source.
    #[bits(1)]
    pub oam_int: bool,
    /// `STAT[4]`: VBlank interrupt source.
    #[bits(1)]
    pub vblank_int: bool,
    /// `STAT[3]`: HBlank interrupt source.
    #[bits(1)]
    pub hblank_int: bool,
    /// `STAT[2]`: LYC=LY compare flag.
    #[bits(1)]
    pub lyc: bool,
    /// `STAT[1:0]`: Mode flag.
    #[bits(2)]
    pub mode: u8,
}

impl Memory for Stat {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Stat {
    type Value = u8;

    fn load(&self) -> Self::Value {
        // unused bit 7 always reads as 1
        self.into_bits() | 0x80
    }

    fn store(&mut self, value: Self::Value) {
        *self = Self::from_bits(value);
    }
}

/// Palette data register.
///
/// Each 2-bit field selects the colour to display for the corresponding
/// colour index.
///
/// | Bits | Index |
/// |------|-------|
/// | 7:6  | 3     |
/// | 5:4  | 2     |
/// | 3:2  | 1     |
/// | 1:0  | 0     |
///
/// See more details [here][pal].
///
/// [pal]: https://gbdev.io/pandocs/Palettes.html
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
pub struct Pal {
    /// `PAL[7:6]`: Colour index 3.
    #[bits(2)]
    pub c3: u8,
    /// `PAL[5:4]`: Colour index 2.
    #[bits(2)]
    pub c2: u8,
    /// `PAL[3:2]`: Colour index 1.
    #[bits(2)]
    pub c1: u8,
    /// `PAL[1:0]`: Colour index 0.
    #[bits(2)]
    pub c0: u8,
}

impl Memory for Pal {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Pal {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.into_bits()
    }

    fn store(&mut self, value: Self::Value) {
        *self = Self::from_bits(value);
    }
}

/// `SCY`: Viewport Y position.
pub type Scy = u8;

/// `SCX`: Viewport X position.
pub type Scx = u8;

/// `LY`: LCD Y coordinate.
pub type Ly = u8;

/// `LYC`: LY compare.
pub type Lyc = u8;

/// `WY`: Window Y position.
pub type Wy = u8;

/// `WX`: Window X position.
pub type Wx = u8;
