//! Video frontend.

use anyhow::Result;
use rugby::core::dmg;

mod imp;

pub use self::imp::{Attributes, Extent, Window};

/// Video windows.
#[derive(Debug)]
pub struct Video {
    /// Main window.
    pub lcd: Window<Main>,
    /// VRAM window group.
    #[cfg(feature = "win")]
    pub dbg: crate::dbg::win::Vram,
}

impl Video {
    /// Constructs a new `Graphics`.
    pub fn new() -> Result<Self> {
        Ok(Self {
            lcd: Window::open()?,
            #[cfg(feature = "win")]
            dbg: crate::dbg::win::Vram::default(),
        })
    }

    /// Checks if the frontend is alive.
    pub fn alive(&self) -> bool {
        self.lcd.is_open()
    }
}

/// Main window.
#[derive(Debug)]
pub struct Main;

impl Attributes for Main {
    const NAME: &str = crate::NAME;

    const SIZE: Extent = Extent {
        wd: dmg::LCD.wd as usize,
        ht: dmg::LCD.ht as usize,
    };
}
