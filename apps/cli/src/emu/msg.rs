//! Emulator message protocol.

#![allow(unused)]

use derive_more::Display;
use rugby::core::dmg::Button;
use rugby::emu::part::joypad::Event;

/// Emulator-thread messages.
#[derive(Debug, Display)]
pub enum Message {
    /// Play emulator.
    #[display("play emulator")]
    Play,
    /// Stop emulator.
    #[display("stop emulator")]
    Stop,
    /// Debugger break.
    #[cfg(feature = "gbd")]
    #[display("debugger break")]
    Break,
    /// Receive data.
    #[display("receive data: {_0:?}")]
    Data(Data),
    /// Synchronize.
    #[display("synchronize: {_0:?}")]
    Sync(Sync),
    /// Exit requested.
    #[display("exit requested")]
    Exit,
}

/// Synchronize.
#[derive(Debug)]
pub enum Sync {
    /// Acknowledge video.
    Video,
}

/// Receive data.
#[derive(Debug)]
pub enum Data {
    /// Joypad events.
    Joypad(Vec<Event<Button>>),
    /// Serial data.
    Serial(u8),
}
