//! DMG-01 motherboard.

use log::warn;
use rugby_arch::mem::Ram;
use rugby_arch::reg::Register;
use rugby_arch::{Block, Shared};

use super::chip::Chip;
use super::mmap::Mmap;
use crate::cart::Cartridge;

/// Sharp LH5164N (64K SRAM).
pub type Sram = Ram<[u8; 0x2000]>;

/// Video RAM.
///
/// 8 KiB RAM used to store tile [data][tdata] and [maps][tmaps].
///
/// [tdata]: https://gbdev.io/pandocs/Tile_Data.html
/// [tmaps]: https://gbdev.io/pandocs/Tile_Maps.html
pub type Vram = Sram;

/// Work RAM.
///
/// 8 KiB RAM used as general-purpose transient memory.
pub type Wram = Sram;

/// DMG-CPU-01 PCB.
#[derive(Debug)]
pub struct Motherboard {
    /// Crystal oscillator.
    pub clk: u128,
    /// Video RAM.
    ///
    /// Designated U2 on the PCB.
    pub vram: Shared<Vram>,
    /// Work RAM.
    ///
    /// Designated U3 on the PCB.
    pub wram: Shared<Wram>,
    /// Game cartridge.
    pub cart: Option<Cartridge>,
    /// Network-on-chip.
    pub noc: Mmap,
    /// System-on-chip.
    ///
    /// Designated U1 on the PCB.
    pub soc: Chip,
}

impl Default for Motherboard {
    fn default() -> Self {
        // Crystal oscillator
        let clk = u128::default();
        // Video RAM
        let vram = Shared::new(Vram::from([u8::default(); 0x2000]));
        // Work RAM
        let wram = Shared::new(Wram::from([u8::default(); 0x2000]));
        // Cartridge slot
        let cart = None;
        // Network-on-chip
        let noc = Mmap::new();
        // System-on-chip
        let soc = Chip::new(&vram, &noc);

        // Finish construction
        Self {
            clk,
            vram,
            wram,
            cart,
            noc,
            soc,
        }
        .prep()
    }
}

impl Motherboard {
    /// Constructs a new `Motherboard`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepares a `Motherboard`.
    #[must_use]
    fn prep(self) -> Self {
        self.mmap();
        self
    }
}

impl Block for Motherboard {
    fn ready(&self) -> bool {
        self.soc.ready()
    }

    fn cycle(&mut self) {
        // Wake halted CPU on pending interrupt
        if self.soc.cpu.halted() && self.soc.irq.line.pending() {
            self.soc.cpu.wake();
        }
        // Wake stopped CPU on joypad input
        if self.soc.cpu.stopped() && self.soc.joy.reg.load() & 0x0f != 0x0f {
            self.soc.cpu.wake();
        }
        // Halt the oscillator while stopped
        if self.soc.cpu.stopped() {
            return;
        }

        // APU: 4 MiHz
        if self.soc.apu.ready() {
            self.soc.apu.cycle();
        }
        // CPU: 1 MiHz
        if self.soc.cpu.ready() && self.clk.is_multiple_of(4) {
            self.soc.cpu.cycle();
        }
        // DMA: 1 MiHz
        if self.soc.dma.ready() && self.clk.is_multiple_of(4) {
            self.soc.dma.cycle();
        }
        // PPU: 4 MiHz
        if self.soc.ppu.ready() {
            self.soc.ppu.cycle();
        } else {
            // When the LCD is disabled, the PPU holds its reset state.
            self.soc.ppu.disable();
        }
        // Serial: 8192 Hz
        if self.soc.sio.ready() && self.clk.is_multiple_of(512) {
            self.soc.sio.cycle();
        }
        // Timer: 4 MiHz
        if self.soc.tma.ready() {
            self.soc.tma.cycle();
        }

        // Update executed cycle count
        let (clock, carry) = self.clk.overflowing_add(1);
        if carry {
            warn!("internal cycle counter overflowed; resetting");
        }
        self.clk = clock;
    }

    fn reset(&mut self) {
        self.soc.reset();
    }
}
