//! _Game Boy_.

use std::io::{BufRead, Write};
use std::marker::PhantomData;

use log::warn;
use rugby_arch::Block;
use rugby_arch::reg::Port;

use self::pcb::Motherboard;
use self::soc::cpu::Cpu;
use self::soc::joy::Joypad;
use self::soc::ppu::Ppu;
use crate::api::audio::{Audio, Chiptune};
use crate::api::cable::Cable;
use crate::api::input::{Event, Input};
use crate::api::video::{Aspect, Video};
use crate::rev::Revision;

pub mod bus;
#[cfg(feature = "debug")]
pub mod dbg;
pub mod pcb;
pub mod soc;

pub mod boot;
pub mod rev;

use crate::cart::Cartridge;

/// Clock frequency.
///
/// Crystal oscillator frequency of 4 KiHz.
#[expect(clippy::doc_markdown)]
pub const CLOCK: u32 = 4_194_304;

/// _Game Boy_.
#[derive(Debug, Default)]
pub struct GameBoy<R: Revision = rev::C> {
    /// DMG-01 Motherboard.
    #[cfg(feature = "debug")]
    pub main: Motherboard,
    #[cfg(not(feature = "debug"))]
    main: Motherboard,
    /// Revision marker.
    _rev: PhantomData<R>,
}

/// Revision-specific hardware model.
trait Instance {
    /// Simulate the bootup sequence.
    fn boot(&mut self);
}

#[rustfmt::skip]
impl Instance for GameBoy<rev::Zero> {
    fn boot(&mut self) {
        let cpu = &mut self.main.soc.cpu;

        // Initialize registers
        #[expect(clippy::items_after_statements)]
        type Select = <Cpu as Port<u16>>::Select;
        cpu.store(Select::AF, 0x0100_u16);
        cpu.store(Select::BC, 0xff13_u16);
        cpu.store(Select::DE, 0x00c1_u16);
        cpu.store(Select::HL, 0x8403_u16);
        cpu.store(Select::SP, 0xfffe_u16);

        // Perform bootup sequence
        cpu.write(0xff40, 0x91); // enable display
        cpu.write(0xff50, 0x01); // disable boot ROM
        cpu.exec(0xfb);          // enable interrupts
        cpu.goto(0x0100);        // transfer program control
    }
}

#[rustfmt::skip]
impl Instance for GameBoy<rev::A> {
    fn boot(&mut self) {
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
}

impl<R: Revision> GameBoy<R> {
    /// Constructs a new `GameBoy`.
    #[must_use]
    #[expect(private_bounds)]
    pub fn new() -> Self
    where
        Self: Instance,
    {
        let mut this = Self::default();
        this.boot();
        this
    }

    /// Constructs a new `GameBoy`, initialized with the provided boot ROM.
    #[must_use]
    pub fn with(boot: boot::Boot) -> Self {
        let mut this = Self::default();

        // Initialize boot ROM
        this.main.soc.boot.insert(boot::Chip::new(boot));

        this
    }

    /// Simulate the bootup sequence.
    ///
    /// This prepares the `GameBoy` to run the contents of a game cartridge.
    /// When no [boot ROM](boot::Boot) is installed, this must be called before
    /// cartridge execution.
    #[expect(private_bounds)]
    pub fn boot(&mut self)
    where
        Self: Instance,
    {
        <Self as Instance>::boot(self);
    }

    /// Gets the inserted game cartridge, if any.
    #[must_use]
    pub fn cart(&self) -> Option<Cartridge> {
        self.main.cart.get()
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
        self.main.cart.insert(cart);
    }

    /// Ejects the inserted game cartridge, if any.
    pub fn eject(&mut self) -> Option<Cartridge> {
        self.main.cart.eject()
    }
}

impl<R: Revision> Block for GameBoy<R>
where
    GameBoy<R>: Instance,
{
    fn ready(&self) -> bool {
        self.main.ready()
    }

    fn cycle(&mut self) {
        self.main.cycle();
    }

    fn reset(&mut self) {
        self.main.reset();
        if self.main.soc.boot.exists() {
            self.main.soc.boot.reset();
        } else {
            self.boot();
        }
        self.main.cart.reset();
    }
}

impl<R: Revision> Audio for GameBoy<R>
where
    GameBoy<R>: Instance,
{
    fn sample(&self) -> Chiptune {
        self.main.soc.apu.sample()
    }
}

impl<R: Revision> Cable for GameBoy<R>
where
    GameBoy<R>: Instance,
{
    fn rx(&mut self) -> &mut dyn BufRead {
        self.main.soc.sio.rx()
    }

    fn tx(&mut self) -> &mut dyn Write {
        self.main.soc.sio.tx()
    }
}

impl<R: Revision> Input for GameBoy<R>
where
    GameBoy<R>: Instance,
{
    type Button = <Joypad as Input>::Button;

    fn recv(&mut self, events: impl IntoIterator<Item = Event<Self::Button>>) {
        self.main.soc.joy.recv(events);
    }
}

impl<R: Revision> Video for GameBoy<R>
where
    GameBoy<R>: Instance,
{
    const SIZE: Aspect = Ppu::SIZE;

    type Pixel = <Ppu as Video>::Pixel;

    fn vsync(&self) -> bool {
        self.main.soc.ppu.vsync()
    }

    fn frame(&self) -> &[Self::Pixel] {
        self.main.soc.ppu.frame()
    }
}

#[cfg(test)]
mod tests;
