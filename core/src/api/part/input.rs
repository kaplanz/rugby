//! Input API.

use std::hash::Hash;

/// Input interface.
pub trait Input {
    /// User input.
    type Button: Button;

    /// Receives user input events, forwarding to the core.
    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>);
}

/// Input button marker.
pub trait Button: Copy + Eq + Hash {}

/// Input event.
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

/// Input button state.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    /// Button pressed.
    Dn,
    /// Button released.
    Up,
}
