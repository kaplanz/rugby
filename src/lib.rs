//! # Game Boy
//!
//! Emulator implementations for the following Game Boy models:
//! - [`DMG`](crate::core::dmg): [Game Boy]
//!
//! # Examples
//!
//! ```
//! use rugby::arch::Block; // for `Block::cycle`
//! use rugby::core::dmg::GameBoy;
//! use rugby::core::cart::Cartridge;
//! use rugby::prelude::*;
//!
//! // Instantiate a cartridge from ROM bytes
//! let rom: &[u8]; // -- snip --
//! # rom = include_bytes!("../roms/test/acid2/dmg-acid2.gb");
//! let cart = Cartridge::new(rom).unwrap();
//!
//! // Create an emulator instance
//! let mut emu: GameBoy = GameBoy::new();
//! // Load the cartridge into the emulator
//! emu.insert(cart);
//!
//! // Run the emulator
//! loop {
//!     emu.cycle();
//! #     break;
//! }
//! ```
//!
//! [Game Boy]: https://en.wikipedia.org/wiki/Game_Boy

#![warn(clippy::pedantic)]

mod sys;

#[doc(inline)]
pub use rugby_arch as arch;
#[doc(inline)]
pub use rugby_core as core;

pub use self::sys::GameBoy;

/// Extra features.
pub mod extra {
    #[cfg(feature = "cfg")]
    #[doc(inline)]
    pub use rugby_cfg as cfg;
    #[cfg(feature = "gbd")]
    #[doc(inline)]
    pub use rugby_gbd as gbd;
    #[cfg(feature = "pal")]
    #[doc(inline)]
    pub use rugby_pal as pal;
}

pub use crate::core::api as emu;

/// Name of this crate.
pub const NAME: &str = env!("CARGO_CRATE_NAME");

/// A prelude for conveniently writing emulator code.
///
/// The purpose of this module is to alleviate imports of many common traits by
/// adding a glob import to the top of modules:
///
/// ```rust
/// use rugby::prelude::*;
/// ```
///
/// Includes all core and library API traits.
#[rustfmt::skip]
pub mod prelude {
    // Emulator
    pub use crate::emu::audio::Audio as _;
    pub use crate::emu::cable::Cable as _;
    pub use crate::emu::input::Input as _;
    pub use crate::emu::video::Video as _;
}
