//! Joypad API.

use rugby::core::dmg;
use rugby::emu::part::joypad::State;
use rugby::prelude::*;
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
            .inside_mut()
            .joypad()
            .recv(Some((dmg::Button::from(key), State::Dn).into()));
    }

    /// Releases a button.
    ///
    /// Removes a pressed button from the internal emulator state. If not
    /// pressed, this is a no-op.
    pub fn release(&mut self, key: Button) {
        self.0
            .inside_mut()
            .joypad()
            .recv(Some((dmg::Button::from(key), State::Up).into()));
    }
}

/// Joypad inputs.
///
/// Represents the 4 face buttons and directional inputs on the joypad.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
impl From<Button> for dmg::Button {
    fn from(value: Button) -> Self {
        match value {
            Button::A      => dmg::Button::A,
            Button::B      => dmg::Button::B,
            Button::Select => dmg::Button::Select,
            Button::Start  => dmg::Button::Start,
            Button::Right  => dmg::Button::Right,
            Button::Left   => dmg::Button::Left,
            Button::Up     => dmg::Button::Up,
            Button::Down   => dmg::Button::Down,
        }
    }
}
