//! Timer registers.

use bitfield_struct::bitfield;
use log::debug;
use rugby_arch::mem::Memory;
use rugby_arch::reg::Register;

/// Divider register.
#[derive(Debug, Default)]
pub struct Div(u16);

impl Div {
    /// Gets the internal clock (lower 8-bits).
    #[expect(unused)]
    #[must_use]
    pub(super) fn clk(&self) -> u8 {
        self.0.load().to_le_bytes()[0]
    }

    /// Gets the full internal register value.
    #[must_use]
    pub(super) fn div(&self) -> u16 {
        self.0.load()
    }

    /// Increment the divider register.
    pub(super) fn inc(&mut self) {
        let value = self.0.load().wrapping_add(1);
        self.0.store(value);
    }
}

impl Memory for Div {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Div {
    type Value = u8;

    /// Loads the value of DIV (upper 8-bits).
    fn load(&self) -> Self::Value {
        self.0.load().to_le_bytes()[1]
    }

    fn store(&mut self, _: Self::Value) {
        debug!("resetting divider");
        self.0.store(0);
    }
}

/// Timer counter.
#[derive(Debug, Default)]
pub struct Tima {
    pub(super) reg: u8,
    pub(super) rel: Reload,
    pub(super) sup: bool,
}

/// Timer reload counter.
///
/// In effect, this models the 1 M-cycle (4 T-cycle) delay between a reload
/// being triggered and it occurring, followed by the 1 M-cycle window during
/// which the reload commits.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub(super) enum Reload {
    /// Timer is not amid a reload.
    #[default]
    None,
    /// Reload pending in `N` cycles.
    Wait(u8),
    /// Reload occurs this cycle.
    Load,
    /// Reload commit for `N` more cycles.
    Done(u8),
}

impl Reload {
    /// Set a reload to occur.
    pub(super) fn sched(&mut self) {
        assert!(matches!(self, Self::None));
        *self = Self::Wait(2);
    }

    /// Advance the reload counter.
    pub(super) fn tick(&mut self) {
        *self = match self {
            // Advance to the reload
            Reload::Wait(0) => Reload::Load,
            // Decrement cycles until reload
            Reload::Wait(count) => Reload::Wait(*count - 1),
            // Reload just occurred, so commit for the next M-cycle
            Reload::Load => Reload::Done(3),
            // Decrement cycles while committing
            Reload::Done(count @ 1..) => Reload::Done(*count - 1),
            // Commit finished, or counter is inactive
            Reload::Done(0) | Reload::None => Reload::None,
        }
    }
}

impl Tima {
    /// Increment the timer counter.
    #[must_use]
    pub(super) fn inc(&mut self) -> bool {
        let (value, carry) = self.reg.load().overflowing_add(1);
        self.reg.store(value);
        carry
    }
}

impl Memory for Tima {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Tima {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.reg.load()
    }

    fn store(&mut self, value: Self::Value) {
        match self.rel {
            // Ignore stores while committing
            Reload::Done(_) => {}
            // Store, suppressing the copy from TMA
            Reload::Wait(_) | Reload::Load => {
                self.reg.store(value);
                self.sup = true;
            }
            // Otherwise, store as usual
            Reload::None => self.reg.store(value),
        }
    }
}

/// Timer modulo.
pub type Tma = u8;

/// `TAC`: Timer control register.
///
/// | Bit | Name         | Use |
/// |-----|--------------|-----|
/// | 7-3 | Unused.      | -   |
/// |   2 | Timer enable | R/W |
/// | 1-0 | Clock select | R/W |
///
/// See more details [here][tac].
///
/// [tac]: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff07--tac-timer-control
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
pub struct Tac {
    /// `TAC[7:3]`: Unused.
    #[bits(5)]
    __: u8,
    /// `TAC[2]`: Timer enable.
    #[bits(1)]
    pub ena: bool,
    /// `TAC[1:0]`: Clock select.
    #[bits(2)]
    pub clk: u8,
}

impl Tac {
    /// Gets the clock select rate.
    #[must_use]
    pub fn select(&self) -> u16 {
        match self.clk() {
            0b01 => 1 << 3,
            0b10 => 1 << 5,
            0b11 => 1 << 7,
            0b00 => 1 << 9,
            _ => unreachable!(),
        }
    }
}

impl Memory for Tac {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Tac {
    type Value = u8;

    fn load(&self) -> Self::Value {
        // unused bits 7:3 always read as 1
        self.into_bits() | 0xf8
    }

    fn store(&mut self, value: Self::Value) {
        *self = Self::from_bits(value & 0b111);
    }
}
