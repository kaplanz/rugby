//! Memory map.
//!
//! The Game Boy's memory map is declared statically at compile time. Each bus
//! master owns a view over the devices it can reach, with address decoding
//! compiled to a match. See more details [here][map].
//!
//! [map]: https://gbdev.io/pandocs/Memory_Map.html

use rugby_arch::Shared;
use rugby_arch::mem::Memory;

use super::boot;
use super::chip::{apu, dma, irq, joy, ppu, sio, tma};

pub mod view;

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
#[derive(Clone, Debug, Default)]
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
