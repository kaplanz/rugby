//! Architecture primitives for `rugby`.
//!
//! This crate defines the foundational building blocks used throughout the
//! emulator. It is hardware-agnostic: nothing here is specific to the Game
//! Boy. All emulated hardware components are ultimately composed from these
//! primitives.

#![warn(clippy::pedantic)]

mod blk;
mod clk;

pub mod dev;
pub mod mem;
pub mod reg;

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub use crate::blk::Block;
pub use crate::clk::Clock;

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
