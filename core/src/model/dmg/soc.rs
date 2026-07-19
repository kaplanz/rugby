//! Sharp LR35902.

use rugby_arch::{Block, Shared};

use super::pcb::{Vram, Wram};
use super::{boot, mmap};
use crate::cart;
pub use crate::chip::{apu, cpu, dma, irq, joy, ppu, sio, tma};

/// Sharp LR35902 (DMG-CPU).
#[derive(Debug)]
pub struct SoC {
    /// Audio processing unit.
    pub apu: apu::Apu,
    /// Boot ROM.
    pub boot: boot::Slot,
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

impl SoC {
    /// Constructs a new `SoC`.
    ///
    /// # Note
    ///
    /// This is actually much more complicated than simply initializing each
    /// component, as there is a somewhat complicated dependency chain between
    /// which parts are connected to each other.
    #[must_use]
    #[expect(clippy::too_many_lines)]
    pub fn new(vram: &Shared<Vram>, wram: &Shared<Wram>, cart: &cart::Slot) -> Self {
        // Boot ROM
        let boot = boot::Slot::new();
        // Interrupt controller
        let irq = irq::Irq::default();
        // Hardware timer
        let tma = tma::Timer {
            reg: tma::File::default(),
            etc: tma::Internal::default(),
            irq: irq.line.clone(),
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
        // Direct memory access unit
        let dma = dma::Dma {
            bus: mmap::view::Dma {
                vbus: mmap::Vbus { vram: vram.clone() },
                ebus: mmap::Ebus {
                    cart: cart.clone(),
                    wram: wram.clone(),
                },
            },
            mem: oam.clone(),
            reg: Shared::new(dma::Control::default()),
        };
        // Joypad controller
        let joy = joy::Joypad {
            reg: Shared::new(joy::Control::default()),
            irq: irq.line.clone(),
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
            irq: irq.line.clone(),
        };
        // Serial communications port
        let sio = sio::Serial {
            reg: sio::File::default(),
            etc: sio::Internal::default(),
            irq: irq.line.clone(),
        };
        // Central processing unit
        let cpu = {
            let mem = cpu::Bank::default();
            cpu::Cpu {
                bus: mmap::view::Cpu {
                    ibus: mmap::Ibus {
                        boot: boot.clone(),
                        ppu: ppu.mem.clone(),
                        apu: apu.mem.clone(),
                        io: mmap::File {
                            joy: joy.reg.clone(),
                            sio: sio.reg.clone(),
                            tma: tma.reg.clone(),
                            irq: irq.reg.clone(),
                            apu: apu.reg.clone(),
                            dma: dma.reg.clone(),
                            ppu: ppu.reg.clone(),
                            boot: boot.clone(),
                        },
                        cpu: mem.clone(),
                    },
                    vbus: mmap::Vbus { vram: vram.clone() },
                    ebus: mmap::Ebus {
                        cart: cart.clone(),
                        wram: wram.clone(),
                    },
                },
                mem,
                reg: cpu::File::default(),
                etc: cpu::Internal::default(),
                irq: irq.line.clone(),
            }
        };

        // Finish construction
        Self {
            apu,
            boot,
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

impl Block for SoC {
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
