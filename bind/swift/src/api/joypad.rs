//! Joypad API.

use rugby::core::dmg::Button;
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
        self.inner
            .write()
            .inside_mut()
            .joypad()
            .recv(Some((key, State::Dn).into()));
    }

    /// Releases a button.
    ///
    /// Removes a pressed button from the internal emulator state. If not
    /// pressed, this is a no-op.
    #[uniffi::method]
    pub fn release(&self, key: Button) {
        self.inner
            .write()
            .inside_mut()
            .joypad()
            .recv(Some((key, State::Up).into()));
    }
}

/// Joypad inputs.
///
/// See [`rugby::core::dmg::Button`].
#[uniffi::remote(Enum)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}
