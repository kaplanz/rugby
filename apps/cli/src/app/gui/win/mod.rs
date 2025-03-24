//! Video frontend.

use rugby::core::dmg;

mod imp;

pub use self::imp::{Attributes, Extent, Window};

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
