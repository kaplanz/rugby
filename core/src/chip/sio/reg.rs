//! Serial registers.

use bitfield_struct::bitfield;
use log::{debug, warn};
use rugby_arch::mem::Memory;
use rugby_arch::reg::Register;

/// Serial data.
pub type Sb = u8;

/// `SC`: Serial control register.
///
/// | Bit | Name                | Use |
/// |-----|---------------------|-----|
/// |   7 | Transfer start flag | R/W |
/// | 6-1 | Unused.             | -   |
/// |   0 | Shift clock.        | R/W |
///
/// See more details [here][sc].
///
/// [sc]: https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
struct ScBits {
    /// `SC[7]`: Transfer enable.
    #[bits(1)]
    ena: bool,
    /// `SC[6:1]`: Unused.
    #[bits(6)]
    __: u8,
    /// `SC[0]`: Shift clock select.
    #[bits(1)]
    clk: bool,
}

/// Serial control.
#[derive(Debug, Default)]
pub struct Sc {
    reg: ScBits,
    pub(super) bit: u8,
}

impl Sc {
    pub(super) fn ena(&self) -> bool {
        self.reg.ena()
    }

    pub(super) fn clk(&self) -> bool {
        self.reg.clk()
    }

    pub(super) fn set_ena(&mut self, v: bool) {
        self.reg.set_ena(v);
    }
}

impl Memory for Sc {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Sc {
    type Value = u8;

    fn load(&self) -> Self::Value {
        // unused bits 6:1 always read as 1
        self.reg.into_bits() | 0x7e
    }

    fn store(&mut self, value: Self::Value) {
        if self.bit != 0 {
            warn!("interrupted serial transfer");
        }
        self.reg = ScBits::from_bits(value);
        // Reset transfer sequence bit
        self.bit = 0b1000_0000;
        debug!("started tx");
    }
}
