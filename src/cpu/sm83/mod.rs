use std::fmt::{Debug, Display};

use log::{debug, info};
use remus::bus::Bus;
use remus::dev::Device;
use remus::reg::Register;

use self::inst::Instruction;

mod inst;

#[derive(Debug)]
pub struct Cpu {
    pub bus: Bus,
    regs: Registers,
    status: Status,
    ime: bool,
    prefixed: bool,
}

impl Cpu {
    pub fn reset(mut self) -> Self {
        drop(self);
        self = Self::default();
        self
    }

    pub fn start(&mut self) {
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
        info!("{:#06x}: {inst}", self.regs.pc.get().wrapping_sub(1));
        // Execute the instruction
        inst.exec(self);
        debug!("Registers:\n{}", self.regs);
    }

    fn fetchbyte(&mut self) -> u8 {
        let pc = self.regs.pc.get();
        let byte = self.bus.read(pc as usize);
        self.regs.pc.set(pc.wrapping_add(1));
        byte
    }

    fn readbyte(&mut self) -> u8 {
        let hl = self.regs.hl.get(&self.regs);
        self.bus.read(hl as usize)
    }

    fn writebyte(&mut self, byte: u8) {
        let hl = self.regs.hl.get(&self.regs);
        self.bus.write(hl as usize, byte);
    }

    fn fetchword(&mut self) -> u16 {
        let mut word = 0u16;
        let pc = self.regs.pc.get();
        word |= self.bus.read(pc as usize) as u16;
        self.regs.pc.set(pc.wrapping_add(1));
        let pc = self.regs.pc.get();
        word |= (self.bus.read(pc as usize) as u16) << 8;
        self.regs.pc.set(pc.wrapping_add(1));
        word
    }

    fn popword(&mut self) -> u16 {
        let mut word = 0u16;
        let sp = self.regs.sp.get();
        word |= self.bus.read(sp as usize) as u16;
        self.regs.sp.set(sp.wrapping_add(1));
        let sp = self.regs.sp.get();
        word |= (self.bus.read(sp as usize) as u16) << 8;
        self.regs.sp.set(sp.wrapping_add(1));
        word
    }

    fn pushword(&mut self, word: u16) {
        let word = word.to_le_bytes();
        let sp = self.regs.sp.get();
        self.regs.sp.set(sp.wrapping_sub(1));
        let sp = self.regs.sp.get();
        self.bus.write(sp as usize, word[1]);
        self.regs.sp.set(sp.wrapping_sub(1));
        let sp = self.regs.sp.get();
        self.bus.write(sp as usize, word[0]);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            bus: Bus::default(),
            regs: Registers::default(),
            status: Status::Stopped,
            ime: false,
            prefixed: false,
        }
    }
}

#[derive(Debug)]
struct Registers {
    a: Register<1>,
    f: Register<1>,
    af: PseudoRegister,
    b: Register<1>,
    c: Register<1>,
    bc: PseudoRegister,
    d: Register<1>,
    e: Register<1>,
    de: PseudoRegister,
    h: Register<1>,
    l: Register<1>,
    hl: PseudoRegister,
    sp: Register<2>,
    pc: Register<2>,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: Default::default(),
            f: Default::default(),
            af: PseudoRegister {
                get: |regs: &Registers| {
                    let a = regs.a.get() as u16;
                    let f = regs.f.get() as u16;
                    (a << 8) | f
                },
                set: |regs: &mut Registers, af: u16| {
                    regs.a.set(((af & 0xff00) >> 8) as u8);
                    regs.f.set((af & 0x00ff) as u8);
                },
            },
            b: Default::default(),
            c: Default::default(),
            bc: PseudoRegister {
                get: |regs: &Registers| {
                    let b = regs.b.get() as u16;
                    let c = regs.c.get() as u16;
                    (b << 8) | c
                },
                set: |regs: &mut Registers, bc: u16| {
                    regs.b.set(((bc & 0xff00) >> 8) as u8);
                    regs.c.set((bc & 0x00ff) as u8);
                },
            },
            d: Default::default(),
            e: Default::default(),
            de: PseudoRegister {
                get: |regs: &Registers| {
                    let d = regs.d.get() as u16;
                    let e = regs.e.get() as u16;
                    (d << 8) | e
                },
                set: |regs: &mut Registers, de: u16| {
                    regs.d.set(((de & 0xff00) >> 8) as u8);
                    regs.e.set((de & 0x00ff) as u8);
                },
            },
            h: Default::default(),
            l: Default::default(),
            hl: PseudoRegister {
                get: |regs: &Registers| {
                    let h = regs.h.get() as u16;
                    let l = regs.l.get() as u16;
                    (h << 8) | l
                },
                set: |regs: &mut Registers, hl: u16| {
                    regs.h.set(((hl & 0xff00) >> 8) as u8);
                    regs.l.set((hl & 0x00ff) as u8);
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
        writeln!(f, "│ A │ {:02x} │ F │ {:02x} │", self.a.get(), self.f.get())?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(f, "│ B │ {:02x} │ C │ {:02x} │", self.b.get(), self.c.get())?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(f, "│ D │ {:02x} │ E │ {:02x} │", self.d.get(), self.e.get())?;
        writeln!(f, "├───┼────┼───┼────┤")?;
        writeln!(f, "│ H │ {:02x} │ L │ {:02x} │", self.h.get(), self.l.get())?;
        writeln!(f, "├───┴────┼───┴────┤")?;
        writeln!(f, "│   SP   │  {:04x}  │", self.sp.get())?;
        writeln!(f, "├────────┼────────┤")?;
        writeln!(f, "│   PC   │  {:04x}  │", self.pc.get())?;
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
