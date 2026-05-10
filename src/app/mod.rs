//! Frontend API.

use crate::emu::core::Core;

pub mod audio;
pub mod cable;
pub mod input;
pub mod video;

use self::audio::Audio;
use self::cable::Cable;
use self::input::Input;
use self::video::Video;

/// Emulator frontend.
pub trait Frontend: Audio + Cable + Input + Video {
    /// Emulator core.
    type Core: Core;
}
