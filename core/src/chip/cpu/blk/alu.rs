//! Arithmetic logic unit.

use std::ops::{BitAnd, BitOr, BitXor};

use crate::chip::cpu::reg::F;

/// Arithmetic logic unit.
///
/// Performs 8-bit arithmetic and logic, taking operands with flags and
/// producing a result with flags, at most once per M-cycle.
#[derive(Debug, Default)]
pub struct Alu {
    /// Cycle usage marker.
    #[cfg(debug_assertions)]
    used: bool,
}

impl Alu {
    /// Adds without carry.
    pub fn add(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute ADD
        let (res, carry) = op1.overflowing_add(op2);
        // Set flags
        f.set_z(res == 0);
        f.set_n(false);
        f.set_h(0x0f < (op1 & 0x0f) + (op2 & 0x0f));
        f.set_c(carry);
        (res, f)
    }

    /// Adds with carry.
    pub fn adc(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute ADC
        let cin = f.c() as u8;
        let (res, carry0) = op1.overflowing_add(op2);
        let (res, carry1) = res.overflowing_add(cin);
        // Set flags
        f.set_z(res == 0);
        f.set_n(false);
        f.set_h(0x0f < (op1 & 0x0f) + (op2 & 0x0f) + cin);
        f.set_c(carry0 | carry1);
        (res, f)
    }

    /// Subtracts without borrow.
    pub fn sub(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute SUB
        let (res, carry) = op1.overflowing_sub(op2);
        // Set flags
        f.set_z(res == 0);
        f.set_n(true);
        f.set_h((op2 & 0x0f) > (op1 & 0x0f));
        f.set_c(carry);
        (res, f)
    }

    /// Subtracts with borrow.
    pub fn sbc(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute SBC
        let cin = f.c() as u8;
        let (res, carry0) = op1.overflowing_sub(op2);
        let (res, carry1) = res.overflowing_sub(cin);
        // Set flags
        f.set_z(res == 0);
        f.set_n(true);
        f.set_h((op2 & 0x0f) + cin > (op1 & 0x0f));
        f.set_c(carry0 | carry1);
        (res, f)
    }

    /// Compares by subtraction.
    pub fn cmp(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute CP
        let (res, carry) = op1.overflowing_sub(op2);
        // Set flags
        f.set_z(res == 0);
        f.set_n(true);
        f.set_h((op2 & 0x0f) > (op1 & 0x0f));
        f.set_c(carry);
        (res, f)
    }

    /// Computes bitwise AND.
    pub fn and(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute AND
        let res = op1.bitand(op2);
        // Set flags
        f.set_z(res == 0);
        f.set_n(false);
        f.set_h(true);
        f.set_c(false);
        (res, f)
    }

    /// Computes bitwise OR.
    pub fn or(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute OR
        let res = op1.bitor(op2);
        // Set flags
        f.set_z(res == 0);
        f.set_n(false);
        f.set_h(false);
        f.set_c(false);
        (res, f)
    }

    /// Computes bitwise XOR.
    pub fn xor(&mut self, op1: u8, op2: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute XOR
        let res = op1.bitxor(op2);
        // Set flags
        f.set_z(res == 0);
        f.set_n(false);
        f.set_h(false);
        f.set_c(false);
        (res, f)
    }

    /// Increments a value.
    #[expect(clippy::verbose_bit_mask)]
    pub fn inc(&mut self, op1: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute INC
        let res = op1.wrapping_add(1);
        // Set flags
        f.set_z(res == 0);
        f.set_n(false);
        f.set_h(res & 0x0f == 0);
        (res, f)
    }

    /// Decrements a value.
    #[expect(clippy::verbose_bit_mask)]
    pub fn dec(&mut self, op1: u8, mut f: F) -> (u8, F) {
        self.mark();
        // Execute DEC
        let res = op1.wrapping_sub(1);
        // Set flags
        f.set_z(res == 0);
        f.set_n(true);
        f.set_h(op1 & 0x0f == 0);
        (res, f)
    }
}

impl Alu {
    /// Marks a use this M-cycle.
    fn mark(&mut self) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(!self.used, "ALU already used this M-cycle");
            self.used = true;
        }
    }

    /// Clears the usage marker.
    pub(super) fn clear(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.used = false;
        }
    }
}
