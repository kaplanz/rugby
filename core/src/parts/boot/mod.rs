//! Boot ROM.

use std::fmt::Debug;

use log::{debug, trace};
use rugby_arch::mem::{Error, Memory, Result};
use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

/// Boot ROM.
pub type Boot = rugby_arch::mem::Rom<[u8; 0x100]>;

/// Boot mapper chip.
#[derive(Clone, Debug)]
pub struct Chip {
    /// Boot disable.
    pub reg: Shared<Control>,
    /// Boot bank.
    pub mem: Shared<Bank>,
}

impl Chip {
    /// Constructs a new `Rom`.
    #[must_use]
    pub fn new(rom: Boot) -> Self {
        trace!("boot ROM:\n{}", hexd::Printer::<u8>::new(0, rom.inner()));
        let reg = Shared::new(Control::new());
        Self {
            mem: Shared::new(Bank::new(reg.clone(), rom)),
            reg,
        }
    }
}

impl Block for Chip {
    fn reset(&mut self) {
        self.reg.reset();
    }
}

impl Mmio for Chip {
    fn attach(&self, bus: &mut Bus) {
        bus.map(0x0000..=0x00ff, self.mem.clone().into());
        bus.map(0xff50..=0xff50, self.reg.clone().into());
    }
}

/// Boot disable register.
#[derive(Debug, Default)]
pub struct Control(bool);

impl Control {
    /// Constructs a new `Control`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Block for Control {
    fn ready(&self) -> bool {
        !self.0
    }

    fn reset(&mut self) {
        std::mem::take(&mut self.0);
    }
}

impl Memory for Control {
    fn read(&self, _: u16) -> Result<u8> {
        Ok(self.load())
    }

    fn write(&mut self, _: u16, value: u8) -> Result<()> {
        self.store(value);
        Ok(())
    }
}

impl Register for Control {
    type Value = u8;

    fn load(&self) -> Self::Value {
        0xfe | u8::from(self.0)
    }

    #[rustfmt::skip]
    fn store(&mut self, value: Self::Value) {
        let enabled = self.ready();       // is boot enabled?
        let disable = value & 0x01 != 0;  // disable request?

        // Disable cannot be undone (other than a reset). This is implemented in
        // the hardware through a feedback loop on this register's output back
        // to its input via an OR gate.
        self.0 |= disable;
        if enabled && disable {
            debug!("disabled boot");
        }
    }
}
/// Boot memory bank.
#[derive(Clone, Debug)]
pub struct Bank {
    /// Boot ROM.
    pub boot: Boot,
    /// Boot disable.
    reg: Shared<Control>,
}

impl Bank {
    /// Constructs a new `Rom`.
    fn new(reg: Shared<Control>, boot: Boot) -> Self {
        Self { boot, reg }
    }
}

impl Block for Bank {
    fn ready(&self) -> bool {
        self.reg.ready()
    }
}

impl Memory for Bank {
    fn read(&self, addr: u16) -> Result<u8> {
        if self.ready() {
            self.boot.read(addr)
        } else {
            Err(Error::Disabled)
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        if self.ready() {
            self.boot.write(addr, data)
        } else {
            Err(Error::Disabled)
        }
    }
}
