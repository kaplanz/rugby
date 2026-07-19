//! Bus master views.

use rugby_arch::Shared;
use rugby_arch::mem::Memory;

use super::File;
use crate::cart;
use crate::dmg::boot;
use crate::dmg::chip::{apu, cpu, ppu};
use crate::dmg::pcb::{Vram, Wram};

/// CPU memory view.
///
/// |     Address     |  Size  | Device | Description   |
/// |:---------------:|-------:|--------|---------------|
/// | `$0000..=$00FF` |  256 B | `boot` | Boot ROM      |
/// | `$0000..=$7FFF` | 32 KiB | `cart` | Cartridge ROM |
/// | `$8000..=$9FFF` |  8 KiB | `ppu`  | Video RAM     |
/// | `$A000..=$BFFF` |  8 KiB | `cart` | External RAM  |
/// | `$C000..=$DFFF` |  8 KiB | `wram` | Work RAM      |
/// | `$E000..=$FDFF` | 7680 B | `wram` | Echo RAM      |
/// | `$FE00..=$FE9F` |  160 B | `ppu`  | Object memory |
/// | `$FF00..=$FF7F` |  128 B | `io`   | I/O registers |
/// | `$FF80..=$FFFE` |  127 B | `cpu`  | High RAM      |
/// | `$FFFF..=$FFFF` |    1 B | `io`   | Interrupt enable |
#[derive(Clone, Debug)]
#[derive(Memory)]
pub struct Cpu {
    /// Boot ROM.
    #[mmap(0x0000..=0x00ff, gate = ready)]
    pub boot: boot::Slot,
    /// Game cartridge.
    #[mmap(0x0000..=0x7fff)]
    #[mmap(0xa000..=0xbfff)]
    pub cart: cart::Slot,
    /// Graphics memory.
    #[mmap(0x8000..=0x9fff)]
    #[mmap(0xfe00..=0xfe9f)]
    pub ppu: ppu::Bank,
    /// Work RAM.
    #[mmap(0xc000..=0xfdff, mask = 0x1fff)]
    pub wram: Shared<Wram>,
    /// Audio memory.
    #[mmap(0xff30..=0xff3f)]
    pub apu: apu::Bank,
    /// I/O registers.
    #[mmap(0xff00..=0xff7f)]
    #[mmap(0xffff)]
    pub io: File,
    /// Processor memory.
    #[mmap(0xff80..=0xfffe)]
    pub cpu: cpu::Bank,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            boot: boot::Slot::default(),
            cart: cart::Slot::default(),
            ppu: ppu::Bank::default(),
            wram: Shared::new(Wram::from([u8::default(); 0x2000])),
            apu: apu::Bank::default(),
            io: File::default(),
            cpu: cpu::Bank::default(),
        }
    }
}

/// DMA memory view.
///
/// Only devices on the external and video buses are reachable, with work RAM
/// echoed over the full `$E000..=$FFFF`, so sources at or above `$FE00` read
/// echo RAM.
///
/// |     Address     |  Size  | Device | Description   |
/// |:---------------:|-------:|--------|---------------|
/// | `$0000..=$7FFF` | 32 KiB | `cart` | Cartridge ROM |
/// | `$8000..=$9FFF` |  8 KiB | `vram` | Video RAM     |
/// | `$A000..=$BFFF` |  8 KiB | `cart` | External RAM  |
/// | `$C000..=$DFFF` |  8 KiB | `wram` | Work RAM      |
/// | `$E000..=$FFFF` |  8 KiB | `wram` | Echo RAM      |
#[derive(Clone, Debug)]
#[derive(Memory)]
pub struct Dma {
    /// Game cartridge.
    #[mmap(0x0000..=0x7fff)]
    #[mmap(0xa000..=0xbfff)]
    pub cart: cart::Slot,
    /// Video RAM.
    #[mmap(0x8000..=0x9fff, mask = 0x1fff)]
    pub vram: Shared<Vram>,
    /// Work RAM.
    #[mmap(0xc000..=0xffff, mask = 0x1fff)]
    pub wram: Shared<Wram>,
}
