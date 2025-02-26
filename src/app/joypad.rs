//! Joypad API.

use crate::emu::part::joypad::{Event, Input};

/// Joypad interface.
pub trait Joypad {
    type Button: Input;

    /// Polls for joypad events.
    ///
    /// Produces an ordered list of all joypad events since input was last
    /// polled.
    fn events(&mut self) -> Vec<Event<Self::Button>>;
}
