use rugby::core::dmg;
use rugby::emu::part::joypad::State;
use rugby::prelude::*;
use uniffi::Enum;

use crate::GameBoy;

#[uniffi::export]
impl GameBoy {
    /// Forwards a button press event.
    #[uniffi::method]
    pub fn press(&self, key: Button) {
        let event = Some((dmg::Button::from(key), State::Dn).into());
        self.0.write().unwrap().inside_mut().joypad().recv(event);
    }

    /// Forwards a button release event.
    #[uniffi::method]
    pub fn release(&self, key: Button) {
        let event = Some((dmg::Button::from(key), State::Up).into());
        self.0.write().unwrap().inside_mut().joypad().recv(event);
    }
}

#[derive(Copy, Clone, Debug, Enum)]
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
