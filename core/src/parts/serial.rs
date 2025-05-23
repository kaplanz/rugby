//! Serial interface.

use std::collections::VecDeque;
use std::io::{BufRead, Write};

use log::{debug, trace};
use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::reg::{Port, Register};
use rugby_arch::{Block, Shared};

use super::pic::{self, Interrupt};
use crate::api::part::serial::Serial as Api;

/// Serial register select.
///
/// See more details [here][serial].
///
/// [serial]: https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html
#[derive(Clone, Copy, Debug)]
pub enum Select {
    /// `[$FF01]`: Serial transfer data.
    ///
    /// Holds the transfer data.
    ///
    /// Before a transfer, it holds the data to be transferred out. During the
    /// transfer bits are shifted leftwards. The bit shifted out is transferred
    /// serially over the wire, and the received bit is shifting in to the least
    /// significant position.
    Sb,
    /// `[$FF02]`: Serial transfer control.
    ///
    /// | Bit | Name                |
    /// |-----|---------------------|
    /// | 7   | Transfer start flag |
    /// | 0   | Shift clock         |
    Sc,
}

/// Serial communications port.
#[derive(Debug)]
pub struct Serial {
    /// Serial registers.
    pub reg: Control,
    /// Serial internals.
    pub etc: Internal,
    /// Interrupt line.
    pub int: pic::Line,
}

/// Serial internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// In-progress byte.
    ip: u8,
    /// Received queue.
    rx: VecDeque<u8>,
    /// Transmitted queue.
    tx: VecDeque<u8>,
}

impl Internal {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Serial {
    /// Perform a tick of the external clock.
    pub fn tick(&mut self) {
        // Only tick if transferring on external clock
        let sc = self.reg.sc.borrow();
        if sc.ena && !sc.clk {
            drop(sc); // release borrow on `self`

            // Perform a cycle
            self.cycle();
        }
    }

    /// Shift-exchange, simultaneously shifting a bit out and in.
    fn tex(&mut self, rx: bool) -> bool {
        // Extract control registers
        let mut sb = self.reg.sb.borrow_mut();
        let mut sc = self.reg.sc.borrow_mut();
        // Load data
        let mut data = sb.load();
        // Perform transfer
        let tx = data & 0x80 != 0;
        data = (data << 1) | (u8::from(rx));
        trace!("tx: {}, rx: {}", u8::from(tx), u8::from(rx));
        // Update bitmask
        sc.bit >>= 1;
        // Store data
        sb.store(data);
        // Return output
        tx
    }
}

impl Api for Serial {
    fn rx(&mut self) -> &mut impl BufRead {
        // Return `tx`, since the internal transmitter is the external receiver.
        &mut self.etc.tx
    }

    fn tx(&mut self) -> &mut impl Write {
        // Return `rx`, since the internal receiver is the external transmitter.
        &mut self.etc.rx
    }
}

impl Block for Serial {
    fn ready(&self) -> bool {
        // Only enable if transferring on internal clock
        let ena = self.reg.sc.borrow().ena;
        let clk = self.reg.sc.borrow().clk;
        ena && clk
    }

    fn cycle(&mut self) {
        // Extract bitmask
        let bit = self.reg.sc.borrow().bit;
        // Determine receiving bit
        let rx = self.etc.rx.front().is_none_or(|rx| rx & bit != 0);

        // Perform transfer-exchange
        let tx = self.tex(rx);

        // Store transferred bit
        let tx = if tx { 0xff } else { 0x00 };
        self.etc.ip |= tx & bit;

        // Clean-up after transfer is complete
        let mut sc = self.reg.sc.borrow_mut();
        if sc.bit == 0 {
            // Transfer out byte
            let tx = std::mem::take(&mut self.etc.ip);
            self.etc.tx.push_back(tx);
            // Mark as complete
            sc.ena = false;
            debug!("finished tx: {tx:#04x}");
            // Request an interrupt
            self.int.raise(Interrupt::Serial);
        }
    }

    fn reset(&mut self) {
        self.reg.reset();
        self.etc.reset();
    }
}

impl Mmio for Serial {
    fn attach(&self, bus: &mut Bus) {
        self.reg.attach(bus);
    }
}

impl Port<u8> for Serial {
    type Select = Select;

    fn load(&self, reg: Self::Select) -> u8 {
        match reg {
            Select::Sb => self.reg.sb.load(),
            Select::Sc => self.reg.sc.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: u8) {
        match reg {
            Select::Sb => self.reg.sb.store(value),
            Select::Sc => self.reg.sc.store(value),
        }
    }
}

/// Serial registers.
///
/// | Address | Size | Name | Description             |
/// |:-------:|------|------|-------------------------|
/// | `$FF01` | Byte | SB   | Serial transfer data    |
/// | `$FF02` | Byte | SC   | Serial transfer control |
#[derive(Debug, Default)]
pub struct Control {
    /// Serial transfer data.
    pub sb: Shared<reg::Sb>,
    /// Serial transfer control.
    pub sc: Shared<reg::Sc>,
}

impl Block for Control {
    fn reset(&mut self) {
        self.sb.take();
        self.sc.take();
    }
}

impl Mmio for Control {
    fn attach(&self, bus: &mut Bus) {
        bus.map(0xff01..=0xff01, self.sb.clone().into());
        bus.map(0xff02..=0xff02, self.sc.clone().into());
    }
}

/// Serial register models.
pub mod reg {
    use log::{debug, warn};
    use rugby_arch::mem::Memory;
    use rugby_arch::reg::Register;

    /// Serial data.
    pub type Sb = u8;

    /// Serial control.
    #[derive(Debug, Default)]
    pub struct Sc {
        pub(super) ena: bool,
        pub(super) clk: bool,
        pub(super) bit: u8,
    }

    impl Memory for Sc {
        fn read(&self, _: u16) -> rugby_arch::mem::Result<u8> {
            Ok(self.load())
        }

        fn write(&mut self, _: u16, data: u8) -> rugby_arch::mem::Result<()> {
            self.store(data);
            Ok(())
        }
    }

    impl Register for Sc {
        type Value = u8;

        fn load(&self) -> Self::Value {
            (u8::from(self.ena) << 7) | 0x7e | u8::from(self.clk)
        }

        fn store(&mut self, value: Self::Value) {
            if self.bit != 0 {
                warn!("interrupted serial transfer");
            }
            // Store individual bits
            self.ena = value & 0x80 != 0;
            self.clk = value & 0x01 != 0;
            // Reset transfer sequence bit
            self.bit = 0b1000_0000;
            debug!("started tx");
        }
    }
}
