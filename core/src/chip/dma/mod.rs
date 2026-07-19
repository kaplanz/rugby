//! Direct memory access.

use log::{debug, trace};
use rugby_arch::mem::Memory;
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

pub use super::ppu::Oam;
use crate::dmg::bus;

/// Direct memory access unit.
#[derive(Debug)]
pub struct Dma {
    /// DMA bus.
    pub bus: bus::view::Dma,
    /// DMA register.
    pub reg: Shared<Control>,
    /// DMA memory.
    pub mem: Shared<Oam>,
}

impl Block for Dma {
    fn ready(&self) -> bool {
        !matches!(self.reg.borrow().mode, Mode::Off)
    }

    fn cycle(&mut self) {
        // Perform DMA
        let mode = match self.reg.borrow().mode {
            Mode::Off => {
                unreachable!("cannot to cycle DMA while disabled");
            }
            Mode::Req(src) => {
                // Initiate transfer
                trace!("started: 0xfe00 <- {src:#04x}00");
                Mode::On { hi: src, lo: 0xff }
            }
            Mode::On { hi, lo: 0xff } => {
                // Start delay
                Mode::On { hi, lo: 0x00 }
            }
            Mode::On { hi, mut lo } => {
                // Transfer single byte
                let addr = u16::from_be_bytes([hi, lo]);
                let data = self.bus.read(addr).unwrap_or(0xff);
                self.mem.write(lo as u16, data).unwrap();
                trace!("copied: $fe{lo:02x} <- ${addr:04x}, data: {data:#04x}");
                // Increment transfer index
                lo += 1;
                if usize::from(lo) < self.mem.borrow().inner().len() {
                    Mode::On { hi, lo }
                } else {
                    // Complete transfer
                    debug!("finished: 0xfe00 <- {hi:#04x}00");
                    Mode::Off
                }
            }
        };
        // Transition state machine
        self.reg.borrow_mut().mode = mode;
    }

    fn reset(&mut self) {
        self.reg.reset();
    }
}

/// DMA control register.
#[derive(Debug, Default)]
pub struct Control {
    /// DMA progress.
    mode: Mode,
    /// DMA source page.
    page: u8,
}

impl Block for Control {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Memory for Control {
    fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Control {
    type Value = u8;

    fn load(&self) -> Self::Value {
        self.page.load()
    }

    fn store(&mut self, value: Self::Value) {
        // Request a new transfer
        //
        // NOTE: Writes during an active transfer restart it from the newly
        //       written source page.
        self.mode = Mode::Req(value);
        debug!("request: 0xfe00 <- {value:#04x}00");
        // Always update stored value
        self.page.store(value);
    }
}

/// DMA transfer mode.
#[derive(Debug, Default)]
enum Mode {
    /// Disabled.
    #[default]
    Off,
    /// Requested.
    Req(u8),
    /// In-progress.
    On { hi: u8, lo: u8 },
}
