//! Increment decrement unit.

/// Increment decrement unit.
///
/// Bumps 16-bit addresses independently of the [ALU](super::Alu), at most
/// once per M-cycle.
#[derive(Debug, Default)]
pub struct Idu {
    /// Cycle usage marker.
    #[cfg(debug_assertions)]
    used: bool,
}

impl Idu {
    /// Increments an address.
    #[must_use]
    pub fn inc(&mut self, addr: u16) -> u16 {
        self.mark();
        addr.wrapping_add(1)
    }

    /// Decrements an address.
    #[must_use]
    pub fn dec(&mut self, addr: u16) -> u16 {
        self.mark();
        addr.wrapping_sub(1)
    }

    /// Marks a use this M-cycle.
    fn mark(&mut self) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(!self.used, "IDU already used this M-cycle");
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
