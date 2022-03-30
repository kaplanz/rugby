use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use log::{debug, trace};
use remus::bus::Bus;
use remus::reg::Register;
use remus::{Block, Device, Machine};

use self::inst::Instruction;
use crate::util::Bitflags;

mod inst;

#[derive(Debug, Default)]
pub struct Cpu {
    pub bus: Rc<RefCell<Bus>>,
    regs: Registers,
    status: Status,
    cycle: usize,
    ime: bool,
    prefix: bool,
}

impl Cpu {
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

impl Block for Cpu {
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Machine for Cpu {
    fn setup(&mut self) {
        self.status = Status::Enabled;
    }

    fn enabled(&self) -> bool {
        matches!(self.status, Status::Enabled)
    }

    fn cycle(&mut self) {
        if self.cycle == 0 {
            // Read the next instruction
            let opcode = self.fetchbyte();
            // Decode the instruction
            let inst = if !self.prefix {
                Instruction::new(opcode)
            } else {
                self.prefix = false;
                Instruction::prefixed(opcode)
            };
            debug!("{:#06x}: {inst}", self.regs.pc.wrapping_sub(1));
            // Execute the instruction
            self.cycle = inst.exec(self);
            trace!("Registers:\n{}", self.regs);
        }
        self.cycle -= 1;
    }
}

#[derive(Debug)]
struct Registers {
    a: Register<u8>,
    f: Register<u8>,
    af: WideRegister,
    b: Register<u8>,
    c: Register<u8>,
    bc: WideRegister,
    d: Register<u8>,
    e: Register<u8>,
    de: WideRegister,
    h: Register<u8>,
    l: Register<u8>,
    hl: WideRegister,
    sp: Register<u16>,
    pc: Register<u16>,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: Default::default(),
            f: Default::default(),
            af: WideRegister {
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
            bc: WideRegister {
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
            de: WideRegister {
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
            hl: WideRegister {
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
struct WideRegister {
    get: fn(&Registers) -> u16,
    set: fn(&mut Registers, u16),
}

impl WideRegister {
    pub fn get(&self, regs: &Registers) -> u16 {
        (self.get)(regs)
    }

    pub fn set(&self, regs: &mut Registers, value: u16) {
        (self.set)(regs, value);
    }
}

impl Debug for WideRegister {
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

impl Bitflags for Flag {}

impl From<Flag> for u8 {
    fn from(value: Flag) -> Self {
        value as u8
    }
}

#[derive(Debug)]
enum Status {
    Enabled,
    Halted,
    Stopped,
}

impl Default for Status {
    fn default() -> Self {
        Self::Stopped
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Mode {
    Normal,
    Interrupt,
}
