use std::cell::RefCell;
use std::rc::Rc;

use remus::bus::Bus;
use remus::dev::Device;
use remus::mem::Ram;
use remus::reg::Register;
use remus::{Block, Machine};

use crate::cart::Cartridge;
use crate::cpu::sm83::Cpu;
use crate::emu::Button;
use crate::hw::joypad::{self, Joypad};
use crate::hw::pic::Pic;
use crate::hw::ppu::{self, Ppu};
use crate::hw::timer::{self, Timer};
use crate::mem::Unmapped;
use crate::Emulator;

mod boot;

const PALETTE: [u32; 4] = [0xe9efec, 0xa0a08b, 0x555568, 0x211e20];

#[derive(Debug)]
pub struct GameBoy {
    cart: Cartridge,
    cycle: usize,
    cpu: Cpu,
    io: InOut,
    joypad: Joypad,
    mem: Memory,
    mmu: Rc<RefCell<Bus>>,
    pic: Rc<RefCell<Pic>>,
    ppu: Ppu,
    timer: Timer,
}

impl GameBoy {
    pub fn new(cart: Cartridge) -> Self {
        let mut this = Self {
            cart,
            cycle: Default::default(),
            cpu: Default::default(),
            io: Default::default(),
            joypad: Default::default(),
            mem: Default::default(),
            mmu: Default::default(),
            pic: Default::default(),
            ppu: Default::default(),
            timer: Default::default(),
        };
        this.reset();
        this
    }

    #[rustfmt::skip]
    fn memmap(&mut self) {
        // Map MMU
        self.mmu.take();                          // ┌──────────┬────────────┬─────┐
        let mut mmu = self.mmu.borrow_mut();      // │   SIZE   │    NAME    │ DEV │
        let pic = self.pic.borrow();              // ├──────────┼────────────┼─────┤
        mmu.map(0x0000, self.mem.boot.clone());   // │    256 B │       Boot │ ROM │
        mmu.map(0x0000, self.cart.rom().clone()); // │  32 Ki B │  Cartridge │ ROM │
        mmu.map(0x8000, self.ppu.vram.clone());   // │   8 Ki B │      Video │ RAM │
        mmu.map(0xa000, self.cart.ram().clone()); // │   8 Ki B │   External │ RAM │
        mmu.map(0xc000, self.mem.wram.clone());   // │   8 Ki B │       Work │ RAM │
        mmu.map(0xe000, self.mem.wram.clone());   // │   7680 B │       Echo │ RAM │
        mmu.map(0xfe00, self.ppu.oam.clone());    // │    160 B │        OAM │ RAM │
                                                  // │     96 B │     Unused │ --- │
        mmu.map(0xff00, self.io.bus.clone());     // │    128 B │        I/O │ Bus │
        mmu.map(0xff80, self.mem.hram.clone());   // │    127 B │       High │ RAM │
        mmu.map(0xffff, pic.enable.clone());      // │      1 B │  Interrupt │ Reg │
                                                  // └──────────┴────────────┴─────┘
        // Use an `Unmapped` as a fallback
        mmu.map(0x0000, Rc::new(RefCell::new(Unmapped::new())));
        drop(mmu); // release mutable borrow
    }
}

impl Block for GameBoy {
    fn reset(&mut self) {
        // Reset CPU
        self.cpu.reset();
        self.cpu.set_bus(self.mmu.clone()); // link CPU bus

        // Reset cartridge
        self.cart.reset();

        // Reset I/O
        self.io.reset();
        self.io.con = self.joypad.p1.clone(); // link joypad
        self.io.timer = self.timer.regs.clone(); // link timer registers
        self.io.iflag = self.pic.borrow().active.clone(); // link IF register
        self.io.lcd = self.ppu.ctl.clone(); // link LCD controller
        self.io.boot = self.mem.boot.borrow().ctl.clone(); // link BOOT controller

        // Reset memory
        self.mem.reset();

        // Reset interrupts
        self.pic.borrow_mut().reset();
        self.cpu.set_pic(self.pic.clone()); // link PIC
        self.joypad.set_pic(self.pic.clone()); // link PIC
        self.timer.set_pic(self.pic.clone()); // link PIC

        // Reset joypad
        self.joypad.reset();

        // Reset PPU
        self.ppu.reset();
        self.ppu.set_pic(self.pic.clone()); // link PIC

        // Reset timer
        self.timer.reset();

        // Re-map MMU
        self.memmap();
        self.io.memmap();
    }
}

impl Emulator for GameBoy {
    fn send(&mut self, btns: Vec<Button>) {
        self.joypad.recv(btns);
    }

    fn redraw<F>(&self, mut draw: F)
    where
        F: FnMut(&[u32]),
    {
        if self.ppu.refresh() {
            let buf = self
                .ppu
                .screen()
                .iter()
                .map(|&pixel| PALETTE[pixel as usize])
                .collect::<Vec<_>>();
            draw(&buf);
        }
    }
}

impl Machine for GameBoy {
    fn setup(&mut self) {
        // Set up CPU
        self.cpu.setup();
        // Set up PPU
        self.ppu.setup();
    }

    fn enabled(&self) -> bool {
        self.cpu.enabled()
    }

    fn cycle(&mut self) {
        // Wake CPU if interrupts pending
        if self.pic.borrow().int().is_some() {
            self.cpu.wake();
        }

        // CPU runs on a 1 MiHz clock: implement using a simple clock divider
        if self.cycle % 4 == 0 && self.cpu.enabled() {
            self.cpu.cycle();
        }

        // PPU runs on a 4 MiHz clock
        if self.ppu.enabled() {
            self.ppu.cycle();
        }

        // Timer runs on a 4 MiHz clock
        if self.timer.enabled() {
            self.timer.cycle();
        }

        // Keep track of cycles executed
        self.cycle = self.cycle.wrapping_add(1);
    }
}

#[derive(Debug, Default)]
struct Memory {
    // ┌────────┬──────┬─────┬───────┐
    // │  SIZE  │ NAME │ DEV │ ALIAS │
    // ├────────┼──────┼─────┼───────┤
    // │  256 B │ Boot │ ROM │       │
    // │ 8 Ki B │ Work │ RAM │ WRAM  │
    // │  127 B │ High │ RAM │ HRAM  │
    // └────────┴──────┴─────┴───────┘
    boot: Rc<RefCell<boot::Rom>>,
    wram: Rc<RefCell<Ram<0x2000>>>,
    hram: Rc<RefCell<Ram<0x007f>>>,
}

impl Block for Memory {
    fn reset(&mut self) {
        // Reset boot ROM
        self.boot.borrow_mut().reset();
    }
}

#[rustfmt::skip]
#[derive(Debug, Default)]
struct InOut {
    pub bus: Rc<RefCell<Bus>>,
    // ┌────────┬──────────────────┬─────┐
    // │  SIZE  │       NAME       │ DEV │
    // ├────────┼──────────────────┼─────┤
    // │    1 B │       Controller │ Reg │
    // │    2 B │    Communication │ Reg │
    // │    4 B │  Divider & Timer │ Reg │
    // │    1 B │   Interrupt Flag │ Reg │
    // │   23 B │            Sound │ RAM │
    // │   16 B │         Waveform │ RAM │
    // │   16 B │              LCD │ PPU │
    // │    1 B │ Boot ROM Disable │ Reg │
    // └────────┴──────────────────┴─────┘
    con:   Rc<RefCell<joypad::Register>>,
    com:   Rc<RefCell<Register<u16>>>,
    timer: Rc<RefCell<timer::Registers>>,
    iflag: Rc<RefCell<Register<u8>>>,
    sound: Rc<RefCell<Ram<0x17>>>,
    wave:  Rc<RefCell<Ram<0x10>>>,
    lcd:   Rc<RefCell<ppu::Registers>>,
    boot:  Rc<RefCell<boot::RomDisable>>,
}

impl InOut {
    #[rustfmt::skip]
    fn memmap(&mut self) {
        // Map bus                            // ┌────────┬─────────────────┬─────┐
        self.bus.take();                      // │  SIZE  │      NAME       │ DEV │
        let mut bus = self.bus.borrow_mut();  // ├────────┼─────────────────┼─────┤
        bus.map(0x00, self.con.clone());      // │    1 B │      Controller │ Reg │
        bus.map(0x01, self.com.clone());      // │    2 B │   Communication │ Reg │
                                              // │    1 B │          Unused │ --- │
        bus.map(0x04, self.timer.clone());    // │    4 B │ Divider & Timer │ Reg │
                                              // │    7 B │          Unused │ --- │
        bus.map(0x0f, self.iflag.clone());    // │    1 B │  Interrupt Flag │ Reg │
        bus.map(0x10, self.sound.clone());    // │   23 B │           Sound │ RAM │
                                              // │    9 B │          Unused │ --- │
        bus.map(0x30, self.wave.clone());     // │   16 B │        Waveform │ RAM │
        bus.map(0x40, self.lcd.clone());      // │   12 B │             LCD │ Ppu │
                                              // │    4 B │          Unused │ --- │
        bus.map(0x50, self.boot.clone());     // │    1 B │   Boot ROM Bank │ Reg │
                                              // │   47 B │          Unused │ --- │
        drop(bus); // release mutable borrow  // └────────┴─────────────────┴─────┘
    }
}

impl Block for InOut {}

impl Device for InOut {
    fn contains(&self, index: usize) -> bool {
        self.bus.borrow().contains(index)
    }

    fn len(&self) -> usize {
        self.bus.borrow().len()
    }

    fn read(&self, index: usize) -> u8 {
        self.bus.borrow().read(index)
    }

    fn write(&mut self, index: usize, value: u8) {
        self.bus.borrow_mut().write(index, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GameBoy {
        let rom: [u8; 0x150] = [
            0xc3, 0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x8b, 0x02, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x87, 0xe1,
            0x5f, 0x16, 0x00, 0x19, 0x5e, 0x23, 0x56, 0xd5, 0xe1, 0xe9, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc3, 0xfd, 0x01, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xc3, 0x12, 0x27, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc3, 0x12, 0x27, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xc3, 0x7e, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0x00, 0xc3, 0x50, 0x01, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d,
            0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f,
            0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb,
            0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x01, 0x00, 0x00, 0xdc, 0x31, 0xbb,
        ];
        GameBoy::new(Cartridge::new(&rom).unwrap())
    }

    #[test]
    fn mmu_works() {
        // NOTE: Test reads (and writes) for each component separately
        let gb = setup();

        // Cartridge ROM
        (0x0100..=0x7fff).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x20));
        assert!((0x0100..=0x7fff)
            .map(|addr| gb.cart.rom().borrow().read(addr))
            .any(|byte| byte != 0x20));
        // Video RAM
        (0x8000..=0x9fff).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x30));
        (0x0000..=0x1fff)
            .map(|addr| gb.ppu.vram.borrow().read(addr))
            .for_each(|byte| assert_eq!(byte, 0x30));
        // External RAM
        (0xa000..=0xbfff).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x40));
        (0x0000..=0x1fff)
            .map(|addr| gb.cart.ram().borrow().read(addr))
            .for_each(|byte| assert_eq!(byte, 0x40));
        // OAM RAM
        (0xfe00..=0xfe9f).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x50));
        (0x00..=0x9f)
            .map(|addr| gb.ppu.oam.borrow().read(addr))
            .for_each(|byte| assert_eq!(byte, 0x50));
        // I/O Bus
        {
            // Controller
            (0xff00..=0xff00).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x61));
            // NOTE: Only bits 0x30 are writable
            (0x00..=0x00)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0xef));
            (0x0..=0x0)
                .map(|addr| gb.io.con.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0xef));
            // Communication
            (0xff01..=0xff02).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x62));
            (0x01..=0x02)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x62));
            (0x00..=0x01)
                .map(|addr| gb.io.com.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x62));
            // Divider & Timer
            (0xff04..=0xff07).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x63));
            (0x04..=0x07)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x63));
            (0x00..=0x03)
                .map(|addr| gb.io.timer.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x63));
            // Interrupt Flag
            (0xff0f..=0xff0f).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x64));
            (0x0f..=0x0f)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x64));
            (0x0..=0x0)
                .map(|addr| gb.io.iflag.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x64));
            (0x0..=0x0)
                .map(|addr| gb.pic.borrow().active.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x64));
            // Sound
            (0xff10..=0xff26).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x65));
            (0x10..=0x26)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x65));
            (0x00..=0x16)
                .map(|addr| gb.io.sound.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x65));
            // Waveform RAM
            (0xff30..=0xff3f).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x66));
            (0x30..=0x3f)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x66));
            (0x00..=0x0f)
                .map(|addr| gb.io.wave.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x66));
            // LCD
            (0xff40..=0xff4b).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x67));
            (0x40..=0x4b)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x67));
            (0x00..=0x0b)
                .map(|addr| gb.io.lcd.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x67));
            (0x00..=0x0b)
                .map(|addr| gb.ppu.ctl.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x67));
            // Boot ROM Disable
            (0xff50..=0xff50).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x68));
            (0x50..=0x50)
                .map(|addr| gb.io.bus.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x68));
            (0x00..=0x00)
                .map(|addr| gb.io.boot.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x68));
            (0x00..=0x00)
                .map(|addr| gb.mem.boot.borrow().ctl.borrow().read(addr))
                .for_each(|byte| assert_eq!(byte, 0x68));
        }
        // High RAM
        (0xff80..=0xfffe).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x70));
        (0x00..=0x7e)
            .map(|addr| gb.mem.hram.borrow().read(addr))
            .for_each(|byte| assert_eq!(byte, 0x70));
        // Interrupt Enable
        (0xffff..=0xffff).for_each(|addr| gb.mmu.borrow_mut().write(addr, 0x80));
        (0x0..=0x0)
            .map(|addr| gb.pic.borrow().enable.borrow().read(addr))
            .for_each(|byte| assert_eq!(byte, 0x80));
    }

    #[test]
    fn mmu_read_write_works() {
        let gb = setup();

        // Disable boot ROM
        gb.mmu.borrow_mut().write(0xff50, 0x01);

        // Read from every address
        (0x0000..=0xffff).for_each(|addr| {
            gb.mmu.borrow().read(addr);
        });

        // Write to every address
        (0x0000..=0xffff).for_each(|addr| {
            gb.mmu.borrow_mut().write(addr, 0xaa);
        });

        // Ensure ROMs didn't get overwritten
        assert!((0x0000..=0x7fff)
            .map(|addr| gb.mmu.borrow().read(addr))
            .any(|byte| byte != 0xaa));

        // Ensure RAM, registers did get overwritten
        (0x8000..=0xffff)
            .filter(|addr| ![0xff00].contains(addr))
            .map(|addr| gb.mmu.borrow().read(addr))
            .for_each(|byte| assert_eq!(byte, 0xaa));
    }

    #[test]
    #[should_panic]
    fn mmu_rom_write_boot_panics() {
        let gb = setup();

        // Perform a write to the boot ROM
        gb.mmu.borrow_mut().write(0x0000, 0xaa);
    }

    #[test]
    fn boot_disable_works() {
        let gb = setup();

        // Ensure boot ROM starts enabled
        (0x0000..=0x0100)
            .map(|addr| gb.mmu.borrow().read(addr))
            .zip([
                0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26,
                0xff, 0x0e, 0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77,
                0x77, 0x3e, 0xfc, 0xe0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95,
                0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b, 0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06,
                0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9, 0x3e, 0x19, 0xea, 0x10, 0x99, 0x21,
                0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20, 0xf9, 0x2e, 0x0f, 0x18,
                0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04, 0x1e, 0x02,
                0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
                0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64,
                0x20, 0x06, 0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15,
                0x20, 0xd2, 0x05, 0x20, 0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb,
                0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17, 0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9,
                0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c,
                0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6,
                0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc,
                0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
                0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe,
                0x34, 0x20, 0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe,
                0x3e, 0x01, 0xe0, 0x50,
            ])
            .for_each(|(read, rom)| assert_eq!(read, rom));

        // Disable boot ROM
        gb.mmu.borrow_mut().write(0xff50, 0x01);

        // Check if disable was successful
        (0x0000..=0x0100)
            .map(|addr| gb.mmu.borrow().read(addr))
            .zip([
                0xc3, 0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x8b, 0x02, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x87, 0xe1,
                0x5f, 0x16, 0x00, 0x19, 0x5e, 0x23, 0x56, 0xd5, 0xe1, 0xe9, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc3, 0xfd, 0x01, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xc3, 0x12, 0x27, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc3, 0x12, 0x27, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xc3, 0x7e, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0x00, 0xc3, 0x50, 0x01, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d,
                0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f,
                0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb,
                0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x02, 0x01, 0x00, 0x00, 0xdc, 0x31, 0xbb,
            ])
            .for_each(|(read, rom)| assert_eq!(read, rom));
    }
}
