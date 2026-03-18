//! Joypad API.

use std::hash::Hash;

/// Joypad interface.
pub trait Joypad {
    /// User input.
    type Button: Input;

    /// Receives user input events, forwarding to the core.
    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>);
}

/// Joypad input.
pub trait Input: Copy + Eq + Hash {}

/// Jopypad event.
#[derive(Copy, Clone, Debug)]
pub struct Event<I>
where
    I: Copy + Eq + Hash,
{
    /// Input identifier.
    pub input: I,
    /// Input state.
    pub state: State,
}

impl<I: Copy + Eq + Hash> From<(I, State)> for Event<I> {
    fn from(value: (I, State)) -> Self {
        let (input, state) = value;
        Event { input, state }
    }
}

/// Joypad button state.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// Button pressed.
    Dn,
    /// Button released.
    Up,
}
