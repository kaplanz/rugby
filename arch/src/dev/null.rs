use crate::mem::{Memory, Result};

/// Null device.
///
/// # Usage
///
/// The `Null` device ignores all writes, and always yields the same "garbage"
/// values when read. This can be useful to allow memory accesses to an unmapped
/// region of memory without causing a panic.
///
/// `Null` defaults to yielding the null byte (`0x00`) when read, but this can
/// be changed either by constructing with [`Null::with`], or through the
/// [`Null::value`] method at runtime.
#[derive(Debug, Default)]
pub struct Null(u8);

impl Null {
    /// Constructs a new `Null` device.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an instance of [`Null`] that yields the specified value when
    /// performing a read.
    #[must_use]
    pub fn with(value: u8) -> Self {
        Self(value)
    }

    /// Set the value to be used when performing a read.
    pub fn value(&mut self, value: u8) {
        self.0 = value;
    }
}

impl Memory for Null {
    fn read(&self, _: u16) -> Result<u8> {
        Ok(self.0)
    }

    fn write(&mut self, _: u16, _: u8) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let null = Null::new();
        assert!(
            (0..0x100)
                .map(|addr| null.read(addr))
                .all(|data| data == Ok(0))
        );
    }

    #[test]
    fn with_works() {
        let null = Null::with(0xaa);
        assert!(
            (0..0x100)
                .map(|addr| null.read(addr))
                .all(|data| data == Ok(0xaa))
        );
    }

    #[test]
    fn memory_read_works() {
        let null = Null::with(0xaa);
        assert!(
            (0..0x100)
                .map(|addr| null.read(addr))
                .all(|data| data == Ok(null.0))
        );
    }

    #[test]
    fn memory_write_works() {
        let mut null = Null::new();
        (0..0x100).for_each(|addr| null.write(addr, 0xaa).unwrap());
        assert!(
            (0..0x100)
                .map(|addr| null.read(addr))
                .all(|data| data == Ok(0))
        );
    }
}
