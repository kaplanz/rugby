//! Direct memory access.

use log::{debug, trace, warn};
use rugby_arch::mem::Memory;
use rugby_arch::mio::Bus;
use rugby_arch::reg::Register;
use rugby_arch::{Block, Byte, Shared, Word};

pub use super::ppu::Oam;
use crate::dmg::Mmap;

/// Direct memory access unit.
#[derive(Debug)]
pub struct Dma {
    /// DMA bus.
    pub bus: Bus,
    /// DMA register.
    pub reg: Shared<Control>,
    /// DMA memory.
    pub mem: Shared<Oam>,
    /// Network-on-chip.
    pub noc: Mmap,
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
                Mode::On { hi: src, lo: 0x00 }
            }
            Mode::On { hi, lo } => {
                // Free system buses
                self.noc.ebus.borrow_mut().free();
                self.noc.vbus.borrow_mut().free();
                // Transfer single byte
                let addr = u16::from_be_bytes([hi, lo]);
                let data = self.bus.read(addr).unwrap_or(0xff);
                self.mem.write(lo as Word, data).unwrap();
                trace!("copied: $fe{lo:02x} <- ${addr:04x}, data: {data:#04x}");
                // Increment transfer index
                let lo = lo.saturating_add(1);
                if usize::from(lo) < self.mem.borrow().inner().len() {
                    // Lock system buses
                    self.noc.ebus.borrow_mut().busy();
                    self.noc.vbus.borrow_mut().busy();
                    // Continue transfer
                    Mode::On { hi, lo }
                } else {
                    // Complete transfer
                    debug!("finished: 0xfe00 <- {hi:#04x}00");
                    Mode::Off
                }
            }
        };
        // Determine next mode
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
    page: Byte,
}

impl Block for Control {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Memory for Control {
    fn read(&self, _: Word) -> rugby_arch::mem::Result<Byte> {
        Ok(self.load())
    }

    fn write(&mut self, _: Word, data: Byte) -> rugby_arch::mem::Result<()> {
        self.store(data);
        Ok(())
    }
}

impl Register for Control {
    type Value = Byte;

    fn load(&self) -> Self::Value {
        self.page.load()
    }

    fn store(&mut self, value: Self::Value) {
        match self.mode {
            Mode::Off => {
                // Request a new transfer
                self.mode = Mode::Req(value);
                debug!("request: 0xfe00 <- {value:#04x}00");
            }
            Mode::Req(_) | Mode::On { .. } => {
                warn!("ignored request; already in progress");
            }
        }
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
    Req(Byte),
    /// In-progress.
    On { hi: Byte, lo: Byte },
}
