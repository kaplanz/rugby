//! Sharp LR35902.

use rugby_arch::{Block, Shared};

use super::boot;
use super::mmap::Mmap;
use super::pcb::Vram;
pub use crate::chip::{apu, cpu, dma, irq, joy, ppu, sio, tma};

/// Sharp LR35902 (DMG-CPU).
#[derive(Debug)]
pub struct Chip {
    /// Audio processing unit.
    pub apu: apu::Apu,
    /// Boot ROM.
    pub boot: Option<boot::Chip>,
    /// Central processing unit.
    pub cpu: cpu::Cpu,
    /// Direct memory access unit.
    pub dma: dma::Dma,
    /// Interrupt controller.
    pub irq: irq::Irq,
    /// Joypad controller.
    pub joy: joy::Joypad,
    /// Picture processing unit
    pub ppu: ppu::Ppu,
    /// Serial communications port.
    pub sio: sio::Serial,
    /// Hardware timer.
    pub tma: tma::Timer,
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
    pub fn new(vram: &Shared<Vram>, noc: &Mmap) -> Self {
        // Interrupt controller
        let irq = irq::Irq::default();
        // Hardware timer
        let tma = tma::Timer {
            reg: tma::File::default(),
            etc: tma::Internal::default(),
            int: irq.line.clone(),
        };
        // Object attribute memory
        //
        // Created before the PPU and DMA, which each hold a handle.
        let oam = Shared::new(ppu::Oam::from([u8::default(); 0x00a0]));

        // Audio processing unit
        let apu = {
            let mem = apu::Bank::default();
            let reg = apu::File::default();
            apu::Apu {
                ch1: apu::ch1::Channel {
                    out: f32::default(),
                    reg: apu::ch1::File::with(&reg),
                    etc: apu::ch1::Internal::default(),
                },
                ch2: apu::ch2::Channel {
                    out: f32::default(),
                    reg: apu::ch2::File::with(&reg),
                    etc: apu::ch2::Internal::default(),
                },
                ch3: apu::ch3::Channel {
                    out: f32::default(),
                    reg: apu::ch3::File::with(&reg),
                    mem: mem.clone(),
                    etc: apu::ch3::Internal::default(),
                },
                ch4: apu::ch4::Channel {
                    out: f32::default(),
                    reg: apu::ch4::File::with(&reg),
                    etc: apu::ch4::Internal::default(),
                },
                reg,
                mem,
                seq: apu::Sequencer {
                    bit: bool::default(),
                    clk: u8::default(),
                    div: tma.reg.div.clone(),
                },
                etc: apu::Internal::default(),
            }
        };
        // Central processing unit
        let cpu = cpu::Cpu {
            bus: noc.cpu(),
            mem: cpu::Bank::default(),
            reg: cpu::File::default(),
            etc: cpu::Internal::default(),
            int: irq.line.clone(),
        };
        // Direct memory access unit
        let dma = dma::Dma {
            bus: noc.dma(),
            mem: oam.clone(),
            reg: Shared::new(dma::Control::default()),
        };
        // Joypad controller
        let joy = joy::Joypad {
            reg: Shared::new(joy::Control::default()),
            int: irq.line.clone(),
        };
        // Picture processing unit
        let ppu = ppu::Ppu {
            mem: ppu::Bank {
                vram: vram.clone(),
                oam,
            },
            reg: ppu::File {
                dma: dma.reg.clone(),
                ..Default::default()
            },
            etc: ppu::Internal::default(),
            int: irq.line.clone(),
        };
        // Serial communications port
        let sio = sio::Serial {
            reg: sio::File::default(),
            etc: sio::Internal::default(),
            int: irq.line.clone(),
        };

        // Finish construction
        Self {
            apu,
            boot: None,
            cpu,
            dma,
            irq,
            joy,
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
