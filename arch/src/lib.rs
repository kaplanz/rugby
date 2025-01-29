//! Game Boy Architecture

#![warn(clippy::pedantic)]

mod blk;
mod clk;

pub mod dev;
pub mod mem;
pub mod mio;
pub mod reg;

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub use crate::blk::Block;
pub use crate::clk::Clock;

/// Native architecture byte.
pub type Byte = u8;
/// Native architecture word.
pub type Word = u16;

/// Interface for accessing values according to a bitmask.
pub trait Bitmask<M>
where
    M: Copy + Into<Self>,
    Self: Sized,
{
    /// Tests whether a given control bit is set.
    #[must_use]
    fn test(&self, mask: M) -> bool;

    /// Updates a given control bit's value.
    fn set(&mut self, mask: M, value: bool);

    /// Atomically overwrite a bit, returning the old value.
    #[must_use]
    fn test_and_set(&mut self, mask: M, value: bool) -> bool {
        // Test old value.
        let old = self.test(mask);
        // Set new value.
        self.set(mask, value);
        // Return old value
        old
    }
}

/// Shared memory-mapped device.
#[derive(Debug, Default)]
pub struct Shared<T: ?Sized>(Inner<T>);

/// Underlying shared pointer.
type Inner<T> = Rc<RefCell<T>>;

impl<T> Shared<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

impl<T: ?Sized> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: ?Sized> Deref for Shared<T> {
    type Target = Inner<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for Shared<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}
