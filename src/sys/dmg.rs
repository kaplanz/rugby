//! Game Boy (DMG) revisions.

use crate::arch::Block;
use crate::core::cart::Cartridge;
use crate::core::dmg::{GameBoy, rev};
#[cfg(feature = "cfg")]
use crate::extra::cfg::types::model::dmg::Rev;

/// Game Boy (DMG) revision.
#[derive(Debug)]
#[non_exhaustive]
pub enum Dmg {
    /// DMG-CPU 0.
    Zero(GameBoy<rev::Zero>),
    /// DMG-CPU A.
    A(GameBoy<rev::A>),
    /// DMG-CPU B.
    B(GameBoy<rev::B>),
    /// DMG-CPU C.
    C(GameBoy<rev::C>),
}

impl Default for Dmg {
    fn default() -> Self {
        Self::C(GameBoy::default())
    }
}

#[cfg(feature = "cfg")]
impl From<Rev> for Dmg {
    fn from(rev: Rev) -> Self {
        match rev {
            Rev::Zero => Self::Zero(GameBoy::new()),
            Rev::A => Self::A(GameBoy::new()),
            Rev::B => Self::B(GameBoy::new()),
            Rev::C => Self::C(GameBoy::new()),
        }
    }
}

impl Block for Dmg {
    fn ready(&self) -> bool {
        match self {
            Self::Zero(gb) => gb.ready(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.ready(),
        }
    }

    fn cycle(&mut self) {
        match self {
            Self::Zero(gb) => gb.cycle(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.cycle(),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::Zero(gb) => gb.reset(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.reset(),
        }
    }
}

impl Dmg {
    /// Gets the inserted cartridge, if any.
    #[must_use]
    pub fn cart(&self) -> Option<&Cartridge> {
        match self {
            Self::Zero(gb) => gb.cart(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.cart(),
        }
    }

    /// Inserts a cartridge.
    pub fn insert(&mut self, cart: Cartridge) {
        match self {
            Self::Zero(gb) => gb.insert(cart),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.insert(cart),
        }
    }

    /// Ejects the inserted cartridge, if any.
    #[must_use]
    pub fn eject(&mut self) -> Option<Cartridge> {
        match self {
            Self::Zero(gb) => gb.eject(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.eject(),
        }
    }
}
