//! Emulation cores for `rugby`.
//!
//! This crate implements cycle-accurate emulation for the Nintendo Game Boy
//! family of systems. It is the engine beneath the top-level [`rugby`] crate
//! and is not intended for direct use by application code. Frontend developers
//! should prefer [`rugby`] instead.
//!
//! # Revisions
//!
//! Emulator models differentiate silicon revision via a generic [`Revision`]
//! marker type. The following revisions are supported:
//!
//! - [DMG-CPU-0](model::dmg::rev::Zero).
//! - [DMG-CPU-A](model::dmg::rev::A).
//! - [DMG-CPU-B](model::dmg::rev::B).
//! - [DMG-CPU-C](model::dmg::rev::C).
//!
//! [`rugby`]: https://docs.rs/rugby

#![warn(clippy::pedantic)]
// Allowed lints: rustc
#![allow(unused_parens)]
// Allowed lints: clippy
#![allow(clippy::similar_names)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unusual_byte_groupings)]

mod model;
mod rev;

pub mod api;
pub mod cart;
pub mod chip;

pub use crate::model::dmg;
pub use crate::rev::Revision;
