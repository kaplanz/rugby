//! Joypad API.

use rugby::core::dmg;
use rugby::emu::part::joypad::State;
use rugby::prelude::*;

use super::GameBoy;

#[uniffi::export]
impl GameBoy {
    /// Presses a button.
    ///
    /// Presses a button in the internal emulator state. If already pressed,
    /// this is a no-op.
    #[uniffi::method]
    pub fn press(&self, key: Button) {
        self.0
            .write()
            .unwrap()
            .inside_mut()
            .joypad()
            .recv(Some((dmg::Button::from(key), State::Dn).into()));
    }

    /// Releases a button.
    ///
    /// Removes a pressed button from the internal emulator state. If not
    /// pressed, this is a no-op.
    #[uniffi::method]
    pub fn release(&self, key: Button) {
        self.0
            .write()
            .unwrap()
            .inside_mut()
            .joypad()
            .recv(Some((dmg::Button::from(key), State::Up).into()));
    }
}

/// Joypad inputs.
///
/// Represents the 4 face buttons and directional inputs on the joypad.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[derive(uniffi::Enum)]
#[rustfmt::skip]
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
