//! Memory models.
//!
//! # Usage
//!
//! The [`Ram`] and [`Rom`] memory models work similarly to one another, with
//! the obvious exception that `Rom` returns an [error](Error::Misuse) on
//! writes.

use std::fmt::Debug;

use thiserror::Error;

use crate::Shared;
use crate::reg::Register;

mod ram;
mod rom;

pub use self::ram::Ram;
pub use self::rom::Rom;

/// Addressable memory-mapped interface.
pub trait Memory: Debug {
    /// Reads from the specified address.
    ///
    /// # Errors
    ///
    /// Errors if the device could not successfully be read from.
    fn read(&self, addr: u16) -> Result<u8>;

    /// Writes to the specified address.
    ///
    /// # Errors
    ///
    /// Errors if the device could not successfully be written to.
    fn write(&mut self, addr: u16, data: u8) -> Result<()>;
}

impl Memory for u8 {
    fn read(&self, _: u16) -> Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Memory for [u8] {
    fn read(&self, addr: u16) -> Result<u8> {
        self.get(usize::from(addr)).copied().ok_or(Error::Range)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.get_mut(usize::from(addr))
            .map(|val| *val = data)
            .ok_or(Error::Range)
    }
}

impl<const N: usize> Memory for [u8; N] {
    fn read(&self, addr: u16) -> Result<u8> {
        self.as_ref().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.as_mut().write(addr, data)
    }
}

impl Memory for Box<[u8]> {
    fn read(&self, addr: u16) -> Result<u8> {
        self.as_ref().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.as_mut().write(addr, data)
    }
}

impl<const N: usize> Memory for Box<[u8; N]> {
    fn read(&self, addr: u16) -> Result<u8> {
        self.as_ref().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.as_mut().write(addr, data)
    }
}

impl<M: Memory + ?Sized> Memory for Shared<M> {
    fn read(&self, addr: u16) -> Result<u8> {
        self.borrow().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.borrow_mut().write(addr, data)
    }
}

impl Memory for Vec<u8> {
    fn read(&self, addr: u16) -> Result<u8> {
        self.as_slice().read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.as_mut_slice().write(addr, data)
    }
}

/// A convenient type alias for [`Result`](std::result::Result).
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error caused by a [memory](Memory) operation.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Error {
    /// Device is unavailable.
    #[error("device is unavailable")]
    Busy,
    /// Device is disabled.
    #[error("device is disabled")]
    Disabled,
    /// Unsupported operation.
    #[error("unsupported operation")]
    Misuse,
    /// Address out of range.
    #[error("address out of range")]
    Range,
}
