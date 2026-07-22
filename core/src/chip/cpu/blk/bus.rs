//! Processor bus.

use log::warn;
use rugby_arch::mem::Memory;

use crate::dmg::bus;

/// Processor bus.
#[derive(Debug)]
pub struct Bus {
    /// Bus view.
    bus: bus::view::Cpu,
}

impl Bus {
    /// Constructs a new `Bus`.
    #[must_use]
    pub fn new(bus: bus::view::Cpu) -> Self {
        Self { bus }
    }

    /// Gets the underlying bus view.
    #[cfg(test)]
    pub(crate) fn view(&mut self) -> &mut bus::view::Cpu {
        &mut self.bus
    }

    /// Read the byte at the given address.
    #[must_use]
    pub fn read(&self, addr: u16) -> u8 {
        self.bus
            .read(addr)
            .inspect_err(|err| warn!("failed to read [${addr:04x}] (default: `0xff`): {err}"))
            .unwrap_or(0xff)
    }

    /// Write to the byte at the given address.
    pub fn write(&mut self, addr: u16, data: u8) {
        let _ = self
            .bus
            .write(addr, data)
            .inspect_err(|err| warn!("failed to write [${addr:04x}] <- {data:#04x}: {err}"));
    }
}
