//! DMG: [Game Boy].
//!
//! [Game Boy]: https://en.wikipedia.org/wiki/Game_Boy

use std::marker::PhantomData;

use log::warn;
use rugby_arch::Block;
use rugby_arch::mio::Mmio;
use rugby_arch::reg::Port;

use self::chip::apu::Apu;
use self::chip::cpu::Cpu;
use self::chip::joy::Joypad;
use self::chip::ppu::Ppu;
use self::chip::sio::Serial;
use self::pcb::Motherboard;
use crate::api::core::{self, Core};
use crate::api::part::proc::Processor;
use crate::rev::Revision;

pub mod chip;
#[cfg(feature = "debug")]
pub mod dbg;
pub mod mmap;
pub mod pcb;

pub mod boot;
pub mod rev;

use crate::cart::Cartridge;

/// Clock frequency.
///
/// Crystal oscillator frequency of 4 KiHz.
#[expect(clippy::doc_markdown)]
pub const CLOCK: u32 = 4_194_304;

/// Game Boy handheld game console.
#[derive(Debug, Default)]
pub struct GameBoy<R: Revision = rev::C> {
    /// Boot ROM.
    boot: Option<boot::Chip>,
    /// Game cartridge.
    cart: Option<Cartridge>,
    /// DMG-01 Motherboard.
    #[cfg(feature = "debug")]
    pub main: Motherboard,
    #[cfg(not(feature = "debug"))]
    main: Motherboard,
    /// Revision marker.
    _rev: PhantomData<R>,
}

impl<R: Revision> GameBoy<R> {
    /// Constructs a new `GameBoy`.
    #[must_use]
    pub fn new() -> Self {
        let mut this = Self::default();

        // Simulate bootup sequence
        this.boot();

        this
    }

    /// Constructs a new `GameBoy`, initialized with the provided boot ROM.
    #[must_use]
    pub fn with(boot: boot::Boot) -> Self {
        let mut this = Self::default();

        // Initialize boot ROM
        let boot = boot::Chip::new(boot);
        boot.attach(&mut this.main.noc.ibus.borrow_mut());
        this.boot = Some(boot);

        this
    }

    /// Simulate the bootup sequence.
    ///
    /// This prepares the `GameBoy` to run the contents of a game cartridge.
    /// When no [boot ROM](boot::Boot) is installed, this must be called before
    /// cartridge execution.
    #[rustfmt::skip]
    pub fn boot(&mut self) {
        let cpu = &mut self.main.soc.cpu;

        // Initialize registers
        #[expect(clippy::items_after_statements)]
        type Select = <Cpu as Port<u16>>::Select;
        cpu.store(Select::AF, 0x01b0_u16);
        cpu.store(Select::BC, 0x0013_u16);
        cpu.store(Select::DE, 0x00d8_u16);
        cpu.store(Select::HL, 0x014d_u16);
        cpu.store(Select::SP, 0xfffe_u16);

        // Perform bootup sequence
        cpu.write(0xff40, 0x91); // enable display
        cpu.write(0xff50, 0x01); // disable boot ROM
        cpu.exec(0xfb);          // enable interrupts
        cpu.goto(0x0100);        // transfer program control
    }

    /// Gets the inserted game cartridge, if any.
    #[must_use]
    pub fn cart(&self) -> Option<&Cartridge> {
        self.cart.as_ref()
    }

    /// Inserts a game cartridge.
    ///
    /// If a cartridge is already inserted, it will first be
    /// [ejected](Self::eject).
    pub fn insert(&mut self, cart: Cartridge) {
        // Disconnect previous cartridge
        if let Some(cart) = self.eject() {
            warn!("ejected previous cartridge: {}", cart.header());
        }
        // Insert supplied cartridge
        let ebus = &mut *self.main.noc.ebus.borrow_mut();
        cart.attach(ebus);
        self.cart = Some(cart);
    }

    /// Ejects the inserted game cartridge, if any.
    pub fn eject(&mut self) -> Option<Cartridge> {
        // Disconnect from bus
        let ebus = &mut *self.main.noc.ebus.borrow_mut();
        if let Some(cart) = &self.cart {
            cart.detach(ebus);
        }
        // Remove inserted cartridge
        self.cart.take()
    }
}

impl<R: Revision> Block for GameBoy<R> {
    fn ready(&self) -> bool {
        self.main.ready()
    }

    fn cycle(&mut self) {
        self.main.cycle();
    }

    #[rustfmt::skip]
    fn reset(&mut self) {
        self.main.reset();
        self.boot.as_mut().map(Block::reset).unwrap_or_else(|| self.boot());
        self.cart.as_mut().map(Block::reset);
    }
}

impl<R: Revision> Core for GameBoy<R> {}

impl<R: Revision> core::has::Audio for GameBoy<R> {
    type Audio = Apu;

    fn audio(&self) -> &Self::Audio {
        &self.main.soc.apu
    }

    fn audio_mut(&mut self) -> &mut Self::Audio {
        &mut self.main.soc.apu
    }
}

impl<R: Revision> core::has::Processor for GameBoy<R> {
    type Proc = Cpu;

    fn proc(&self) -> &Self::Proc {
        &self.main.soc.cpu
    }

    fn proc_mut(&mut self) -> &mut Self::Proc {
        &mut self.main.soc.cpu
    }
}

impl<R: Revision> core::has::Joypad for GameBoy<R> {
    type Joypad = Joypad;

    fn joypad(&self) -> &Self::Joypad {
        &self.main.soc.joy
    }

    fn joypad_mut(&mut self) -> &mut Self::Joypad {
        &mut self.main.soc.joy
    }
}

impl<R: Revision> core::has::Serial for GameBoy<R> {
    type Serial = Serial;

    fn serial(&self) -> &Self::Serial {
        &self.main.soc.sio
    }

    fn serial_mut(&mut self) -> &mut Self::Serial {
        &mut self.main.soc.sio
    }
}

impl<R: Revision> core::has::Video for GameBoy<R> {
    type Video = Ppu;

    fn video(&self) -> &Self::Video {
        &self.main.soc.ppu
    }

    fn video_mut(&mut self) -> &mut Self::Video {
        &mut self.main.soc.ppu
    }
}

#[cfg(test)]
mod tests;
