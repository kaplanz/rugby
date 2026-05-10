//! Input API.

use crate::emu::part::input::{Button, Event};

/// Input interface.
pub trait Input {
    type Button: Button;

    fn events(&mut self) -> Vec<Event<Self::Button>>;
}
