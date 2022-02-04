use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use log::{debug, info};
use remus::bus::Bus;
use remus::reg::Register;
use remus::Device;

use self::inst::Instruction;

mod inst;

#[derive(Debug)]
pub struct Cpu {
    pub bus: Rc<RefCell<Bus>>,
    regs: Registers,
    status: Status,
    ime: bool,
    prefixed: bool,
}

impl Cpu {
    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn setup(&mut self) {
        self.status = Status::Enabled(Mode::Normal);
    }

    pub fn enabled(&self) -> bool {
        matches!(self.status, Status::Enabled(_))
    }

    pub fn cycle(&mut self) {
        // Read the next instruction
        let opcode = self.fetchbyte();
        // Decode the instruction
        let inst = if !self.prefixed {
            Instruction::new(opcode)
        } else {
            self.prefixed = false;
            Instruction::prefixed(opcode)
        };
        info!("{:#06x}: {inst}", self.regs.pc.wrapping_sub(1));
        // Execute the instruction
        inst.exec(self);
        debug!("Registers:\n{}", self.regs);
    }

    fn fetchbyte(&mut self) -> u8 {
        let pc = &mut *self.regs.pc;
        let byte = self.bus.borrow().read(*pc as usize);
        *self.regs.pc = pc.wrapping_add(1);
        byte
    }

    fn readbyte(&mut self) -> u8 {
        let hl = self.regs.hl.get(&self.regs);
        self.bus.borrow().read(hl as usize)
    }

    fn writebyte(&mut self, byte: u8) {
        let hl = self.regs.hl.get(&self.regs);
        self.bus.borrow_mut().write(hl as usize, byte);
    }

    fn fetchword(&mut self) -> u16 {
        let pc = &mut *self.regs.pc;
        let mut word = 0u16;
        word |= self.bus.borrow().read(*pc as usize) as u16;
        *pc = pc.wrapping_add(1);
        word |= (self.bus.borrow().read(*pc as usize) as u16) << 8;
        *pc = pc.wrapping_add(1);
        word
    }

    fn popword(&mut self) -> u16 {
        let sp = &mut *self.regs.sp;
        let mut word = 0u16;
        word |= self.bus.borrow().read(*sp as usize) as u16;
        *sp = sp.wrapping_add(1);
        word |= (self.bus.borrow().read(*sp as usize) as u16) << 8;
        *sp = sp.wrapping_add(1);
        word
    }

    fn pushword(&mut self, word: u16) {
        let sp = &mut *self.regs.sp;
        let word = word.to_le_bytes();
        *sp = sp.wrapping_sub(1);
        self.bus.borrow_mut().write(*sp as usize, word[1]);
        *sp = sp.wrapping_sub(1);
        self.bus.borrow_mut().write(*sp as usize, word[0]);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            bus: Default::default(),
            regs: Registers::default(),
            status: Status::Stopped,
            ime: false,
            prefixed: false,
        }
    }
}

#[derive(Debug)]
struct Registers {
    a: Register<u8>,
    f: Register<u8>,
    af: PseudoRegister,
    b: Register<u8>,
    c: Register<u8>,
    bc: PseudoRegister,
    d: Register<u8>,
    e: Register<u8>,
    de: PseudoRegister,
    h: Register<u8>,
    l: Register<u8>,
    hl: PseudoRegister,
    sp: Register<u16>,
    pc: Register<u16>,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: Default::default(),
            f: Default::default(),
            af: PseudoRegister {
                get: |regs: &Registers| {
                    let a = *regs.a as u16;
                    let f = *regs.f as u16;
                    (a << 8) | f
                },
                set: |regs: &mut Registers, af: u16| {
                    *regs.a = ((af & 0xff00) >> 8) as u8;
                    *regs.f = (af & 0x00ff) as u8;
                },
            },
            b: Default::default(),
            c: Default::default(),
            bc: PseudoRegister {
                get: |regs: &Registers| {
                    let b = *regs.b as u16;
                    let c = *regs.c as u16;
                    (b << 8) | c
                },
                set: |regs: &mut Registers, bc: u16| {
                    *regs.b = ((bc & 0xff00) >> 8) as u8;
                    *regs.c = (bc & 0x00ff) as u8;
                },
            },
            d: Default::default(),
            e: Default::default(),
            de: PseudoRegister {
                get: |regs: &Registers| {
                    let d = *regs.d as u16;
                    let e = *regs.e as u16;
                    (d << 8) | e
                },
                set: |regs: &mut Registers, de: u16| {
                    *regs.d = ((de & 0xff00) >> 8) as u8;
                    *regs.e = (de & 0x00ff) as u8;
                },
            },
            h: Default::default(),
            l: Default::default(),
            hl: PseudoRegister {
                get: |regs: &Registers| {
                    let h = *regs.h as u16;
                    let l = *regs.l as u16;
                    (h << 8) | l
                },
                set: |regs: &mut Registers, hl: u16| {
                    *regs.h = ((hl & 0xff00) >> 8) as u8;
                    *regs.l = (hl & 0x00ff) as u8;
                },
            },
            sp: Default::default(),
            pc: Default::default(),
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌───┬────┬───┬────┐")?;
        writeln!(f, "│ A │ {:02x} │ F │ {:02x} │", *self.a, *self.f)?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(f, "│ B │ {:02x} │ C │ {:02x} │", *self.b, *self.c)?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(f, "│ D │ {:02x} │ E │ {:02x} │", *self.d, *self.e)?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(f, "│ H │ {:02x} │ L │ {:02x} │", *self.h, *self.l)?;
        writeln!(f, "├───┴────┼───┴────┤")?;
        writeln!(f, "│   SP   │  {:04x}  │", *self.sp)?;
        writeln!(f, "├────────┼────────┤")?;
        writeln!(f, "│   PC   │  {:04x}  │", *self.pc)?;
        write!(f, "└────────┴────────┘")
    }
}

#[derive(Copy, Clone)]
struct PseudoRegister {
    get: fn(&Registers) -> u16,
    set: fn(&mut Registers, u16),
}

impl PseudoRegister {
    pub fn get(&self, regs: &Registers) -> u16 {
        (self.get)(regs)
    }

    pub fn set(&self, regs: &mut Registers, value: u16) {
        (self.set)(regs, value);
    }
}

impl Debug for PseudoRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PseudoRegister")
    }
}

#[derive(Copy, Clone, Debug)]
enum Flag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}

impl Flag {
    pub fn get(self, f: &u8) -> bool {
        *f & self as u8 != 0
    }

    pub fn set(self, f: &mut u8, enable: bool) {
        *f ^= (*f & self as u8) ^ (!(enable as u8).wrapping_sub(1) & self as u8)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Enabled(Mode),
    Halted,
    Stopped,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Mode {
    Normal,
    Interrupt,
}
