//! On-chip silicon.

#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

pub mod apu;
pub mod cpu;
pub mod dma;
pub mod irq;
pub mod joy;
pub mod ppu;
pub mod sio;
pub mod tma;
