use rugby::core::dmg;
use rugby::emu::part::joypad::State;
use rugby::prelude::*;
use wasm_bindgen::prelude::*;

use crate::emu::GameBoy;

#[wasm_bindgen]
impl GameBoy {
    pub fn press(&mut self, key: Button) {
        let event = Some((dmg::Button::from(key), State::Dn).into());
        self.0.inside_mut().joypad().recv(event);
    }

    pub fn release(&mut self, key: Button) {
        let event = Some((dmg::Button::from(key), State::Up).into());
        self.0.inside_mut().joypad().recv(event);
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
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
