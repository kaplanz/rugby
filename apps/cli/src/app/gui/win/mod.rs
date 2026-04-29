//! Video frontend.

use rugby::core::chip::ppu;

mod imp;

pub use self::imp::{Attributes, Extent, Window};

/// Main window.
#[derive(Debug)]
pub struct Main;

impl Attributes for Main {
    const NAME: &str = crate::NAME;

    const SIZE: Extent = Extent {
        wd: ppu::LCD.wd as usize,
        ht: ppu::LCD.ht as usize,
    };
}
