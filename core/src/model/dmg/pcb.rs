//! DMG-01 motherboard.

use log::warn;
use rugby_arch::Block;

use super::mem::Bank;
use super::noc::Mmap;
use super::soc::Chip;
use crate::api::part::proc::Processor;

/// DMG-CPU-01 PCB.
#[derive(Debug)]
pub struct Motherboard {
    /// Crystal oscillator.
    pub clk: u128,
    /// Embedded memory.
    pub mem: Bank,
    /// Network-on-chip.
    pub noc: Mmap,
    /// System-on-chip.
    pub soc: Chip,
}

impl Default for Motherboard {
    fn default() -> Self {
        // Crystal oscillator
        let clk = u128::default();
        // Embedded memory
        let mem = Bank::new();
        // Network-on-chip
        let noc = Mmap::new();
        // System-on-chip
        let soc = Chip::new(&mem, &noc);

        // Finish construction
        Self { clk, mem, noc, soc }.prep()
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
        // Wake on pending interrupt
        if !self.soc.cpu.ready() && self.soc.pic.line.pending() {
            self.soc.cpu.wake();
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
