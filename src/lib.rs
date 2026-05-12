//! A cycle-accurate Game Boy emulator.
//!
//! Emulator implementations for the following Game Boy models:
//!
//! - [`DMG`](crate::core::dmg): [Game Boy]
//!
//! # Examples
//!
//! ```
//! use rugby::arch::Block;
//! use rugby::core::cart::Cartridge;
//! use rugby::core::dmg::GameBoy;
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

#[doc(inline)]
pub use rugby_arch as arch;
#[cfg(feature = "cfg")]
#[doc(inline)]
pub use rugby_cfg as cfg;
#[doc(inline)]
pub use rugby_core as core;
#[cfg(feature = "gbd")]
#[doc(inline)]
pub use rugby_gbd as gbd;
#[cfg(feature = "pal")]
#[doc(inline)]
pub use rugby_pal as pal;

pub use crate::core::api;
pub use crate::emu::GameBoy;

pub mod emu;

/// Name of this crate.
pub const NAME: &str = env!("CARGO_CRATE_NAME");
