use std::cell::RefCell;
use std::rc::Rc;

use log::warn;
use minifb::{ScaleMode, Window, WindowOptions};
use remus::bus::Bus;
use remus::dev::{Device, Null};
use remus::mem::{Ram, Rom};
use remus::reg::Register;
use remus::{clk, Block, Machine};

use self::ppu::Ppu;
use crate::cart::Cartridge;
use crate::cpu::sm83::Cpu;

mod ppu;

const BOOTROM: [u8; 0x100] = [
    0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26, 0xff, 0x0e,
    0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e, 0xfc, 0xe0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95, 0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b,
    0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06, 0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9,
    0x3e, 0x19, 0xea, 0x10, 0x99, 0x21, 0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20,
    0xf9, 0x2e, 0x0f, 0x18, 0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04,
    0x1e, 0x02, 0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
    0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64, 0x20, 0x06,
    0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15, 0x20, 0xd2, 0x05, 0x20,
    0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17,
    0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e,
    0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
    0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
    0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe, 0x34, 0x20,
    0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe, 0x3e, 0x01, 0xe0, 0x50,
];

#[derive(Debug)]
pub struct GameBoy {
    bus: Rc<RefCell<Bus>>,
    cpu: Cpu,
    cart: Cartridge,
    devs: Devices,
    ppu: Ppu,
}

impl GameBoy {
    pub fn new(cart: Cartridge) -> Self {
        Self {
            bus: Default::default(),
            cpu: Default::default(),
            cart,
            devs: Default::default(),
            ppu: Default::default(),
        }
        .reset()
    }

    #[rustfmt::skip]
    fn reset(mut self) -> Self {
        // Reset bus                                  // ┌──────────┬────────────┬─────┐
        self.bus.replace(Bus::default());             // │   SIZE   │    NAME    │ DEV │
        let mut bus = self.bus.borrow_mut();          // ├──────────┼────────────┼─────┤
        bus.map(0x0000, self.devs.boot.clone());      // │    256 B │       Boot │ ROM │
        bus.map(0x0000, self.cart.mbc.rom().clone()); // │  32 Ki B │  Cartridge │ ROM │
        bus.map(0x8000, self.ppu.vram.clone());       // │   8 Ki B │      Video │ RAM │
        bus.map(0xa000, self.cart.mbc.ram().clone()); // │   8 Ki B │   External │ RAM │
        bus.map(0xc000, self.devs.wram.clone());      // │   8 Ki B │       Work │ RAM │
        bus.map(0xe000, self.devs.wram.clone());      // │   7680 B │       Echo │ RAM │
        bus.map(0xfe00, self.ppu.oam.clone());        // │    160 B │      Video │ RAM │
                                                      // │     96 B │     Unused │ --- │
        bus.map(0xff00, self.devs.io.clone());        // │    128 B │        I/O │ Bus │
        bus.map(0xff80, self.devs.hram.clone());      // │    127 B │       High │ RAM │
        bus.map(0xffff, self.devs.ie.clone());        // │      1 B │  Interrupt │ Reg │
                                                      // └──────────┴────────────┴─────┘
        bus.map(0x0000, Rc::new(RefCell::new(Null::<0x10000>::new())));
        drop(bus);
        // Reset CPU
        self.cpu.reset();
        self.cpu.bus = self.bus.clone();
        // Reset cartridge
        self.cart.reset();
        // Reset devices
        self.devs.io.borrow_mut().lcd = self.ppu.regs.clone();
        self.devs.reset();
        // Reset PPU
        self.ppu.reset();
        self
    }

    pub fn run(&mut self) {
        // Perform setup for FSMs
        self.cpu.setup();
        self.ppu.setup();

        // Create a framebuffer window
        const PALETTE: [u32; 4] = [0xe9efec, 0xa0a08b, 0x555568, 0x211e20];
        let mut win = Window::new(
            "Game Boy",
            160,
            144,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..Default::default()
            },
        )
        .unwrap();

        // Mark the starting time
        let mut now = std::time::Instant::now();
        let mut active = 0;

        // Run FSMs on a 4 MiHz clock
        for (cycle, _) in clk::with_freq(4194304).enumerate() {
            // As an abstraction, let the CPU run on a 1 MiHz clock which can
            // be done using a simple clock division by 4.
            if cycle % 4 == 0 && self.cpu.enabled() {
                self.cpu.cycle();
            }

            // Let the PPU run on at full speed, and internally manage clock
            // division as needed.
            if self.ppu.enabled() {
                self.ppu.cycle();
            }

            // Update the screen at 59.7 Hz
            if cycle % (456 * 154) == 0 {
                let buf = self
                    .ppu
                    .lcd
                    .iter()
                    .map(|pixel| PALETTE[*pixel as usize])
                    .collect::<Vec<_>>();
                win.update_with_buffer(&buf, 160, 144).unwrap();
            }

            // Calculate real-time clock frequency
            if now.elapsed().as_secs() > 0 {
                warn!("Frequency: {active}");
                active = 0;
                now = std::time::Instant::now();
            }
            active += 1;
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
struct Devices {
                                     // ┌────────┬───────────┬─────┬───────┐
                                     // │  SIZE  │    NAME   │ DEV │ ALIAS │
                                     // ├────────┼───────────┼─────┼───────┤
    boot: Rc<RefCell<Rom<0x0100>>>,  // │  256 B │      Boot │ ROM │       │
    wram: Rc<RefCell<Ram<0x2000>>>,  // │ 8 Ki B │      Work │ RAM │ WRAM  │
    io:   Rc<RefCell<IoBus>>,        // │  128 B │       I/O │ Bus │       │
    hram: Rc<RefCell<Ram<0x007f>>>,  // │  127 B │      High │ RAM │ HRAM  │
    ie:   Rc<RefCell<Register<u8>>>, // │    1 B │ Interrupt │ Reg │ IE    │
                                     // └────────┴───────────┴─────┴───────┘
}

impl Block for Devices {
    fn reset(&mut self) {
        // Reset Boot ROM
        self.boot.replace(Rom::from(&BOOTROM));
        // Reset I/O
        self.io.borrow_mut().reset();
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
struct IoBus {
    bus: Bus,                           // ┌────────┬─────────────────┬─────┐
                                        // │  SIZE  │      NAME       │ DEV │
                                        // ├────────┼─────────────────┼─────┤
    con:   Rc<RefCell<Register<u8>>>,   // │    1 B │      Controller │ Reg │
    com:   Rc<RefCell<Register<u16>>>,  // │    2 B │   Communication │ Reg │
    timer: Rc<RefCell<Register<u32>>>,  // │    4 B │ Divider & Timer │ Reg │
    is: Rc<RefCell<Register<u8>>>,      // │    1 B │  Interrupt Flag │ Reg │
    sound: Rc<RefCell<Ram<0x17>>>,      // │   23 B │           Sound │ RAM │
    wave:  Rc<RefCell<Ram<0x10>>>,      // │   16 B │        Waveform │ RAM │
    lcd:   Rc<RefCell<ppu::Registers>>, // │   16 B │             LCD │ Ppu │
    bank:  Rc<RefCell<Register<u8>>>,   // │    1 B │   Boot ROM Bank │ Reg │
                                        // └────────┴─────────────────┴─────┘
}

#[rustfmt::skip]
impl Block for IoBus {
    fn reset(&mut self) {
        // Reset bus                            // ┌────────┬─────────────────┬─────┐
        self.bus = Bus::default();              // │  SIZE  │      NAME       │ DEV │
                                                // ├────────┼─────────────────┼─────┤
        self.bus.map(0x00, self.con.clone());   // │    1 B │      Controller │ Reg │
        self.bus.map(0x01, self.com.clone());   // │    2 B │   Communication │ Reg │
                                                // │    1 B │          Unused │ --- │
        self.bus.map(0x04, self.timer.clone()); // │    4 B │ Divider & Timer │ Reg │
                                                // │    7 B │          Unused │ --- │
        self.bus.map(0x0f, self.is.clone());    // │    1 B │  Interrupt Flag │ Reg │
        self.bus.map(0x10, self.sound.clone()); // │   23 B │           Sound │ RAM │
                                                // │    9 B │          Unused │ --- │
        self.bus.map(0x30, self.wave.clone());  // │   16 B │        Waveform │ RAM │
        self.bus.map(0x40, self.lcd.clone());   // │   12 B │             LCD │ Ppu │
                                                // │    4 B │          Unused │ --- │
        self.bus.map(0x50, self.bank.clone());  // │    1 B │   Boot ROM Bank │ Reg │
                                                // │   47 B │          Unused │ --- │
                                                // └────────┴─────────────────┴─────┘
    }
}

impl Device for IoBus {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GameBoy {
        let rom: [u8; 0x150] = [
            0xc3, 0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x8b, 0x02, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0xe1,
            0x5f, 0x16, 0x00, 0x19, 0x5e, 0x23, 0x56, 0xd5, 0xe1, 0xe9, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0xfd, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xc3, 0x12, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x12, 0x27, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xc3, 0x7e, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x50, 0x01, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d,
            0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f,
            0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb,
            0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
            0x52, 0x4f, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xf8, 0x32, 0xbb,
        ];
        GameBoy::new(Cartridge::new(&rom).unwrap())
    }

    #[test]
    fn bus_works() {
        let gb = setup();
        // Cartridge ROM
        (0x0100..=0x7fff).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x20));
        (0x0100..=0x7fff)
            .map(|addr| gb.cart.mbc.rom().borrow().read(addr))
            .any(|byte| byte != 0x20);
        // Video RAM
        (0x8000..=0x9fff).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x30));
        assert!((0x0000..=0x1fff)
            .map(|addr| gb.ppu.vram.borrow().read(addr))
            .all(|byte| byte == 0x30));
        // External RAM
        (0xa000..=0xbfff).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x40));
        assert!((0x0000..=0x1fff)
            .map(|addr| gb.cart.mbc.ram().borrow().read(addr))
            .all(|byte| byte == 0x40));
        // Video RAM (OAM)
        (0xfe00..=0xfe9f).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x50));
        assert!((0x00..=0x9f)
            .map(|addr| gb.ppu.oam.borrow().read(addr))
            .all(|byte| byte == 0x50));
        // I/O Bus
        {
            // Controller
            (0xff00..=0xff00).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x61));
            assert!((0x00..=0x00)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x61));
            assert!((0x0..=0x0)
                .map(|addr| gb.devs.io.borrow().con.borrow().read(addr))
                .all(|byte| byte == 0x61));
            // Communication
            (0xff01..=0xff02).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x62));
            assert!((0x01..=0x02)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x62));
            assert!((0x00..=0x01)
                .map(|addr| gb.devs.io.borrow().com.borrow().read(addr))
                .all(|byte| byte == 0x62));
            // Divider & Timer
            (0xff04..=0xff07).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x63));
            assert!((0x04..=0x07)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x63));
            assert!((0x00..=0x03)
                .map(|addr| gb.devs.io.borrow().timer.borrow().read(addr))
                .all(|byte| byte == 0x63));
            // Interrupt Flag
            (0xff0f..=0xff0f).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x64));
            assert!((0x0f..=0x0f)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x64));
            assert!((0x00..=0x00)
                .map(|addr| gb.devs.io.borrow().is.borrow().read(addr))
                .all(|byte| byte == 0x64));
            // Sound
            (0xff10..=0xff26).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x65));
            assert!((0x10..=0x26)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x65));
            assert!((0x00..=0x16)
                .map(|addr| gb.devs.io.borrow().sound.borrow().read(addr))
                .all(|byte| byte == 0x65));
            // Waveform RAM
            (0xff30..=0xff3f).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x66));
            assert!((0x30..=0x3f)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x66));
            assert!((0x00..=0x0f)
                .map(|addr| gb.devs.io.borrow().wave.borrow().read(addr))
                .all(|byte| byte == 0x66));
            // LCD
            (0xff40..=0xff4b).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x67));
            assert!((0x40..=0x4b)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x67));
            assert!((0x00..=0x0b)
                .map(|addr| gb.devs.io.borrow().lcd.borrow().read(addr))
                .all(|byte| byte == 0x67));
            assert!((0x00..=0x0b)
                .map(|addr| gb.ppu.regs.borrow().read(addr))
                .all(|byte| byte == 0x67));
            // Boot ROM Disable
            (0xff50..=0xff50).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x68));
            assert!((0x50..=0x50)
                .map(|addr| gb.devs.io.borrow().bus.read(addr))
                .all(|byte| byte == 0x68));
            assert!((0x00..=0x00)
                .map(|addr| gb.devs.io.borrow().bank.borrow().read(addr))
                .all(|byte| byte == 0x68));
        }
        // High RAM
        (0xff80..=0xfffe).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x70));
        assert!((0x00..=0x7e)
            .map(|addr| gb.devs.hram.borrow().read(addr))
            .all(|byte| byte == 0x70));
        // Interrupt Enable
        (0xffff..=0xffff).for_each(|addr| gb.bus.borrow_mut().write(addr, 0x80));
        assert!((0x0..=0x0)
            .map(|addr| gb.devs.ie.borrow().read(addr))
            .all(|byte| byte == 0x80));
    }

    #[test]
    fn bus_null_works() {
        let gb = setup();
        // Write to every (writable) address
        (0x8000..=0xffff).for_each(|addr| {
            gb.bus.borrow_mut().write(addr, 0xaa);
        });
        // Read from every address
        (0x0000..=0xffff).for_each(|addr| {
            gb.bus.borrow().read(addr);
        })
    }

    #[test]
    fn bus_rom_read_works() {
        let gb = setup();
        // Boot ROM
        (0x0000..=0x00ff).for_each(|addr| {
            gb.bus.borrow().read(addr);
        });
        // Cartridge ROM
        (0x0100..=0x7fff).for_each(|addr| {
            gb.bus.borrow().read(addr);
        });
    }

    #[test]
    #[should_panic]
    fn bus_rom_write_boot_panics() {
        let gb = setup();
        // Boot ROM
        gb.bus.borrow_mut().write(0x0000, 0xaa);
    }
}
