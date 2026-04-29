//! Joypad registers.

use bitfield_struct::bitfield;
use rugby_arch::mem::Memory;
use rugby_arch::reg::Register;

/// `P1`: Joypad register.
///
/// | Bit | Name                     | Use |
/// |-----|--------------------------|-----|
/// | 7-6 | Unused.                  | -   |
/// |   5 | Action button select.    | W   |
/// |   4 | Direction button select. | W   |
/// | 3-0 | Button states.           | R   |
///
/// See more details [here][p1].
///
/// [p1]: https://gbdev.io/pandocs/Joypad_Input.html
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
pub struct P1 {
    /// `P1[7:6]`: Unused.
    #[bits(2)]
    __: u8,
    /// `P1[5]`: Action button select.
    #[bits(1)]
    pub action: bool,
    /// `P1[4]`: Direction button select.
    #[bits(1)]
    pub direction: bool,
    /// `P1[3:0]`: Button states.
    #[bits(4)]
    pub keys: u8,
}

impl Memory for P1 {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for P1 {
    type Value = u8;

    fn load(&self) -> Self::Value {
        // unused bits 7:6 always read as 1
        self.into_bits() | 0xc0
    }

    fn store(&mut self, value: Self::Value) {
        *self = Self::from_bits(value);
    }
}
