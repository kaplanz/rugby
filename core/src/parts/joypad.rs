//! Player input device.

use std::collections::HashSet;
use std::ops::{BitOr, Not};

use log::{debug, trace};
use rugby_arch::mem::Memory;
use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

use super::pic::{self, Interrupt};
use crate::api::part::joypad::{Event, Input, Joypad as Api, State};

/// Joypad inputs.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Button {
    /// Game button: A
    A      = 0b0010_0001,
    /// Game button: B
    B      = 0b0010_0010,
    /// Menu button: Select
    Select = 0b0010_0100,
    /// Menu button: Start
    Start  = 0b0010_1000,
    /// D-pad input: Right
    Right  = 0b0001_0001,
    /// D-pad input: Left
    Left   = 0b0001_0010,
    /// D-pad input: Up
    Up     = 0b0001_0100,
    /// D-pad input: Down
    Down   = 0b0001_1000,
}

impl Button {
    fn mode(self) -> u8 {
        Mode::MASK & self as u8
    }

    fn key(self) -> u8 {
        Mode::KEYS & self as u8
    }
}

impl Input for Button {}

/// Joypad register mode.
#[derive(Clone, Copy, Debug, Default)]
pub enum Mode {
    /// Select none.
    #[default]
    None = 0b0000_0000,
    /// Select d-pad.
    DPad = 0b0001_1111,
    /// Select button.
    Keys = 0b0010_1111,
    /// Select both.
    Both = 0b0011_1111,
}

impl Mode {
    const KEYS: u8 = 0b0000_1111;
    const MASK: u8 = 0b0011_0000;

    fn mode(self) -> u8 {
        Self::MASK & self as u8
    }

    fn select(self, btn: Button) -> bool {
        btn.mode() & self.mode() != 0
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value & Self::MASK {
            0b0000_0000 => Mode::None,
            0b0001_0000 => Mode::DPad,
            0b0010_0000 => Mode::Keys,
            0b0011_0000 => Mode::Both,
            _ => unreachable!(),
        }
    }
}

/// Joypad controller.
#[derive(Debug)]
pub struct Joypad {
    /// Joypad register.
    pub reg: Shared<Control>,
    /// Interrupt line.
    pub int: pic::Line,
}

impl Api for Joypad {
    type Button = Button;

    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>) {
        // Borrow controller state
        let mut con = self.reg.borrow_mut();
        let mode = con.mode;
        let keys = &mut con.keys;
        // Update pressed keys
        let mut updated = false;
        let mut trigger = false;
        for Event { input: btn, state } in events {
            trace!("event: {btn:?}, {state:?}");
            // Update internal state
            updated |= match state {
                State::Dn => keys.insert(btn),
                State::Up => keys.remove(&btn),
            };
            // Determine if an interrupt should occur
            trigger |= state == State::Dn && mode.select(btn);
        }
        // Handle key updates
        if updated {
            debug!("updated keys: {keys:?}");
            // Schedule an interrupt on changes
            if trigger {
                self.int.raise(Interrupt::Joypad);
            }
        } else {
            trace!("received no input events");
        }
    }
}

impl Block for Joypad {
    fn reset(&mut self) {
        self.reg.reset();
    }
}

impl Mmio for Joypad {
    fn attach(&self, bus: &mut Bus) {
        bus.map(0xff00..=0xff00, self.reg.clone().into());
    }
}

/// Joypad register.
#[derive(Debug, Default)]
pub struct Control {
    mode: Mode,
    keys: HashSet<Button>,
}

impl Block for Control {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Memory for Control {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Control {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.keys
            .iter()
            .copied()
            // keep only selected keys
            .filter(|btn| self.mode.select(*btn))
            // mask the button's key
            .map(Button::key)
            // apply selected mode
            .fold(self.mode.mode(), BitOr::bitor)
            // values are read inverted
            .not()
    }

    fn store(&mut self, value: Self::Value) {
        self.mode = Mode::from(
            // values are written inverted
            !value,
        );
    }
}
