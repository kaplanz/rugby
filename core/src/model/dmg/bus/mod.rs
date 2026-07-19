//! Memory map.
//!
//! The Game Boy's memory architecture is divided across three distinct
//! buses:
//!
//! - [Internal](Ibus): Embedded within Sharp LR35902 CPU.
//! - [External](Ebus): Accessible to on-board components.
//! - [Graphics](Vbus): Connected only to VRAM.
//!
//! Each bus driver owns a view of the buses it can reach, with address decoding
//! compiled to a match. Views decode into the buses below, so traffic claimed
//! by an earlier stage never appears on a later one.
//!
//! See more details [here][mmap].
//!
//! ```text
//!                   CPU                                DMA
//!                    │                                  │
//!      ┌─────────────┼─────────────┐             ┌──────┴──────┐
//!      │             │             │             │             │
//!   ╭──▼───╮      ╭──▼───╮      ╭──▼───╮      ╭──▼───╮      ╭──▼───╮
//!   │ ibus │      │ vbus │      │ ebus │      │ vbus │      │ ebus │
//!   ╰──────╯      ╰──────╯      ╰──────╯      ╰──────╯      ╰──────╯
//! $0000-$00FF   $8000-$9FFF   $0000-$FDFF   $8000-$9FFF   $0000-$FFFF
//! $FE00-$FFFF
//! ```
//!
//! [mmap]: https://gbdev.io/pandocs/Memory_Map.html

use rugby_arch::Shared;
use rugby_arch::mem::Memory;

use super::boot;
use super::pcb::{Vram, Wram};
use super::soc::{apu, cpu, dma, irq, joy, ppu, sio, tma};
use crate::cart;

pub mod view;

/// Internal bus.
///
/// Embedded within the LR35902, usable only by the CPU.
///
/// |     Address     |  Size  | Device | Description      |
/// |:---------------:|-------:|--------|------------------|
/// | `$0000..=$00FF` |  256 B | `boot` | Boot ROM         |
/// | `$FE00..=$FE9F` |  160 B | `ppu`  | Object memory    |
/// | `$FF00..=$FF7F` |  128 B | `io`   | I/O registers    |
/// | `$FF30..=$FF3F` |   16 B | `apu`  | Wave RAM         |
/// | `$FF80..=$FFFE` |  127 B | `cpu`  | High RAM         |
/// | `$FFFF..=$FFFF` |    1 B | `io`   | Interrupt enable |
#[derive(Debug, Default)]
#[derive(Memory)]
pub struct Ibus {
    /// Boot ROM.
    #[mmap(0x0000..=0x00ff)]
    pub boot: boot::Slot,
    /// Graphics memory.
    #[mmap(0xfe00..=0xfe9f)]
    pub ppu: ppu::Bank,
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

impl Ibus {
    /// Checks whether the boot ROM overlay is mapped.
    #[must_use]
    pub fn ready(&self) -> bool {
        self.boot.ready()
    }
}

/// External bus.
///
/// Accessible to on-board components.
///
/// # Note
///
/// Work RAM decodes over all of `$C000..=$FFFF`. Bit `A13` is ignored, so the
/// upper lower half is mirrored, resulting in a copy known as echo RAM.
///
/// |     Address     |  Size  | Device | Description   |
/// |:---------------:|-------:|--------|---------------|
/// | `$0000..=$7FFF` | 32 KiB | `cart` | Cartridge ROM |
/// | `$A000..=$BFFF` |  8 KiB | `cart` | External RAM  |
/// | `$C000..=$FFFF` |  8 KiB | `wram` | Work/Echo RAM |
#[derive(Debug)]
#[derive(Memory)]
pub struct Ebus {
    /// Game cartridge.
    #[mmap(0x0000..=0x7fff)]
    #[mmap(0xa000..=0xbfff)]
    pub cart: cart::Slot,
    /// Work RAM.
    #[mmap(0xc000..=0xffff, mask = 0x1fff)]
    pub wram: Shared<Wram>,
}

impl Default for Ebus {
    fn default() -> Self {
        Self {
            cart: cart::Slot::default(),
            wram: Shared::new(Wram::from([u8::default(); 0x2000])),
        }
    }
}

/// Video bus.
///
/// Connected only to VRAM.
///
/// |     Address     |  Size  | Device | Description |
/// |:---------------:|-------:|--------|-------------|
/// | `$8000..=$9FFF` |  8 KiB | `vram` | Video RAM   |
#[derive(Debug)]
#[derive(Memory)]
pub struct Vbus {
    /// Video RAM.
    #[mmap(0x8000..=0x9fff, mask = 0x1fff)]
    pub vram: Shared<Vram>,
}

impl Default for Vbus {
    fn default() -> Self {
        Self {
            vram: Shared::new(Vram::from([u8::default(); 0x2000])),
        }
    }
}

/// I/O registers.
///
/// |     Address     |  Size  | Device | Description      |
/// |:---------------:|-------:|--------|------------------|
/// | `$FF00..=$FF00` |    1 B | `joy`  | Joypad           |
/// | `$FF01..=$FF02` |    2 B | `sio`  | Serial           |
/// | `$FF04..=$FF07` |    4 B | `tma`  | Timer            |
/// | `$FF0F..=$FF0F` |    1 B | `irq`  | Interrupt flag   |
/// | `$FF10..=$FF26` |   23 B | `apu`  | Audio            |
/// | `$FF40..=$FF4B` |   12 B | `ppu`  | Graphics         |
/// | `$FF46..=$FF46` |    1 B | `dma`  | OAM DMA          |
/// | `$FF50..=$FF50` |    1 B | `boot` | Boot disable     |
/// | `$FFFF..=$FFFF` |    1 B | `irq`  | Interrupt enable |
#[derive(Debug, Default)]
#[derive(Memory)]
pub struct File {
    /// Joypad.
    #[mmap(0xff00)]
    pub joy: Shared<joy::Control>,
    /// Serial.
    #[mmap(0xff01..=0xff02)]
    pub sio: sio::File,
    /// Timer.
    #[mmap(0xff04..=0xff07)]
    pub tma: tma::File,
    /// Interrupts.
    #[mmap(0xff0f)]
    #[mmap(0xffff)]
    pub irq: irq::File,
    /// Audio.
    #[mmap(0xff10..=0xff26)]
    pub apu: apu::File,
    /// OAM DMA.
    #[mmap(0xff46)]
    pub dma: Shared<dma::Control>,
    /// Graphics.
    #[mmap(0xff40..=0xff4b)]
    pub ppu: ppu::File,
    /// Boot disable.
    #[mmap(0xff50, gate = exists)]
    pub boot: boot::Slot,
}
