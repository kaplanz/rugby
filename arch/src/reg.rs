//! Register models.

use crate::Shared;

/// An access point within a register file.
pub trait Port<R: Register> {
    /// Register to select.
    type Select;

    /// Loads from the specified register.
    fn load(&self, reg: Self::Select) -> R;

    /// Stores to the specified register.
    fn store(&mut self, reg: Self::Select, value: R);
}

/// Register load-store interface.
pub trait Register {
    /// Value stored by the register.
    type Value;

    /// Loads the value of a register.
    fn load(&self) -> Self::Value;

    /// Stores a value into a register.
    fn store(&mut self, value: Self::Value);
}

impl Register for u8 {
    type Value = Self;

    fn load(&self) -> u8 {
        *self
    }

    fn store(&mut self, value: u8) {
        *self = value;
    }
}

impl Register for u16 {
    type Value = Self;

    fn load(&self) -> u16 {
        *self
    }

    fn store(&mut self, value: u16) {
        *self = value;
    }
}

impl<R: Register> Register for Shared<R> {
    type Value = <R as Register>::Value;

    fn load(&self) -> Self::Value {
        self.borrow().load()
    }

    fn store(&mut self, value: Self::Value) {
        self.borrow_mut().store(value);
    }
}
