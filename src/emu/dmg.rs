//! Game Boy (DMG) revisions.

use std::io::{BufRead, Write};

use crate::arch::Block;
use crate::core::api::audio::{Audio, Chiptune};
use crate::core::api::cable::Cable;
use crate::core::api::input::{Event, Input};
use crate::core::api::video::{Aspect, Video};
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

impl Audio for Dmg {
    fn sample(&self) -> Chiptune {
        match self {
            Self::Zero(gb) => gb.sample(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.sample(),
        }
    }
}

impl Cable for Dmg {
    fn rx(&mut self) -> &mut dyn BufRead {
        match self {
            Self::Zero(gb) => gb.rx(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.rx(),
        }
    }

    fn tx(&mut self) -> &mut dyn Write {
        match self {
            Self::Zero(gb) => gb.tx(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.tx(),
        }
    }
}

impl Input for Dmg {
    type Button = <GameBoy<rev::C> as Input>::Button;

    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>) {
        match self {
            Self::Zero(gb) => gb.recv(events),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.recv(events),
        }
    }
}

impl Video for Dmg {
    const SIZE: Aspect = GameBoy::<rev::C>::SIZE;

    type Pixel = <GameBoy<rev::C> as Video>::Pixel;

    fn vsync(&self) -> bool {
        match self {
            Self::Zero(gb) => gb.vsync(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.vsync(),
        }
    }

    fn frame(&self) -> &[Self::Pixel] {
        match self {
            Self::Zero(gb) => gb.frame(),
            Self::A(gb) | Self::B(gb) | Self::C(gb) => gb.frame(),
        }
    }
}
