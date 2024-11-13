//! Frontend message protocol.

#![allow(unused)]

use derive_more::Display;
use rugby::core::dmg::ppu;

use super::Exit;
use crate::emu::ctx::Stats;

/// Frontend-thread messages.
#[derive(Debug, Display)]
pub enum Message {
    /// Debug info.
    #[cfg(feature = "debug")]
    #[display("debug info: {_0}")]
    Debug(Debug),
    // Run statistics.
    #[display("run statistics: {_0}")]
    Stats(Stats),
    /// Video data.
    #[display("video data")]
    Video(Box<[ppu::Color]>),
    /// Exit condition.
    #[display("exit condition: {_0}")]
    Exit(Exit),
}

/// Debug info.
#[cfg(feature = "debug")]
#[derive(Debug, Display)]
pub enum Debug {
    /// Video RAM data.
    #[cfg(feature = "win")]
    #[display("VRAM")]
    Vram(ppu::dbg::Debug),
}
