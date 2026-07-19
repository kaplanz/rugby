//! Joypad API.

use rugby::api::input::{Input, State};
use rugby::core::dmg;
use wasm_bindgen::prelude::*;

use crate::api::GameBoy;

#[wasm_bindgen]
impl GameBoy {
    /// Presses a button.
    ///
    /// Presses a button in the internal emulator state. If already pressed,
    /// this is a no-op.
    pub fn press(&mut self, key: Button) {
        self.0
            .recv(Some((dmg::soc::joy::Button::from(key), State::Dn).into()));
    }

    /// Releases a button.
    ///
    /// Removes a pressed button from the internal emulator state. If not
    /// pressed, this is a no-op.
    pub fn release(&mut self, key: Button) {
        self.0
            .recv(Some((dmg::soc::joy::Button::from(key), State::Up).into()));
    }
}

/// Joypad inputs.
///
/// Represents the 4 face buttons and directional inputs on the joypad.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub enum Button {
    /// Game button: A
    A,
    /// Game button: B
    B,
    /// Menu button: Select
    Select,
    /// Menu button: Start
    Start,
    /// D-pad input: Right
    Right,
    /// D-pad input: Left
    Left,
    /// D-pad input: Up
    Up,
    /// D-pad input: Down
    Down,
}

#[rustfmt::skip]
impl From<Button> for dmg::soc::joy::Button {
    fn from(value: Button) -> Self {
        match value {
            Button::A      => dmg::soc::joy::Button::A,
            Button::B      => dmg::soc::joy::Button::B,
            Button::Select => dmg::soc::joy::Button::Select,
            Button::Start  => dmg::soc::joy::Button::Start,
            Button::Right  => dmg::soc::joy::Button::Right,
            Button::Left   => dmg::soc::joy::Button::Left,
            Button::Up     => dmg::soc::joy::Button::Up,
            Button::Down   => dmg::soc::joy::Button::Down,
        }
    }
}
