//! Hardware blocks.
//!
//! Structural components of the SM83 core: the arithmetic and address
//! units, each usable at most once per M-cycle, and the port through
//! which the core drives the bus.

use rugby_arch::Block;

mod alu;
mod bus;
mod idu;

pub use self::alu::Alu;
pub use self::bus::Bus;
pub use self::idu::Idu;

/// Hardware blocks.
#[derive(Debug)]
pub struct Hardware {
    /// Arithmetic logic unit.
    pub alu: Alu,
    /// Increment decrement unit.
    pub idu: Idu,
    /// Processor bus.
    pub bus: Bus,
}

impl Hardware {
    /// Constructs a new `Hardware`.
    #[must_use]
    pub fn new(bus: Bus) -> Self {
        Self {
            alu: Alu::default(),
            idu: Idu::default(),
            bus,
        }
    }
}

impl Block for Hardware {
    fn cycle(&mut self) {
        // Release the blocks for reuse
        self.alu.clear();
        self.idu.clear();
    }
}
