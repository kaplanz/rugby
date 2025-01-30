//! System-on-chip.

use rugby_arch::{Block, Shared};

use super::mem::Bank;
use super::noc::Mmap;
use super::{apu, cpu, dma, joypad, pic, ppu, serial, timer};

/// Sharp LR35902 (DMG-CPU).
#[derive(Debug)]
pub struct Chip {
    /// Audio processing unit.
    pub apu: apu::Apu,
    /// Central processing unit.
    pub cpu: cpu::Cpu,
    /// Direct memory access unit.
    pub dma: dma::Dma,
    /// Joypad controller.
    pub joy: joypad::Joypad,
    /// Interrupt controller.
    pub pic: pic::Pic,
    /// Picture processing unit
    pub ppu: ppu::Ppu,
    /// Serial communications port.
    pub sio: serial::Serial,
    /// Hardware timer.
    pub tma: timer::Timer,
}

impl Chip {
    /// Constructs a new `Chip`.
    ///
    /// # Note
    ///
    /// This is actually much more complicated than simply initializing each
    /// component, as there is a somewhat complicated dependency chain between
    /// which parts are connected to each other.
    #[must_use]
    pub fn new(mem: &Bank, noc: &Mmap) -> Self {
        // Interrupt controller
        let pic = pic::Pic::default();

        // Audio processing unit
        let apu = apu::Apu::default();
        // Central processing unit
        let cpu = cpu::Cpu {
            bus: noc.cpu(),
            mem: cpu::Bank {
                wram: mem.wram.clone(),
                hram: mem.hram.clone(),
            },
            reg: cpu::Control::default(),
            etc: cpu::Internal::default(),
            int: pic.line.clone(),
        };
        // Direct memory access unit
        let dma = dma::Dma {
            bus: noc.dma(),
            mem: mem.oam.clone(),
            reg: Shared::new(dma::Control::default()),
        };
        // Joypad controller
        let joy = joypad::Joypad {
            reg: Shared::new(joypad::Control::default()),
            int: pic.line.clone(),
        };
        // Picture processing unit
        let ppu = ppu::Ppu {
            mem: ppu::Bank {
                vram: mem.vram.clone(),
                oam: mem.oam.clone(),
            },
            reg: ppu::Control {
                dma: dma.reg.clone(),
                ..Default::default()
            },
            etc: ppu::Internal::default(),
            int: pic.line.clone(),
        };
        // Serial communications port
        let sio = serial::Serial {
            reg: serial::Control::default(),
            etc: serial::Internal::default(),
            int: pic.line.clone(),
        };
        // Hardware timer
        let tma = timer::Timer {
            reg: timer::Control::default(),
            etc: timer::Internal::default(),
            int: pic.line.clone(),
        };

        // Finish construction
        Self {
            apu,
            cpu,
            dma,
            joy,
            pic,
            ppu,
            sio,
            tma,
        }
    }
}

impl Block for Chip {
    fn ready(&self) -> bool {
        self.cpu.ready()
    }

    fn reset(&mut self) {
        self.apu.reset();
        self.cpu.reset();
        self.dma.reset();
        self.joy.reset();
        self.ppu.reset();
        self.sio.reset();
        self.tma.reset();
    }
}
