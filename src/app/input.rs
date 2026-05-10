//! Input API.

use crate::emu::input::{Button, Event};

/// Input interface.
pub trait Input {
    type Button: Button;

    fn events(&mut self) -> Vec<Event<Self::Button>>;
}
