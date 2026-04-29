//! Processor registers.

use bitfield_struct::bitfield;
use rugby_arch::reg::Register;

/// `A`: Accumulator register.
pub type A = u8;

/// `B`: General register B.
pub type B = u8;

/// `C`: General register C.
pub type C = u8;

/// `D`: General register D.
pub type D = u8;

/// `E`: General register E.
pub type E = u8;

/// `H`: Address register (high byte).
pub type H = u8;

/// `L`: Address register (low byte).
pub type L = u8;

/// `SP`: Stack pointer.
pub type Sp = u16;

/// `PC`: Program counter.
pub type Pc = u16;

/// `F`: Flags register.
///
/// | Bit | Name | Explanation         |
/// |-----|------|---------------------|
/// |   7 | Z    | Zero flag.          |
/// |   6 | N    | Subtraction flag.   |
/// |   5 | H    | Half-carry flag.    |
/// |   4 | C    | Carry flag.         |
///
/// See more details [here][flag].
///
/// [flag]: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html#the-flags-register-lower-8-bits-of-af-register
#[bitfield(u8, order = msb)]
#[derive(PartialEq, Eq)]
pub struct F {
    /// `F[7]`: Zero flag.
    #[bits(1)]
    pub z: bool,
    /// `F[6]`: Subtraction flag.
    #[bits(1)]
    pub n: bool,
    /// `F[5]`: Half-carry flag.
    #[bits(1)]
    pub h: bool,
    /// `F[4]`: Carry flag.
    #[bits(1)]
    pub c: bool,
    /// `F[3:0]`: Unused.
    #[bits(4)]
    __: u8,
}

impl Register for F {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.into_bits()
    }

    fn store(&mut self, value: Self::Value) {
        *self = Self::from_bits(value & 0xf0);
    }
}
