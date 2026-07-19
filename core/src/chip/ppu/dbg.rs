//! Debugging the [PPU](super).

use itertools::Itertools;
use rugby_arch::mem::Memory;

use super::meta::{Layer, Meta, Pixel, Tile};
use super::{Color, Ppu};

/// Collects debug information.
#[must_use]
pub fn info(ppu: &Ppu) -> Debug {
    Debug::new(ppu)
}

/// Debug information.
#[derive(Debug)]
pub struct Debug {
    /// Tile data.
    ///
    /// Rendering of the VRAM's tile data (`$8000..=$97FF`).
    pub tdat: Box<[Color; 0x06000]>,
    /// Tile map 1.
    ///
    /// Rendering of the VRAM's tile map 1 (`$9800..=$9BFF`).
    pub map1: Box<[Color; 0x10000]>,
    /// Tile map 2.
    ///
    /// Rendering of the VRAM's tile map 1 (`$9C00..=$9FFF`).
    pub map2: Box<[Color; 0x10000]>,
}

impl Debug {
    /// Constructs a new `Debug`.
    fn new(ppu: &Ppu) -> Self {
        // Extract scanline info
        let layer = Layer::Background;

        // Extract tile data, maps
        let tdat: [_; 0x1800] = std::array::from_fn(|addr| u16::try_from(addr).unwrap())
            .map(|addr| ppu.mem.vram.read(addr).unwrap());
        let tdat: [_; 0x180] = <[_; 0x180]>::try_from(tdat.as_chunks().0) // 16 bytes per tile
            .unwrap()
            .map(Tile::from);
        let map1: [_; 0x400] = std::array::from_fn(|idx| u16::try_from(0x1800 + idx).unwrap())
            .map(|addr| ppu.mem.vram.read(addr).unwrap())
            .map(|tnum| tdat[usize::from(Ppu::tidx(ppu, layer, tnum) >> 4)].clone());
        let map2: [_; 0x400] = std::array::from_fn(|idx| u16::try_from(0x1c00 + idx).unwrap())
            .map(|addr| ppu.mem.vram.read(addr).unwrap())
            .map(|tnum| tdat[usize::from(Ppu::tidx(ppu, layer, tnum) >> 4)].clone());

        // Render tile data, maps
        let meta = Meta::Bgw;
        let tdat = Self::render(&tdat, ppu, &meta, 16); // 16x24 tiles
        let map1 = Self::render(&map1, ppu, &meta, 32); // 32x32 tiles
        let map2 = Self::render(&map2, ppu, &meta, 32); // 32x32 tiles

        // Return debug info
        Self { tdat, map1, map2 }
    }

    /// Renders tile data as pixels.
    #[expect(clippy::unnecessary_box_returns)]
    fn render<const N: usize>(
        tdat: &[Tile],
        ppu: &Ppu,
        meta: &Meta,
        width: usize,
    ) -> Box<[Color; N]> {
        tdat.chunks_exact(width) // tiles per row
            .flat_map(|row| {
                row.iter()
                    .flat_map(|tile| tile.clone().into_iter().enumerate())
                    .sorted_by_key(|row| row.0)
                    .map(|(_, row)| row)
                    .collect_vec()
            })
            .flat_map(|row| {
                row.into_iter()
                    .map(|col| ppu.color(&Pixel::new(col, meta.clone())))
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }
}
