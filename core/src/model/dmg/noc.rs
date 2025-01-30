//! Network-on-chip.

use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::Shared;

use super::pcb::Motherboard;

/// Memory-mapped I/O.
///
/// The Game Boy's memory architecture is divided across three distinct buses:
/// - [Internal](Self::ibus): Embedded within the Sharp LR35902. Usable only by
///                           the [CPU](super::cpu).
/// - [External](Self::ebus): Accessible to on-board components.
/// - [Video](Self::vbus):    Connected only to VRAM, controlled by the
///                           [PPU](super::ppu).
///
/// # Memory Map
///
/// See more details [here][map].
///
/// |     Address     |  Size  |  Module  | Description      |    Bus    |
/// |:---------------:|-------:|----------|------------------|-----------|
/// | `$0000..=$00FF` |  256 B | `boot`   | Boot ROM         | Internal  |
/// | `$0000..=$7FFF` | 32 KiB | `cart`   | Cartridge ROM    | External  |
/// | `$8000..=$9FFF` |  8 KiB | `vram`   | Video RAM        | Video     |
/// | `$A000..=$BFFF` |  8 KiB | `cart`   | External RAM     | External  |
/// | `$C000..=$DFFF` |  8 KiB | `wram`   | Work RAM         | External  |
/// | `$E000..=$FDFF` | 7680 B | `wram`   | Echo RAM         | External  |
/// | `$FE00..=$FEA0` |  160 B | `oam`    | Object memory    | Internal  |
/// | `$FEA0..=$FEFF` |   96 B | ---      | ---              | ---       |
/// | `$FF00..=$FFFF` |   96 B | `soc`    | I/O registers    | Internal  |
///
/// ## I/O Registers
///
/// |     Address     |  Size  |  Module  | Description      |    Bus    |
/// |:---------------:|-------:|----------|------------------|-----------|
/// | `$FF00..=$FF00` |    1 B | `joypad` | Joypad           | Internal  |
/// | `$FF01..=$FF02` |    2 B | `serial` | Serial           | Internal  |
/// | `$FF03..=$FF03` |    1 B | ---      | ---              | ---       |
/// | `$FF04..=$FF07` |    4 B | `timer`  | Timer            | Internal  |
/// | `$FF08..=$FF0E` |    7 B | ---      | ---              | ---       |
/// | `$FF0F..=$FF0F` |    1 B | `pic`    | Interrupt flag   | Internal  |
/// | `$FF10..=$FF26` |   23 B | `apu`    | Audio            | Internal  |
/// | `$FF27..=$FF2F` |    9 B | ---      | ---              | ---       |
/// | `$FF30..=$FF3F` |   16 B | `apu`    | Wave RAM         | Internal  |
/// | `$FF40..=$FF4B` |   12 B | `ppu`    | Graphics         | Internal  |
/// | `$FF4C..=$FF4F` |    4 B | ---      | ---              | ---       |
/// | `$FF50..=$FF50` |    1 B | `boot`   | Boot disable     | Internal  |
/// | `$FF51..=$FF7F` |   47 B | ---      | ---              | ---       |
/// | `$FF80..=$FFFE` |  127 B | `cpu`    | High RAM         | Internal  |
/// | `$FFFF..=$FFFF` |    1 B | `pic`    | Interrupt enable | Internal  |
///
/// [map]: https://gbdev.io/pandocs/Memory_Map.html
#[derive(Debug, Default)]
pub struct Mmap {
    /// Internal bus.
    pub ibus: Shared<Bus>,
    /// External bus.
    pub ebus: Shared<Bus>,
    /// Video bus.
    pub vbus: Shared<Bus>,
}

impl Mmap {
    /// Constructs a new `Network`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a bus for the CPU.
    pub(super) fn cpu(&self) -> Bus {
        Bus::from([
            (0x0000..=0x00ff, self.ibus.clone().into()), // Boot ROM
            (0x0000..=0xffff, self.ibus.clone().into()), // Internal
            (0x0000..=0xfdff, self.ebus.clone().into()), // External
            (0x0000..=0xfdff, self.vbus.clone().into()), // Video
        ])
    }

    /// Constructs a bus for the DMA.
    pub(super) fn dma(&self) -> Bus {
        Bus::from([
            (0x0000..=0xffff, self.ebus.clone().into()), // External
            (0x0000..=0xffff, self.vbus.clone().into()), // Video
        ])
    }
}

#[rustfmt::skip]
impl Motherboard {
    /// Connect memory-mapped I/O.
    pub(super) fn mmap(&self) {
        self.ibus();
        self.ebus();
        self.vbus();
    }

    /// Connect internal bus.
    fn ibus(&self) {
        let mut ibus = self.noc.ibus.borrow_mut();
        // Memory map
        ibus.map(0xfe00..=0xfe9f, self.mem.oam .clone().into()); // OAM
        ibus.map(0xff30..=0xff3f, self.mem.wave.clone().into()); // WAVE
        ibus.map(0xff80..=0xfffe, self.mem.hram.clone().into()); // HRAM
        // I/O registers
        self.soc.apu.attach(&mut ibus); // Audio
        self.soc.joy.attach(&mut ibus); // Joypad
        self.soc.pic.attach(&mut ibus); // Interrupts
        self.soc.ppu.attach(&mut ibus); // Graphics
        self.soc.sio.attach(&mut ibus); // Serial
        self.soc.tma.attach(&mut ibus); // Timer
    }

    /// Connect external bus.
    fn ebus(&self) {
        let mut ebus = self.noc.ebus.borrow_mut();
        // Memory map
        ebus.map(0xc000..=0xdfff, self.mem.wram.clone().into()); // WRAM
        ebus.map(0xe000..=0xffff, self.mem.wram.clone().into()); // ECHO
    }

    /// Connect video bus.
    fn vbus(&self) {
        let mut vbus = self.noc.vbus.borrow_mut();
        // Memory map
        vbus.map(0x8000..=0x9fff, self.mem.vram.clone().into()); // VRAM
    }
}
