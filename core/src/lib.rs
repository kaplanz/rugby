//! # Game Boy Core.
//!
//! This library implements the core behaviour of the various hardware
//! components of the Nintendo Game Boy family of consoles.

#![warn(clippy::pedantic)]
// Allowed lints: rustc
#![allow(unused_parens)]
// Allowed lints: clippy
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unusual_byte_groupings)]

mod model;
mod parts;

/// Emulator API.
pub mod api {
    pub mod core;
    pub mod part;
}

pub use crate::model::dmg;
