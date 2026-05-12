//! A cycle-accurate Game Boy emulator.
//!
//! This project emulator core implementations for the following Game Boy
//! models:
//!
//! - [`DMG`](crate::core::dmg): _Game Boy_
//!
//! In addition, the following models are planned:
//!
//! - `CGB`: _Game Boy Color_
//! - `AGB`: _Game Boy Advance_ (compat only)[^1]
//!
//! [^1]: Planned support covers Game Boy (Color) compatibility only. Game Boy
//!       Advance games run on a distinct ARM-based architecture which is out of
//!       scope for this project.
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
