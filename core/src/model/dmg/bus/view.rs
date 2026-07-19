//! Bus driver views.

use rugby_arch::mem::Memory;

use super::{Ebus, Ibus, Vbus};

/// CPU memory view.
///
/// The internal bus decodes first, claiming the boot overlay and the entire
/// `$FE00..=$FFFF`, so internal traffic never reaches the external bus.
///
/// |     Address     | Bus    | Description       |
/// |:---------------:|--------|-------------------|
/// | `$0000..=$00FF` | `ibus` | Boot ROM (overlay)|
/// | `$0000..=$7FFF` | `ebus` | Cartridge ROM     |
/// | `$8000..=$9FFF` | `vbus` | Video RAM         |
/// | `$A000..=$BFFF` | `ebus` | External RAM      |
/// | `$C000..=$FDFF` | `ebus` | Work/Echo RAM     |
/// | `$FE00..=$FFFF` | `ibus` | SoC internal      |
#[derive(Debug, Default)]
#[derive(Memory)]
pub struct Cpu {
    /// Internal bus.
    #[mmap(0x0000..=0x00ff, gate = ready)]
    #[mmap(0xfe00..=0xffff)]
    pub ibus: Ibus,
    /// Video bus.
    #[mmap(0x8000..=0x9fff)]
    pub vbus: Vbus,
    /// External bus.
    #[mmap(0x0000..=0xfdff)]
    pub ebus: Ebus,
}

/// DMA memory view.
///
/// Without a view of the internal bus, sources at or above `$FE00` resolve
/// through the external bus to echo RAM.
///
/// |     Address     | Bus    | Description   |
/// |:---------------:|--------|---------------|
/// | `$0000..=$7FFF` | `ebus` | Cartridge ROM |
/// | `$8000..=$9FFF` | `vbus` | Video RAM     |
/// | `$A000..=$BFFF` | `ebus` | External RAM  |
/// | `$C000..=$FFFF` | `ebus` | Work/Echo RAM |
#[derive(Debug)]
#[derive(Memory)]
pub struct Dma {
    /// Video bus.
    #[mmap(0x8000..=0x9fff)]
    pub vbus: Vbus,
    /// External bus.
    #[mmap(0x0000..=0xffff)]
    pub ebus: Ebus,
}
