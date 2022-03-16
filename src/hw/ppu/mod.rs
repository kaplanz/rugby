use std::cell::RefCell;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use log::trace;
use remus::bus::Bus;
use remus::mem::Ram;
use remus::reg::Register;
use remus::{Block, Device, Machine};

use self::pixel::{Colour, Fetch, Fifo};
use self::sprite::Sprite;

mod pixel;
mod sprite;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Screen([Colour; WIDTH * HEIGHT]);

impl Default for Screen {
    fn default() -> Self {
        Self([Default::default(); WIDTH * HEIGHT])
    }
}

impl Deref for Screen {
    type Target = [Colour];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌{}┐", "─".repeat(WIDTH))?;
        let rows = self.chunks_exact(WIDTH);
        let even = rows.clone().step_by(2);
        let odd = rows.clone().step_by(2).skip(1);
        let rows = even.zip(odd);
        for (even, odd) in rows {
            writeln!(
                f,
                "│{}│",
                even.iter()
                    .zip(odd)
                    .map(|(p0, p1)| match (p0, p1) {
                        (Colour::C0, Colour::C0) => ' ',
                        (Colour::C0, _) => '▄',
                        (_, Colour::C0) => '▀',
                        (_, _) => '█',
                    })
                    .collect::<String>()
            )?;
        }
        write!(f, "└{}┘", "─".repeat(WIDTH))
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
pub struct Ppu {
    pub lcd: Screen,
    dot: usize,
    mode: Mode,
    // ┌────────┬──────────────────┬─────┬───────┐
    // │  SIZE  │       NAME       │ DEV │ ALIAS │
    // ├────────┼──────────────────┼─────┼───────┤
    // │ 8 Ki B │            Video │ RAM │ VRAM  │
    // │  160 B │ Object Attribute │ RAM │ OAM   │
    // │   12 B │      LCD Control │ Reg │       │
    // └────────┴──────────────────┴─────┴───────┘
    pub vram: Rc<RefCell<Ram<0x2000>>>,
    pub oam:  Rc<RefCell<Ram<0x00a0>>>,
    pub regs: Rc<RefCell<Registers>>,
}

impl Block for Ppu {
    fn reset(&mut self) {
        // Reset mode
        self.mode = Default::default();
        // Reset LCD
        self.lcd = Default::default();
        // Reset memory
        self.vram.borrow_mut().reset();
        self.oam.borrow_mut().reset();
        // Reset registers
        self.regs.borrow_mut().reset();
    }
}

impl Machine for Ppu {
    fn enabled(&self) -> bool {
        Lcdc::Enable.get(&*self.regs.borrow().lcdc.borrow_mut())
    }

    fn cycle(&mut self) {
        self.mode = std::mem::take(&mut self.mode).exec(self);
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
pub struct Registers {
    bus: Bus,
    // ┌──────┬────────────────────┬─────┬───────┐
    // │ SIZE │        NAME        │ DEV │ ALIAS │
    // ├──────┼────────────────────┼─────┼───────┤
    // │  1 B │        LCD Control │ Reg │ LCDC  │
    // │  1 B │         LCD Status │ Reg │ STAT  │
    // │  1 B │           Scroll Y │ Reg │ SCY   │
    // │  1 B │           Scroll X │ Reg │ SCX   │
    // │  1 B │   LCD Y Coordinate │ Reg │ LY    │
    // │  1 B │         LY Compare │ Reg │ LYC   │
    // │  1 B │ DMA Transfer Start │ Reg │ DMA   │
    // │  1 B │    BG Palette Data │ Reg │ BGP   │
    // │  1 B │ OBJ Palette 0 Data │ Reg │ OBP0  │
    // │  1 B │ OBJ Palette 1 Data │ Reg │ OBP1  │
    // │  1 B │           Window Y │ Reg │ WY    │
    // │  1 B │           Window X │ Reg │ WX    │
    // └──────┴────────────────────┴─────┴───────┘
    lcdc: Rc<RefCell<Register<u8>>>,
    stat: Rc<RefCell<Register<u8>>>,
    scy:  Rc<RefCell<Register<u8>>>,
    scx:  Rc<RefCell<Register<u8>>>,
    ly:   Rc<RefCell<Register<u8>>>,
    lyc:  Rc<RefCell<Register<u8>>>,
    dma:  Rc<RefCell<Register<u8>>>,
    bgp:  Rc<RefCell<Register<u8>>>,
    obp0: Rc<RefCell<Register<u8>>>,
    obp1: Rc<RefCell<Register<u8>>>,
    wy:   Rc<RefCell<Register<u8>>>,
    wx:   Rc<RefCell<Register<u8>>>,
}

impl Block for Registers {
    #[rustfmt::skip]
    fn reset(&mut self) {
        // Reset self
        std::mem::take(self);
        // Reset bus                           // ┌──────┬──────────────────┬─────┐
        self.bus.reset();                      // │ SIZE │      NAME        │ DEV │
                                               // ├──────┼──────────────────┼─────┤
        self.bus.map(0x00, self.lcdc.clone()); // │  1 B │      LCD Control │ Reg │
        self.bus.map(0x01, self.stat.clone()); // │  1 B │       LCD Status │ Reg │
        self.bus.map(0x02, self.scy.clone());  // │  1 B │         Scroll Y │ Reg │
        self.bus.map(0x03, self.scx.clone());  // │  1 B │         Scroll X │ Reg │
        self.bus.map(0x04, self.ly.clone());   // │  1 B │ LCD Y Coordinate │ Reg │
        self.bus.map(0x05, self.lyc.clone());  // │  1 B │       LY Compare │ Reg │
        self.bus.map(0x06, self.dma.clone());  // │  1 B │       LY Compare │ Reg │
        self.bus.map(0x07, self.bgp.clone());  // │  1 B │       LY Compare │ Reg │
        self.bus.map(0x08, self.obp0.clone()); // │  1 B │       LY Compare │ Reg │
        self.bus.map(0x09, self.obp1.clone()); // │  1 B │       LY Compare │ Reg │
        self.bus.map(0x0a, self.wy.clone());   // │  1 B │         Window Y │ Reg │
        self.bus.map(0x0b, self.wx.clone());   // │  1 B │         Window X │ Reg │
                                               // └──────┴──────────────────┴─────┘
    }
}

impl Device for Registers {
    fn contains(&self, index: usize) -> bool {
        self.bus.contains(index)
    }

    fn read(&self, index: usize) -> u8 {
        self.bus.read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        self.bus.write(index, value);
    }
}

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Lcdc {
    Enable      = 0b10000000,
    WinMap      = 0b01000000,
    WinEnable   = 0b00100000,
    BgWinData   = 0b00010000,
    BgMap       = 0b00001000,
    ObjSize     = 0b00000100,
    ObjEnable   = 0b00000010,
    BgWinEnable = 0b00000001,
}

impl Lcdc {
    pub fn get(self, lcdc: &u8) -> bool {
        *lcdc & self as u8 != 0
    }
}

#[derive(Debug)]
enum Mode {
    Scan(Scan),
    Draw(Draw),
    HBlank(HBlank),
    VBlank(VBlank),
}

impl Mode {
    fn exec(self, ppu: &mut Ppu) -> Self {
        trace!("{:03}: {self:?}", ppu.dot);
        // Execute the current PPU mode
        match self {
            Mode::Scan(scan) => scan.exec(ppu),
            Mode::Draw(draw) => draw.exec(ppu),
            Mode::HBlank(hblank) => hblank.exec(ppu),
            Mode::VBlank(vblank) => vblank.exec(ppu),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Scan(Default::default())
    }
}

#[derive(Debug, Default)]
struct Scan {
    idx: usize,
    objs: Vec<Sprite>,
}

impl Scan {
    fn exec(mut self, ppu: &mut Ppu) -> Mode {
        // Extract the sprite and scanline config
        let regs = ppu.regs.borrow();
        let lcdc = **regs.lcdc.borrow();
        let enabled = Lcdc::ObjEnable.get(&lcdc);
        let size = Lcdc::ObjSize.get(&lcdc);
        let ht = [8, 16][size as usize];
        let ly = **regs.ly.borrow();

        // Scan should only run when the following conditions are met:
        // - Sprites are enabled
        // - Fewer than 10 sprites have been found per scanline
        // - During an "on" cycle, as Scan runs at 2 MiHz
        if enabled && self.objs.len() < 10 && ppu.dot % 2 == 0 {
            // Checking an OAM entry takes 4 dots, due to the read. This means
            // we need another clock divider to disable during the "off" cycle.
            if ppu.dot % 4 == 0 {
                // Scan the current OAM entry
                let mut obj = [0; 4];
                obj.iter_mut().for_each(|byte| {
                    *byte = ppu.oam.borrow().read(self.idx);
                    self.idx += 1;
                });
                // Parse into Sprite
                let obj = Sprite::from(obj);

                // Add sprite to be rendered if it's on the current scanline
                if (obj.ypos..=obj.ypos + ht).contains(&ly) {
                    self.objs.push(obj);
                }
            }
        } else {
            // Regardless, move to next OAM entry
            // NOTE: We're incrementing by 2 here, since the PPU has a 16-bit
            //       wide bus to the OAM, allowing it to access one word (2
            //       bytes) per dot.
            // TODO: <add source>
            self.idx += 2;
        }

        // Scan lasts 80 dots, then progresses to Draw
        ppu.dot += 1;
        if ppu.dot < 80 {
            Mode::Scan(self)
        } else {
            Mode::Draw(self.into())
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Draw {
    cx: usize,
    fetch: Fetch,
    fifo: Fifo,
    objs: Vec<Sprite>,
}

impl Draw {
    fn exec(mut self, ppu: &mut Ppu) -> Mode {
        // TODO: draw sprites
        // If we have a pixel to draw, draw it
        if let Some(pixel) = self.fifo.pop() {
            // Extract current scanline
            let regs = ppu.regs.borrow();
            let ly = **regs.ly.borrow();

            // Push the pixel into the framebuffer
            let idx = (ly as usize * WIDTH) + self.cx as usize;
            ppu.lcd[idx] = pixel.colour;
            // Increment the internal pixel column x-position
            self.cx += 1;
        }

        // Execute the next cycle of the fetch
        // NOTE: Since the fetcher runs at 2 MiHz, use a clock divider
        if ppu.dot % 2 == 0 {
            self.fetch.exec(&mut self.fifo, ppu);
        }

        // Either draw next pixel, or enter HBlank
        ppu.dot += 1;
        if self.cx < WIDTH {
            Mode::Draw(self)
        } else {
            Mode::HBlank(Default::default())
        }
    }
}

impl From<Scan> for Draw {
    fn from(scan: Scan) -> Self {
        Self {
            objs: scan.objs,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct HBlank;

impl HBlank {
    fn exec(self, ppu: &mut Ppu) -> Mode {
        // HBlank lasts until the 456th dot
        ppu.dot += 1;
        if ppu.dot < 456 {
            Mode::HBlank(self)
        } else {
            // Extract scanline config
            let regs = ppu.regs.borrow();
            let mut ly = regs.ly.borrow_mut();
            let ly = &mut **ly;
            // Increment scanline at the 456th dot, and reset dot-clock
            *ly += 1;
            ppu.dot = 0;

            // Either begin next scanline, or enter VBlank
            if *ly < HEIGHT as u8 {
                Mode::Scan(Default::default())
            } else {
                Mode::VBlank(Default::default())
            }
        }
    }
}

#[derive(Debug, Default)]
struct VBlank;

impl VBlank {
    fn exec(self, ppu: &mut Ppu) -> Mode {
        // VBlank lasts for 456 dots per scanline
        ppu.dot += 1;
        if ppu.dot < 456 {
            Mode::VBlank(self)
        } else {
            // Extract scanline config
            let regs = ppu.regs.borrow();
            let mut ly = regs.ly.borrow_mut();
            let ly = &mut **ly;
            // Increment scanline at the 456th dot, and reset dot-clock
            *ly += 1;
            ppu.dot = 0;

            // VBlank lasts for scanlines 144..154
            if *ly < 154 {
                Mode::VBlank(self)
            } else {
                // Reset scanline
                *ly = 0;
                // Restart PPU
                Mode::Scan(Default::default())
            }
        }
    }
}