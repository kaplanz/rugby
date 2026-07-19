//! Boot ROM.

use std::fmt::Debug;

use log::{debug, trace};
use rugby_arch::mem::{Error, Memory, Result};
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

/// Boot ROM.
pub type Boot = rugby_arch::mem::Rom<[u8; 0x100]>;

/// Boot ROM slot.
///
/// Models the boot ROM's location within the memory map. While mapped, the
/// slot overlays the inserted boot ROM at `$0000..=$00FF`, with its disable
/// register at `$FF50`. An empty slot leaves both unmapped.
#[derive(Clone, Debug, Default)]
pub struct Slot(Shared<Option<Chip>>);

impl Slot {
    /// Constructs a new, empty `Slot`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if a boot ROM is present.
    #[must_use]
    pub fn exists(&self) -> bool {
        self.0.borrow().is_some()
    }

    /// Checks if the boot ROM is mapped.
    ///
    /// Only true while a boot ROM is present and not yet disabled.
    #[must_use]
    pub fn ready(&self) -> bool {
        self.0
            .borrow()
            .as_ref()
            .is_some_and(|chip| chip.mem.borrow().ready())
    }

    /// Gets the inserted boot ROM, if any.
    #[must_use]
    pub fn get(&self) -> Option<Chip> {
        self.0.borrow().clone()
    }

    /// Inserts a boot ROM into the slot.
    pub fn insert(&mut self, chip: Chip) {
        *self.0.borrow_mut() = Some(chip);
    }
}

impl Block for Slot {
    fn reset(&mut self) {
        if let Some(chip) = self.0.borrow_mut().as_mut() {
            chip.reset();
        }
    }
}

impl Memory for Slot {
    fn read(&self, addr: u16) -> Result<u8> {
        self.0
            .borrow()
            .as_ref()
            .map_or(Err(Error::Range), |chip| match addr {
                0x0000..=0x00ff => chip.mem.read(addr),
                0xff50 => chip.reg.read(addr),
                _ => Err(Error::Range),
            })
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<()> {
        self.0
            .borrow_mut()
            .as_mut()
            .map_or(Err(Error::Range), |chip| match addr {
                0x0000..=0x00ff => chip.mem.write(addr, data),
                0xff50 => chip.reg.write(addr, data),
                _ => Err(Error::Range),
            })
    }
}

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
