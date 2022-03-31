use std::cell::RefCell;
use std::rc::Rc;

use remus::reg::Register;
use remus::Block;
use thiserror::Error;

use crate::util::Bitflags;

#[derive(Debug, Default)]
pub struct Pic {
    // ┌─────┬──────────┬─────────┐
    // │ BIT │  SOURCE  │ HANDLER │
    // ├─────┼──────────┼─────────┤
    // │   0 │   VBlank │  0x0040 │
    // │   1 │ LCD STAT │  0x0048 │
    // │   2 │    Timer │  0x0050 │
    // │   3 │   Serial │  0x0058 │
    // │   4 │   Joypad │  0x0060 │
    // └─────┴──────────┴─────────┘
    // Interrupt Enable (IE)
    pub enable: Rc<RefCell<Register<u8>>>,
    // Interrupt Flag (IF)
    pub active: Rc<RefCell<Register<u8>>>,
}

impl Block for Pic {
    fn reset(&mut self) {
        // Reset registers
        self.enable.borrow_mut().reset();
        self.active.borrow_mut().reset();
    }
}

#[rustfmt::skip]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    VBlank  = 0b00000001,
    LcdStat = 0b00000010,
    Timer   = 0b00000100,
    Serial  = 0b00001000,
    Joypad  = 0b00010000,
}

impl Bitflags for Interrupt {}

impl TryFrom<u8> for Interrupt {
    type Error = InterruptError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value.leading_zeros() {
            0 => Ok(Self::VBlank),
            1 => Ok(Self::LcdStat),
            2 => Ok(Self::Timer),
            3 => Ok(Self::Serial),
            4 => Ok(Self::Joypad),
            _ => Err(InterruptError::Unknown),
        }
    }
}

impl From<Interrupt> for u8 {
    fn from(value: Interrupt) -> Self {
        value as u8
    }
}

#[derive(Debug, Error)]
pub enum InterruptError {
    #[error("Unknown Interrupt")]
    Unknown,
}
