//! Model interface.

use rugby_arch::Block;

mod imp;

pub(crate) mod has;

/// Emulator core.
pub trait Core: Block + Sized {
    /// Borrow the core's insides.
    fn inside(&'_ self) -> Inside<'_, Self> {
        Inside(self)
    }

    /// Mutably borrow the core's insides.
    fn inside_mut(&'_ mut self) -> InsideMut<'_, Self> {
        InsideMut(self)
    }
}

/// Borrow the core's insides.
pub struct Inside<'a, C: Core>(&'a C);

/// Mutably borrow the core's insides.
pub struct InsideMut<'a, C: Core>(&'a mut C);
