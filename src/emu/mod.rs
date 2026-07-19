//! Game Boy models.

use std::io::{BufRead, Write};

use crate::arch::Block;
#[cfg(feature = "cfg")]
use crate::cfg::types::model::dmg::Rev;
use crate::core::api::audio::{Audio, Chiptune};
use crate::core::api::cable::Cable;
use crate::core::api::input::{Event, Input};
use crate::core::api::video::{Aspect, Video};
use crate::core::cart::Cartridge;
use crate::core::dmg::{self, rev};

/// Game Boy handheld game console.
#[derive(Debug)]
#[non_exhaustive]
pub enum GameBoy {
    /// DMG-CPU 0.
    Dmg0(dmg::GameBoy<rev::Zero>),
    /// DMG-CPU A.
    DmgA(dmg::GameBoy<rev::A>),
    /// DMG-CPU B.
    DmgB(dmg::GameBoy<rev::B>),
    /// DMG-CPU C.
    DmgC(dmg::GameBoy<rev::C>),
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::DmgC(dmg::GameBoy::default())
    }
}

#[cfg(feature = "cfg")]
impl From<Rev> for GameBoy {
    fn from(rev: Rev) -> Self {
        match rev {
            Rev::Zero => Self::Dmg0(dmg::GameBoy::new()),
            Rev::A => Self::DmgA(dmg::GameBoy::new()),
            Rev::B => Self::DmgB(dmg::GameBoy::new()),
            Rev::C => Self::DmgC(dmg::GameBoy::new()),
        }
    }
}

impl Block for GameBoy {
    fn ready(&self) -> bool {
        match self {
            Self::Dmg0(dmg) => dmg.ready(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.ready(),
        }
    }

    fn cycle(&mut self) {
        match self {
            Self::Dmg0(dmg) => dmg.cycle(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.cycle(),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::Dmg0(dmg) => dmg.reset(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.reset(),
        }
    }
}

impl GameBoy {
    /// Gets the inserted cartridge, if any.
    #[must_use]
    pub fn cart(&self) -> Option<Cartridge> {
        match self {
            Self::Dmg0(dmg) => dmg.cart(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.cart(),
        }
    }

    /// Inserts a cartridge.
    pub fn insert(&mut self, cart: Cartridge) {
        match self {
            Self::Dmg0(dmg) => dmg.insert(cart),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.insert(cart),
        }
    }

    /// Ejects the inserted cartridge, if any.
    #[must_use]
    pub fn eject(&mut self) -> Option<Cartridge> {
        match self {
            Self::Dmg0(dmg) => dmg.eject(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.eject(),
        }
    }
}

impl Audio for GameBoy {
    fn sample(&self) -> Chiptune {
        match self {
            Self::Dmg0(dmg) => dmg.sample(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.sample(),
        }
    }
}

impl Cable for GameBoy {
    fn rx(&mut self) -> &mut dyn BufRead {
        match self {
            Self::Dmg0(dmg) => dmg.rx(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.rx(),
        }
    }

    fn tx(&mut self) -> &mut dyn Write {
        match self {
            Self::Dmg0(dmg) => dmg.tx(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.tx(),
        }
    }
}

impl Input for GameBoy {
    type Button = <dmg::GameBoy<rev::C> as Input>::Button;

    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>) {
        match self {
            Self::Dmg0(dmg) => dmg.recv(events),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.recv(events),
        }
    }
}

impl Video for GameBoy {
    const SIZE: Aspect = dmg::GameBoy::<rev::C>::SIZE;

    type Pixel = <dmg::GameBoy<rev::C> as Video>::Pixel;

    fn vsync(&self) -> bool {
        match self {
            Self::Dmg0(dmg) => dmg.vsync(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.vsync(),
        }
    }

    fn frame(&self) -> &[Self::Pixel] {
        match self {
            Self::Dmg0(dmg) => dmg.frame(),
            Self::DmgA(dmg) | Self::DmgB(dmg) | Self::DmgC(dmg) => dmg.frame(),
        }
    }
}
