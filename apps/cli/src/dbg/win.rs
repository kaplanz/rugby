//! Debug windows.

use anyhow::{Context, Result};
use rugby::core::dmg::ppu;

use crate::gui::Frontend;
use crate::gui::win::{Attributes, Extent, Window};

impl Frontend {
    /// Render debug windows.
    #[expect(clippy::needless_pass_by_value)]
    pub fn vram(&mut self, data: ppu::dbg::Debug) -> Result<()> {
        // Extract GUI
        let Some(gui) = self.win.as_mut() else {
            return Ok(());
        };
        // Extract PPU state
        let recolor = |col: ppu::Color| self.cfg.pal[col as usize].into();
        let tdat = data.tdat.into_iter().map(recolor).collect::<Box<_>>();
        let map1 = data.map1.into_iter().map(recolor).collect::<Box<_>>();
        let map2 = data.map2.into_iter().map(recolor).collect::<Box<_>>();
        // Display PPU state
        gui.dbg
            .tile
            .as_mut()
            .map(|win| win.redraw(&tdat))
            .transpose()
            .context("error drawing tile data")?;
        gui.dbg
            .map1
            .as_mut()
            .map(|win| win.redraw(&map1))
            .transpose()
            .context("error drawing tile map 1")?;
        gui.dbg
            .map2
            .as_mut()
            .map(|win| win.redraw(&map2))
            .transpose()
            .context("error drawing tile map 2")?;
        Ok(())
    }
}

/// VRAM windows.
#[derive(Debug, Default)]
pub struct Vram {
    /// Tile data.
    pub tile: Option<Window<Tile>>,
    /// Tile map 1.
    pub map1: Option<Window<Map1>>,
    /// Tile map 2.
    pub map2: Option<Window<Map2>>,
}

#[expect(unused)]
impl Vram {
    /// Constructs a new `Debug`.
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this.open().map(|()| this)
    }

    /// Opens all debug windows.
    pub fn open(&mut self) -> Result<()> {
        if self.tile.is_none() {
            self.tile = Window::<Tile>::open().map(Some)?;
        }
        if self.map1.is_none() {
            self.map1 = Window::<Map1>::open().map(Some)?;
        }
        if self.map2.is_none() {
            self.map2 = Window::<Map2>::open().map(Some)?;
        }
        Ok(())
    }
}

/// Tile data window.
#[derive(Debug)]
pub struct Tile;

impl Attributes for Tile {
    const NAME: &str = "Tile Data";

    const SIZE: Extent = Extent {
        wd: 16 * 8,
        ht: 24 * 8,
    };
}

/// Tile map 1 window.
#[derive(Debug)]
pub struct Map1;

impl Attributes for Map1 {
    const NAME: &str = "Tile Map 1";

    const SIZE: Extent = Extent {
        wd: 32 * 8,
        ht: 24 * 8,
    };
}

/// Tile map 2 window.
#[derive(Debug)]
pub struct Map2;

impl Attributes for Map2 {
    const NAME: &str = "Tile Map 2";

    const SIZE: Extent = Extent {
        wd: 32 * 8,
        ht: 24 * 8,
    };
}
