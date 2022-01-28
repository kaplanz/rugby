use std::fmt::Display;
use std::ops::{BitAnd, BitOr, BitXor};

use remus::dev::Device;

use super::{Cpu, Flag, Status};

#[derive(Clone)]
pub struct Instruction {
    opcode: u8,
    exec: fn(&Instruction, &mut Cpu),
    fmt: &'static str,
}

impl Instruction {
    pub fn new(opcode: u8) -> Self {
        DECODE[opcode as usize].clone()
    }

    pub fn prefixed(opcode: u8) -> Self {
        PREFIX[opcode as usize].clone()
    }

    pub fn exec(self, cpu: &mut Cpu) {
        (self.exec)(&self, cpu);
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt)
    }
}

impl Instruction {
    fn adc(&self, cpu: &mut Cpu) {
        // Execute adc
        let op2 = match self.opcode {
            0x88..=0x8f => helpers::get_op8(cpu, self.opcode & 0x07),
            0xce => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let cin = Flag::C.get(&*cpu.regs.f) as u8;
        let (res, carry0) = cpu.regs.a.overflowing_add(op2);
        let (res, carry1) = res.overflowing_add(cin);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(
            flags,
            (res & 0x0f) > (*cpu.regs.a & 0x0f) + (op2 & 0x0f) + (cin & 0x0f),
        );
        Flag::C.set(flags, carry0 | carry1);
    }

    fn add8(&self, cpu: &mut Cpu) {
        // Execute add8
        let op2 = match self.opcode {
            0x80..=0x87 => helpers::get_op8(cpu, self.opcode & 0x07),
            0xc6 => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let (res, carry) = cpu.regs.a.overflowing_add(op2);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, (res & 0x0f) > (*cpu.regs.a & 0x0f) + (op2 & 0x0f));
        Flag::C.set(flags, carry);
    }

    fn add16(&self, cpu: &mut Cpu) {
        // Execute add16
        let hl = cpu.regs.hl;
        let op1 = hl.get(&cpu.regs);
        let op2 = match self.opcode {
            0x09 => cpu.regs.bc.get(&cpu.regs),
            0x19 => cpu.regs.de.get(&cpu.regs),
            0x29 => cpu.regs.hl.get(&cpu.regs),
            0x39 => *cpu.regs.sp,
            _ => panic!("Illegal instruction."),
        };
        let (res, carry) = op1.overflowing_add(op2);
        hl.set(&mut cpu.regs, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::N.set(flags, false);
        Flag::H.set(flags, (res & 0x0f) > (op1 & 0x0f) + (op2 & 0x0f));
        Flag::C.set(flags, carry);
    }

    fn and8(&self, cpu: &mut Cpu) {
        // Execute and8
        let op2 = match self.opcode {
            0xa0..=0xa0 => helpers::get_op8(cpu, self.opcode & 0x07),
            0xe6 => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let res = cpu.regs.a.bitand(op2);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, true);
        Flag::C.set(flags, false);
    }

    fn bit(&self, cpu: &mut Cpu) {
        // Execute bit
        let op1 = (self.opcode & 0x38) >> 3;
        let op2 = helpers::get_op8(cpu, self.opcode & 0x07);
        let res = (0b1 << op1) & op2;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, true);
    }

    fn call(&self, cpu: &mut Cpu) {
        // Execute call
        let op1 = cpu.fetchword();
        let flags = &mut cpu.regs.f;
        let cond = match self.opcode {
            0xc4 => !Flag::Z.get(flags),
            0xcc => Flag::Z.get(flags),
            0xcd => true,
            0xd4 => !Flag::C.get(flags),
            0xdc => Flag::C.get(flags),
            _ => panic!("Illegal instruction."),
        };
        if cond {
            cpu.pushword(*cpu.regs.pc);
            *cpu.regs.pc = op1;
        }
    }

    fn ccf(&self, cpu: &mut Cpu) {
        // Execute ccf
        let flags = &mut cpu.regs.f;
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        let carry = Flag::C.get(flags);
        Flag::C.set(flags, !carry);
    }

    fn cp(&self, cpu: &mut Cpu) {
        // Execute cp
        let op2 = match self.opcode {
            0xb8..=0xbf => helpers::get_op8(cpu, self.opcode & 0x07),
            0xfe => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let (res, carry) = cpu.regs.a.overflowing_sub(op2);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, true);
        let (resl, accl, op2l) = (
            (res & 0x0f | 0xf0) as i8,
            (*cpu.regs.a & 0x0f | 0xf0) as i8,
            (op2 & 0x0f | 0xf0) as i8,
        );
        Flag::H.set(flags, resl > accl - op2l);
        Flag::C.set(flags, carry);
    }

    fn cpl(&self, cpu: &mut Cpu) {
        // Execute cpl
        let res = !*cpu.regs.a;
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, res & 0x0f == 0);
    }

    fn daa(&self, cpu: &mut Cpu) {
        // Execute daa
        let mut res = *cpu.regs.a;
        let hcarry = (res & 0x0f) > 0x09;
        if hcarry {
            res += 0x06;
        }
        let carry = (res & 0xf0) > 0x90;
        if carry {
            res += 0x60;
        }
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::H.set(flags, hcarry);
        Flag::C.set(flags, carry);
    }

    fn dec8(&self, cpu: &mut Cpu) {
        // Execute dec8
        let res = if self.opcode == 0x35 {
            let op1 = cpu.readbyte();
            let res = op1.wrapping_sub(1);
            cpu.writebyte(res);
            res
        } else {
            let op1 = match self.opcode {
                0x05 => &mut *cpu.regs.b,
                0x0d => &mut *cpu.regs.c,
                0x15 => &mut *cpu.regs.d,
                0x1d => &mut *cpu.regs.e,
                0x25 => &mut *cpu.regs.h,
                0x2d => &mut *cpu.regs.l,
                0x3d => &mut *cpu.regs.a,
                _ => panic!("Illegal instruction."),
            };
            let res = op1.wrapping_sub(1);
            *op1 = res;
            res
        };
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, res & 0x0f == 0);
    }

    fn dec16(&self, cpu: &mut Cpu) {
        // Execute dec16
        if self.opcode == 0x3b {
            let op1 = &mut *cpu.regs.sp;
            let res = op1.wrapping_sub(1);
            *op1 = res;
        } else {
            let op1 = match self.opcode {
                0x0b => cpu.regs.bc,
                0x1b => cpu.regs.de,
                0x2b => cpu.regs.hl,
                _ => panic!("Illegal instruction."),
            };
            let res = op1.get(&cpu.regs).wrapping_sub(1);
            op1.set(&mut cpu.regs, res);
        }
    }

    fn di(&self, cpu: &mut Cpu) {
        // Execute di
        cpu.ime = false;
    }

    fn ei(&self, cpu: &mut Cpu) {
        // Execute ei
        cpu.ime = true;
    }

    fn halt(&self, cpu: &mut Cpu) {
        // Execute halt
        cpu.status = Status::Halted;
    }

    fn inc8(&self, cpu: &mut Cpu) {
        // Execute inc8
        let res = if self.opcode == 0x34 {
            let op1 = cpu.readbyte();
            let res = op1.wrapping_add(1);
            cpu.writebyte(res);
            res
        } else {
            let op1 = match self.opcode {
                0x04 => &mut *cpu.regs.b,
                0x0c => &mut *cpu.regs.c,
                0x14 => &mut *cpu.regs.d,
                0x1c => &mut *cpu.regs.e,
                0x24 => &mut *cpu.regs.h,
                0x2c => &mut *cpu.regs.l,
                0x3c => &mut *cpu.regs.a,
                _ => panic!("Illegal instruction."),
            };
            let res = op1.wrapping_add(1);
            *op1 = res;
            res
        };
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, res & 0x0f == 0);
    }

    fn inc16(&self, cpu: &mut Cpu) {
        // Execute inc16
        if self.opcode == 0x33 {
            let op1 = &mut *cpu.regs.sp;
            let res = op1.wrapping_add(1);
            *op1 = res;
        } else {
            let op1 = match self.opcode {
                0x03 => cpu.regs.bc,
                0x13 => cpu.regs.de,
                0x23 => cpu.regs.hl,
                _ => panic!("Illegal instruction."),
            };
            let res = op1.get(&cpu.regs).wrapping_add(1);
            op1.set(&mut cpu.regs, res);
        }
    }

    fn jp(&self, cpu: &mut Cpu) {
        // Execute jp
        let op1 = match self.opcode {
            0xc2 | 0xc3 | 0xca | 0xd2 | 0xda => cpu.fetchword(),
            0xe9 => cpu.regs.hl.get(&cpu.regs),
            _ => panic!("Illegal instruction."),
        };
        let flags = &mut cpu.regs.f;
        let cond = match self.opcode {
            0xc2 => !Flag::Z.get(flags),
            0xc3 => true,
            0xca => Flag::Z.get(flags),
            0xd2 => !Flag::C.get(flags),
            0xda => Flag::C.get(flags),
            0xe9 => true,
            _ => panic!("Illegal instruction."),
        };
        if cond {
            *cpu.regs.pc = op1;
        }
    }

    fn jr(&self, cpu: &mut Cpu) {
        // Execute jr
        let op1 = cpu.fetchbyte() as i8 as i16;
        let flags = &mut cpu.regs.f;
        let cond = match self.opcode {
            0x18 => true,
            0x20 => !Flag::Z.get(flags),
            0x28 => Flag::Z.get(flags),
            0x30 => !Flag::C.get(flags),
            0x38 => Flag::C.get(flags),
            _ => panic!("Illegal instruction."),
        };
        if cond {
            let pc = *cpu.regs.pc as i16;
            let res = pc.wrapping_add(op1) as u16;
            *cpu.regs.pc = res;
        }
    }

    fn ld8(&self, cpu: &mut Cpu) {
        // Execute ld8
        match self.opcode {
            0x02 => {
                let op2 = *cpu.regs.a;
                let addr = cpu.regs.bc.get(&cpu.regs);
                cpu.bus.borrow_mut().write(addr as usize, op2);
            }
            0x12 => {
                let op2 = *cpu.regs.a;
                let addr = cpu.regs.de.get(&cpu.regs);
                cpu.bus.borrow_mut().write(addr as usize, op2);
            }
            0x22 => {
                let op2 = *cpu.regs.a;
                let hl = cpu.regs.hl;
                let addr = hl.get(&cpu.regs);
                cpu.bus.borrow_mut().write(addr as usize, op2);
                hl.set(&mut cpu.regs, addr.wrapping_add(1));
            }
            0x32 => {
                let op2 = *cpu.regs.a;
                let hl = cpu.regs.hl;
                let addr = hl.get(&cpu.regs);
                cpu.bus.borrow_mut().write(addr as usize, op2);
                hl.set(&mut cpu.regs, addr.wrapping_sub(1));
            }
            0x0a => {
                let addr = cpu.regs.bc.get(&cpu.regs);
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
            }
            0x1a => {
                let addr = cpu.regs.de.get(&cpu.regs);
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
            }
            0x2a => {
                let hl = cpu.regs.hl;
                let addr = hl.get(&cpu.regs);
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
                hl.set(&mut cpu.regs, addr.wrapping_add(1));
            }
            0x3a => {
                let hl = cpu.regs.hl;
                let addr = hl.get(&cpu.regs);
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
                hl.set(&mut cpu.regs, addr.wrapping_sub(1));
            }
            0x06 | 0x16 | 0x26 | 0x36 | 0x0e | 0x1e | 0x2e | 0x3e => {
                let op2 = cpu.fetchbyte();
                helpers::set_op8(cpu, (self.opcode & 0x38) >> 3, op2);
            }
            0x76 => panic!("Illegal instruction."),
            0x40..=0x7f => {
                let op2 = helpers::get_op8(cpu, self.opcode & 0x07);
                helpers::set_op8(cpu, (self.opcode & 0x38) >> 3, op2);
            }
            0xea => {
                let op2 = *cpu.regs.a;
                let addr = cpu.fetchword();
                cpu.bus.borrow_mut().write(addr as usize, op2);
            }
            0xfa => {
                let addr = cpu.fetchword();
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
            }
            _ => panic!("Illegal instruction."),
        }
    }

    fn ld16(&self, cpu: &mut Cpu) {
        // Execute ld16
        let op2 = cpu.fetchword();
        match self.opcode {
            0x01 => (cpu.regs.bc.set)(&mut cpu.regs, op2),
            0x11 => (cpu.regs.de.set)(&mut cpu.regs, op2),
            0x21 => (cpu.regs.hl.set)(&mut cpu.regs, op2),
            0x31 => *cpu.regs.sp = op2,
            _ => panic!("Illegal instruction."),
        }
    }

    fn ldh(&self, cpu: &mut Cpu) {
        // Execute ldh
        match self.opcode {
            0xe0 => {
                let op2 = *cpu.regs.a;
                let addr = 0xff00 | cpu.fetchbyte() as u16;
                cpu.bus.borrow_mut().write(addr as usize, op2);
            }
            0xe2 => {
                let op2 = *cpu.regs.a;
                let addr = 0xff00 | *cpu.regs.c as u16;
                cpu.bus.borrow_mut().write(addr as usize, op2);
            }
            0xf0 => {
                let addr = 0xff00 | cpu.fetchbyte() as u16;
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
            }
            0xf2 => {
                let addr = 0xff00 | *cpu.regs.c as u16;
                let op2 = cpu.bus.borrow().read(addr as usize);
                *cpu.regs.a = op2;
            }
            _ => panic!("Illegal instruction."),
        }
    }

    fn nop(&self, _: &mut Cpu) {}

    fn or8(&self, cpu: &mut Cpu) {
        // Execute or8
        let op2 = match self.opcode {
            0xb0..=0xb0 => helpers::get_op8(cpu, self.opcode & 0x07),
            0xf6 => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let res = cpu.regs.a.bitor(op2);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, false);
    }

    fn pop(&self, cpu: &mut Cpu) {
        // Execute pop
        let op1 = match self.opcode {
            0xc1 => cpu.regs.bc,
            0xd1 => cpu.regs.de,
            0xe1 => cpu.regs.hl,
            0xf1 => cpu.regs.af,
            _ => panic!("Illegal instruction."),
        };
        let word = cpu.popword();
        op1.set(&mut cpu.regs, word);
    }

    fn prefix(&self, cpu: &mut Cpu) {
        // Execute prefix
        cpu.prefixed = true;
    }

    fn push(&self, cpu: &mut Cpu) {
        // Execute push
        let op1 = match self.opcode {
            0xc5 => cpu.regs.bc,
            0xd5 => cpu.regs.de,
            0xe5 => cpu.regs.hl,
            0xf5 => cpu.regs.af,
            _ => panic!("Illegal instruction."),
        };
        cpu.pushword(op1.get(&cpu.regs));
    }

    fn res(&self, cpu: &mut Cpu) {
        // Execute res
        let op1 = (self.opcode & 0x38) >> 3;
        let op2 = helpers::get_op8(cpu, self.opcode & 0x07);
        let mask = !(0b1 << op1);
        let res = mask & op2;
        helpers::set_op8(cpu, self.opcode & 0x07, res);
    }

    fn ret(&self, cpu: &mut Cpu) {
        // Execute ret
        let flags = &mut cpu.regs.f;
        let cond = match self.opcode {
            0xc0 => !Flag::Z.get(flags),
            0xc8 => Flag::Z.get(flags),
            0xc9 => true,
            0xd0 => !Flag::C.get(flags),
            0xd8 => Flag::C.get(flags),
            _ => panic!("Illegal instruction."),
        };
        if cond {
            let pc = cpu.popword();
            *cpu.regs.pc = pc;
        }
    }

    fn reti(&self, cpu: &mut Cpu) {
        // Execute reti
        let pc = cpu.popword();
        *cpu.regs.pc = pc;
        cpu.ime = true;
    }

    fn rl(&self, cpu: &mut Cpu) {
        // Execute rl
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let flags = &mut cpu.regs.f;
        let cin = Flag::C.get(flags);
        let carry = op1 & 0x80 != 0;
        let res = op1 << 1 | (cin as u8);
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rla(&self, cpu: &mut Cpu) {
        // Execute rla
        let flags = &mut cpu.regs.f;
        let cin = Flag::C.get(flags);
        let carry = *cpu.regs.a & 0x80 != 0;
        let res = *cpu.regs.a << 1 | (cin as u8);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, false);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rlc(&self, cpu: &mut Cpu) {
        // Execute rlc
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let carry = op1 & 0x80 != 0;
        let res = op1 << 1 | (carry as u8);
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rlca(&self, cpu: &mut Cpu) {
        // Execute rlca
        let carry = *cpu.regs.a & 0x80 != 0;
        let res = *cpu.regs.a << 1 | (carry as u8);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, false);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rr(&self, cpu: &mut Cpu) {
        // Execute rr
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let flags = &mut cpu.regs.f;
        let cin = Flag::C.get(flags);
        let carry = op1 & 0x01 != 0;
        let res = ((cin as u8) << 7) | op1 >> 1;
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rra(&self, cpu: &mut Cpu) {
        // Execute rra
        let flags = &mut cpu.regs.f;
        let cin = Flag::C.get(flags);
        let carry = *cpu.regs.a & 0x01 != 0;
        let res = ((cin as u8) << 7) | *cpu.regs.a >> 1;
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, false);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rrc(&self, cpu: &mut Cpu) {
        // Execute rrc
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let carry = op1 & 0x01 != 0;
        let res = ((carry as u8) << 7) | op1 >> 1;
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rrca(&self, cpu: &mut Cpu) {
        // Execute rrca
        let carry = *cpu.regs.a & 0x01 != 0;
        let res = ((carry as u8) << 7) | *cpu.regs.a >> 1;
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, false);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn rst(&self, cpu: &mut Cpu) {
        // Execute rst
        let op1 = match self.opcode {
            0xc7 => 0x00,
            0xcf => 0x08,
            0xd7 => 0x10,
            0xdf => 0x18,
            0xe7 => 0x20,
            0xef => 0x28,
            0xf7 => 0x30,
            0xff => 0x38,
            _ => panic!("Illegal instruction."),
        };
        *cpu.regs.pc = op1;
    }

    fn sbc8(&self, cpu: &mut Cpu) {
        // Execute sbc8
        let op2 = match self.opcode {
            0x98..=0x9f => helpers::get_op8(cpu, self.opcode & 0x07),
            0xde => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let cin = Flag::C.get(&*cpu.regs.f) as u8;
        let (res, carry0) = cpu.regs.a.overflowing_sub(op2);
        let (res, carry1) = res.overflowing_sub(cin);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, true);
        let (resl, accl, op2l, cinl) = (
            (res & 0x0f | 0xf0),
            (*cpu.regs.a & 0x0f | 0xf0),
            (op2 & 0x0f | 0xf0),
            (cin & 0x0f | 0xf0),
        );
        Flag::H.set(flags, resl > accl - op2l - cinl);
        Flag::C.set(flags, carry0 | carry1);
    }

    fn scf(&self, cpu: &mut Cpu) {
        // Execute scf
        let flags = &mut cpu.regs.f;
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, true);
    }

    fn set(&self, cpu: &mut Cpu) {
        // Execute set
        let op1 = (self.opcode & 0x38) >> 3;
        let op2 = helpers::get_op8(cpu, self.opcode & 0x07);
        let mask = !(0b1 << op1);
        let res = (mask & op2) | !mask;
        helpers::set_op8(cpu, self.opcode & 0x07, res);
    }

    fn sla(&self, cpu: &mut Cpu) {
        // Execute sla
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let carry = op1 & 0x80 != 0;
        let res = op1 << 1;
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn sra(&self, cpu: &mut Cpu) {
        // Execute sra
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let sign = op1 & 0x80;
        let carry = op1 & 0x01 != 0;
        let res = sign | (op1 >> 1);
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn srl(&self, cpu: &mut Cpu) {
        // Execute srl
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let carry = op1 & 0x01 != 0;
        let res = 0x7f & (op1 >> 1);
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, carry);
    }

    fn stop(&self, cpu: &mut Cpu) {
        // Execute stop
        cpu.status = Status::Stopped;
    }

    fn sub(&self, cpu: &mut Cpu) {
        // Execute sub
        let op2 = match self.opcode {
            0x90..=0x90 => helpers::get_op8(cpu, self.opcode & 0x07),
            0xd6 => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let (res, carry) = cpu.regs.a.overflowing_sub(op2);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, true);
        let (resl, accl, op2l) = (
            (res & 0x0f | 0xf0) as i8,
            (*cpu.regs.a & 0x0f | 0xf0) as i8,
            (op2 & 0x0f | 0xf0) as i8,
        );
        Flag::H.set(flags, resl > accl - op2l);
        Flag::C.set(flags, carry);
    }

    fn swap(&self, cpu: &mut Cpu) {
        // Execute swap
        let op1 = helpers::get_op8(cpu, self.opcode & 0x07);
        let res = ((op1 & 0xf0) >> 4) | ((op1 ^ 0x0f) << 4);
        helpers::set_op8(cpu, self.opcode & 0x07, res);
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, false);
    }

    fn unused(&self, _: &mut Cpu) {
        panic!("Illegal instruction.");
    }

    fn xor8(&self, cpu: &mut Cpu) {
        // Execute xor8
        let op2 = match self.opcode {
            0xa8..=0xaf => helpers::get_op8(cpu, self.opcode & 0x07),
            0xee => cpu.fetchbyte(),
            _ => panic!("Illegal instruction."),
        };
        let res = cpu.regs.a.bitxor(op2);
        *cpu.regs.a = res;
        // Set flags
        let flags = &mut cpu.regs.f;
        Flag::Z.set(flags, res == 0);
        Flag::N.set(flags, false);
        Flag::H.set(flags, false);
        Flag::C.set(flags, false);
    }
}

mod helpers {
    use super::*;

    pub fn get_op8(cpu: &mut Cpu, idx: u8) -> u8 {
        match idx {
            0x0 => *cpu.regs.b,
            0x1 => *cpu.regs.c,
            0x2 => *cpu.regs.d,
            0x3 => *cpu.regs.e,
            0x4 => *cpu.regs.h,
            0x5 => *cpu.regs.l,
            0x6 => cpu.readbyte(),
            0x7 => *cpu.regs.a,
            _ => panic!("Illegal register."),
        }
    }

    pub fn set_op8(cpu: &mut Cpu, idx: u8, value: u8) {
        match idx {
            0x0 => *cpu.regs.b = value,
            0x1 => *cpu.regs.c = value,
            0x2 => *cpu.regs.d = value,
            0x3 => *cpu.regs.e = value,
            0x4 => *cpu.regs.h = value,
            0x5 => *cpu.regs.l = value,
            0x6 => cpu.writebyte(value),
            0x7 => *cpu.regs.a = value,
            _ => panic!("Illegal register."),
        };
    }
}

#[rustfmt::skip]
const DECODE: [Instruction; 0x100] = [
    Instruction { opcode: 0x00, exec: Instruction::nop,    fmt: "NOP"            },
    Instruction { opcode: 0x01, exec: Instruction::ld16,   fmt: "LD BC, d16"     },
    Instruction { opcode: 0x02, exec: Instruction::ld8,    fmt: "LD (BC), A"     },
    Instruction { opcode: 0x03, exec: Instruction::inc16,  fmt: "INC BC"         },
    Instruction { opcode: 0x04, exec: Instruction::inc8,   fmt: "INC B"          },
    Instruction { opcode: 0x05, exec: Instruction::dec8,   fmt: "DEC B"          },
    Instruction { opcode: 0x06, exec: Instruction::ld8,    fmt: "LD B, d8"       },
    Instruction { opcode: 0x07, exec: Instruction::rlca,   fmt: "RLCA"           },
    Instruction { opcode: 0x08, exec: Instruction::ld16,   fmt: "LD (a16), SP"   },
    Instruction { opcode: 0x09, exec: Instruction::add16,  fmt: "ADD HL, BC"     },
    Instruction { opcode: 0x0a, exec: Instruction::ld8,    fmt: "LD A, (BC)"     },
    Instruction { opcode: 0x0b, exec: Instruction::dec16,  fmt: "DEC BC"         },
    Instruction { opcode: 0x0c, exec: Instruction::inc8,   fmt: "INC C"          },
    Instruction { opcode: 0x0d, exec: Instruction::dec8,   fmt: "DEC C"          },
    Instruction { opcode: 0x0e, exec: Instruction::ld8,    fmt: "LD C, d8"       },
    Instruction { opcode: 0x0f, exec: Instruction::rrca,   fmt: "RRCA"           },
    Instruction { opcode: 0x10, exec: Instruction::stop,   fmt: "STOP"           },
    Instruction { opcode: 0x11, exec: Instruction::ld16,   fmt: "LD DE, (d16)"   },
    Instruction { opcode: 0x12, exec: Instruction::ld8,    fmt: "LD (DE), A"     },
    Instruction { opcode: 0x13, exec: Instruction::inc16,  fmt: "INC DE"         },
    Instruction { opcode: 0x14, exec: Instruction::inc8,   fmt: "INC D"          },
    Instruction { opcode: 0x15, exec: Instruction::dec8,   fmt: "DEC D"          },
    Instruction { opcode: 0x16, exec: Instruction::ld8,    fmt: "LD D, d8"       },
    Instruction { opcode: 0x17, exec: Instruction::rla,    fmt: "RLA"            },
    Instruction { opcode: 0x18, exec: Instruction::jr,     fmt: "JR r8"          },
    Instruction { opcode: 0x19, exec: Instruction::add16,  fmt: "ADD HL, DE"     },
    Instruction { opcode: 0x1a, exec: Instruction::ld8,    fmt: "LD A, (DE)"     },
    Instruction { opcode: 0x1b, exec: Instruction::dec16,  fmt: "DEC DE"         },
    Instruction { opcode: 0x1c, exec: Instruction::inc8,   fmt: "INC E"          },
    Instruction { opcode: 0x1d, exec: Instruction::dec8,   fmt: "DEC E"          },
    Instruction { opcode: 0x1e, exec: Instruction::ld8,    fmt: "LD E, d8"       },
    Instruction { opcode: 0x1f, exec: Instruction::rra,    fmt: "RRA"            },
    Instruction { opcode: 0x20, exec: Instruction::jr,     fmt: "JR NZ, r8"      },
    Instruction { opcode: 0x21, exec: Instruction::ld16,   fmt: "LD HL, d16"     },
    Instruction { opcode: 0x22, exec: Instruction::ld8,    fmt: "LD (HL+), A"    },
    Instruction { opcode: 0x23, exec: Instruction::inc16,  fmt: "INC HL"         },
    Instruction { opcode: 0x24, exec: Instruction::inc8,   fmt: "INC H"          },
    Instruction { opcode: 0x25, exec: Instruction::dec8,   fmt: "DEC H"          },
    Instruction { opcode: 0x26, exec: Instruction::ld8,    fmt: "LD H, d8"       },
    Instruction { opcode: 0x27, exec: Instruction::daa,    fmt: "DAA"            },
    Instruction { opcode: 0x28, exec: Instruction::jr,     fmt: "JR Z, r8"       },
    Instruction { opcode: 0x29, exec: Instruction::add16,  fmt: "ADD HL, HL"     },
    Instruction { opcode: 0x2a, exec: Instruction::ld8,    fmt: "LD A, (HL+)"    },
    Instruction { opcode: 0x2b, exec: Instruction::dec16,  fmt: "DEC HL"         },
    Instruction { opcode: 0x2c, exec: Instruction::inc8,   fmt: "INC L"          },
    Instruction { opcode: 0x2d, exec: Instruction::dec8,   fmt: "DEC L"          },
    Instruction { opcode: 0x2e, exec: Instruction::ld8,    fmt: "LD L, d8"       },
    Instruction { opcode: 0x2f, exec: Instruction::cpl,    fmt: "CPL"            },
    Instruction { opcode: 0x30, exec: Instruction::jr,     fmt: "JR NC, r8"      },
    Instruction { opcode: 0x31, exec: Instruction::ld16,   fmt: "LD SP, d16"     },
    Instruction { opcode: 0x32, exec: Instruction::ld8,    fmt: "LD (HL-), A"    },
    Instruction { opcode: 0x33, exec: Instruction::inc16,  fmt: "INC SP"         },
    Instruction { opcode: 0x34, exec: Instruction::inc16,  fmt: "INC (HL)"       },
    Instruction { opcode: 0x35, exec: Instruction::dec16,  fmt: "DEC (HL)"       },
    Instruction { opcode: 0x36, exec: Instruction::ld8,    fmt: "LD (HL), d8"    },
    Instruction { opcode: 0x37, exec: Instruction::scf,    fmt: "SCF"            },
    Instruction { opcode: 0x38, exec: Instruction::jr,     fmt: "JR C, r8"       },
    Instruction { opcode: 0x39, exec: Instruction::add16,  fmt: "ADD HL, SP"     },
    Instruction { opcode: 0x3a, exec: Instruction::ld8,    fmt: "LD A, (HL-)"    },
    Instruction { opcode: 0x3b, exec: Instruction::dec16,  fmt: "DEC SP"         },
    Instruction { opcode: 0x3c, exec: Instruction::inc8,   fmt: "INC A"          },
    Instruction { opcode: 0x3d, exec: Instruction::dec8,   fmt: "DEC A"          },
    Instruction { opcode: 0x3e, exec: Instruction::ld8,    fmt: "LD A, d8"       },
    Instruction { opcode: 0x3f, exec: Instruction::ccf,    fmt: "CCP"            },
    Instruction { opcode: 0x40, exec: Instruction::ld8,    fmt: "LD B, B"        },
    Instruction { opcode: 0x41, exec: Instruction::ld8,    fmt: "LD B, C"        },
    Instruction { opcode: 0x42, exec: Instruction::ld8,    fmt: "LD B, D"        },
    Instruction { opcode: 0x43, exec: Instruction::ld8,    fmt: "LD B, E"        },
    Instruction { opcode: 0x44, exec: Instruction::ld8,    fmt: "LD B, H"        },
    Instruction { opcode: 0x45, exec: Instruction::ld8,    fmt: "LD B, L"        },
    Instruction { opcode: 0x46, exec: Instruction::ld8,    fmt: "LD B, (HL)"     },
    Instruction { opcode: 0x47, exec: Instruction::ld8,    fmt: "LD B, A"        },
    Instruction { opcode: 0x48, exec: Instruction::ld8,    fmt: "LD C, B"        },
    Instruction { opcode: 0x49, exec: Instruction::ld8,    fmt: "LD C, C"        },
    Instruction { opcode: 0x4a, exec: Instruction::ld8,    fmt: "LD C, D"        },
    Instruction { opcode: 0x4b, exec: Instruction::ld8,    fmt: "LD C, E"        },
    Instruction { opcode: 0x4c, exec: Instruction::ld8,    fmt: "LD C, H"        },
    Instruction { opcode: 0x4d, exec: Instruction::ld8,    fmt: "LD C, L"        },
    Instruction { opcode: 0x4e, exec: Instruction::ld8,    fmt: "LD C, (HL)"     },
    Instruction { opcode: 0x4f, exec: Instruction::ld8,    fmt: "LD C, A"        },
    Instruction { opcode: 0x50, exec: Instruction::ld8,    fmt: "LD D, B"        },
    Instruction { opcode: 0x51, exec: Instruction::ld8,    fmt: "LD D, C"        },
    Instruction { opcode: 0x52, exec: Instruction::ld8,    fmt: "LD D, D"        },
    Instruction { opcode: 0x53, exec: Instruction::ld8,    fmt: "LD D, E"        },
    Instruction { opcode: 0x54, exec: Instruction::ld8,    fmt: "LD D, H"        },
    Instruction { opcode: 0x55, exec: Instruction::ld8,    fmt: "LD D, L"        },
    Instruction { opcode: 0x56, exec: Instruction::ld8,    fmt: "LD D, (HL)"     },
    Instruction { opcode: 0x57, exec: Instruction::ld8,    fmt: "LD D, A"        },
    Instruction { opcode: 0x58, exec: Instruction::ld8,    fmt: "LD E, B"        },
    Instruction { opcode: 0x59, exec: Instruction::ld8,    fmt: "LD E, C"        },
    Instruction { opcode: 0x5a, exec: Instruction::ld8,    fmt: "LD E, D"        },
    Instruction { opcode: 0x5b, exec: Instruction::ld8,    fmt: "LD E, E"        },
    Instruction { opcode: 0x5c, exec: Instruction::ld8,    fmt: "LD E, H"        },
    Instruction { opcode: 0x5d, exec: Instruction::ld8,    fmt: "LD E, L"        },
    Instruction { opcode: 0x5e, exec: Instruction::ld8,    fmt: "LD E, (HL)"     },
    Instruction { opcode: 0x5f, exec: Instruction::ld8,    fmt: "LD E, A"        },
    Instruction { opcode: 0x60, exec: Instruction::ld8,    fmt: "LD H, B"        },
    Instruction { opcode: 0x61, exec: Instruction::ld8,    fmt: "LD H, C"        },
    Instruction { opcode: 0x62, exec: Instruction::ld8,    fmt: "LD H, D"        },
    Instruction { opcode: 0x63, exec: Instruction::ld8,    fmt: "LD H, E"        },
    Instruction { opcode: 0x64, exec: Instruction::ld8,    fmt: "LD H, H"        },
    Instruction { opcode: 0x65, exec: Instruction::ld8,    fmt: "LD H, L"        },
    Instruction { opcode: 0x66, exec: Instruction::ld8,    fmt: "LD H, (HL)"     },
    Instruction { opcode: 0x67, exec: Instruction::ld8,    fmt: "LD H, A"        },
    Instruction { opcode: 0x68, exec: Instruction::ld8,    fmt: "LD L, B"        },
    Instruction { opcode: 0x69, exec: Instruction::ld8,    fmt: "LD L, C"        },
    Instruction { opcode: 0x6a, exec: Instruction::ld8,    fmt: "LD L, D"        },
    Instruction { opcode: 0x6b, exec: Instruction::ld8,    fmt: "LD L, E"        },
    Instruction { opcode: 0x6c, exec: Instruction::ld8,    fmt: "LD L, H"        },
    Instruction { opcode: 0x6d, exec: Instruction::ld8,    fmt: "LD L, L"        },
    Instruction { opcode: 0x6e, exec: Instruction::ld8,    fmt: "LD L, (HL)"     },
    Instruction { opcode: 0x6f, exec: Instruction::ld8,    fmt: "LD L, A"        },
    Instruction { opcode: 0x70, exec: Instruction::ld8,    fmt: "LD (HL), B"     },
    Instruction { opcode: 0x71, exec: Instruction::ld8,    fmt: "LD (HL), C"     },
    Instruction { opcode: 0x72, exec: Instruction::ld8,    fmt: "LD (HL), D"     },
    Instruction { opcode: 0x73, exec: Instruction::ld8,    fmt: "LD (HL), E"     },
    Instruction { opcode: 0x74, exec: Instruction::ld8,    fmt: "LD (HL), H"     },
    Instruction { opcode: 0x75, exec: Instruction::ld8,    fmt: "LD (HL), L"     },
    Instruction { opcode: 0x76, exec: Instruction::halt,   fmt: "HALT"           },
    Instruction { opcode: 0x77, exec: Instruction::ld8,    fmt: "LD (HL), A"     },
    Instruction { opcode: 0x78, exec: Instruction::ld8,    fmt: "LD A, B"        },
    Instruction { opcode: 0x79, exec: Instruction::ld8,    fmt: "LD A, C"        },
    Instruction { opcode: 0x7a, exec: Instruction::ld8,    fmt: "LD A, D"        },
    Instruction { opcode: 0x7b, exec: Instruction::ld8,    fmt: "LD A, E"        },
    Instruction { opcode: 0x7c, exec: Instruction::ld8,    fmt: "LD A, H"        },
    Instruction { opcode: 0x7d, exec: Instruction::ld8,    fmt: "LD A, L"        },
    Instruction { opcode: 0x7e, exec: Instruction::ld8,    fmt: "LD A, (HL)"     },
    Instruction { opcode: 0x7f, exec: Instruction::ld8,    fmt: "LD A, A"        },
    Instruction { opcode: 0x80, exec: Instruction::add8,   fmt: "ADD A, B"       },
    Instruction { opcode: 0x81, exec: Instruction::add8,   fmt: "ADD A, C"       },
    Instruction { opcode: 0x82, exec: Instruction::add8,   fmt: "ADD A, D"       },
    Instruction { opcode: 0x83, exec: Instruction::add8,   fmt: "ADD A, E"       },
    Instruction { opcode: 0x84, exec: Instruction::add8,   fmt: "ADD A, H"       },
    Instruction { opcode: 0x85, exec: Instruction::add8,   fmt: "ADD A, L"       },
    Instruction { opcode: 0x86, exec: Instruction::add8,   fmt: "ADD A, (HL)"    },
    Instruction { opcode: 0x87, exec: Instruction::add8,   fmt: "ADD A, A"       },
    Instruction { opcode: 0x88, exec: Instruction::adc,    fmt: "ADC A, B"       },
    Instruction { opcode: 0x89, exec: Instruction::adc,    fmt: "ADC A, C"       },
    Instruction { opcode: 0x8a, exec: Instruction::adc,    fmt: "ADC A, D"       },
    Instruction { opcode: 0x8b, exec: Instruction::adc,    fmt: "ADC A, E"       },
    Instruction { opcode: 0x8c, exec: Instruction::adc,    fmt: "ADC A, H"       },
    Instruction { opcode: 0x8d, exec: Instruction::adc,    fmt: "ADC A, L"       },
    Instruction { opcode: 0x8e, exec: Instruction::adc,    fmt: "ADC A, (HL)"    },
    Instruction { opcode: 0x8f, exec: Instruction::adc,    fmt: "ADC A, A"       },
    Instruction { opcode: 0x90, exec: Instruction::sub,    fmt: "SUB A, B"       },
    Instruction { opcode: 0x91, exec: Instruction::sub,    fmt: "SUB A, C"       },
    Instruction { opcode: 0x92, exec: Instruction::sub,    fmt: "SUB A, D"       },
    Instruction { opcode: 0x93, exec: Instruction::sub,    fmt: "SUB A, E"       },
    Instruction { opcode: 0x94, exec: Instruction::sub,    fmt: "SUB A, H"       },
    Instruction { opcode: 0x95, exec: Instruction::sub,    fmt: "SUB A, L"       },
    Instruction { opcode: 0x96, exec: Instruction::sub,    fmt: "SUB A, (HL)"    },
    Instruction { opcode: 0x97, exec: Instruction::sub,    fmt: "SUB A, A"       },
    Instruction { opcode: 0x98, exec: Instruction::sbc8,   fmt: "SBC A, B"       },
    Instruction { opcode: 0x99, exec: Instruction::sbc8,   fmt: "SBC A, C"       },
    Instruction { opcode: 0x9a, exec: Instruction::sbc8,   fmt: "SBC A, D"       },
    Instruction { opcode: 0x9b, exec: Instruction::sbc8,   fmt: "SBC A, E"       },
    Instruction { opcode: 0x9c, exec: Instruction::sbc8,   fmt: "SBC A, H"       },
    Instruction { opcode: 0x9d, exec: Instruction::sbc8,   fmt: "SBC A, L"       },
    Instruction { opcode: 0x9e, exec: Instruction::sbc8,   fmt: "SBC A, (HL)"    },
    Instruction { opcode: 0x9f, exec: Instruction::sbc8,   fmt: "SBC A, A"       },
    Instruction { opcode: 0xa0, exec: Instruction::and8,   fmt: "AND B"          },
    Instruction { opcode: 0xa1, exec: Instruction::and8,   fmt: "AND C"          },
    Instruction { opcode: 0xa2, exec: Instruction::and8,   fmt: "AND D"          },
    Instruction { opcode: 0xa3, exec: Instruction::and8,   fmt: "AND E"          },
    Instruction { opcode: 0xa4, exec: Instruction::and8,   fmt: "AND H"          },
    Instruction { opcode: 0xa5, exec: Instruction::and8,   fmt: "AND L"          },
    Instruction { opcode: 0xa6, exec: Instruction::and8,   fmt: "AND (HL)"       },
    Instruction { opcode: 0xa7, exec: Instruction::and8,   fmt: "AND A"          },
    Instruction { opcode: 0xa8, exec: Instruction::xor8,   fmt: "XOR B"          },
    Instruction { opcode: 0xa9, exec: Instruction::xor8,   fmt: "XOR C"          },
    Instruction { opcode: 0xaa, exec: Instruction::xor8,   fmt: "XOR D"          },
    Instruction { opcode: 0xab, exec: Instruction::xor8,   fmt: "XOR E"          },
    Instruction { opcode: 0xac, exec: Instruction::xor8,   fmt: "XOR H"          },
    Instruction { opcode: 0xad, exec: Instruction::xor8,   fmt: "XOR L"          },
    Instruction { opcode: 0xae, exec: Instruction::xor8,   fmt: "XOR (HL)"       },
    Instruction { opcode: 0xaf, exec: Instruction::xor8,   fmt: "XOR A"          },
    Instruction { opcode: 0xb0, exec: Instruction::or8,    fmt: "OR B"           },
    Instruction { opcode: 0xb1, exec: Instruction::or8,    fmt: "OR C"           },
    Instruction { opcode: 0xb2, exec: Instruction::or8,    fmt: "OR D"           },
    Instruction { opcode: 0xb3, exec: Instruction::or8,    fmt: "OR E"           },
    Instruction { opcode: 0xb4, exec: Instruction::or8,    fmt: "OR H"           },
    Instruction { opcode: 0xb5, exec: Instruction::or8,    fmt: "OR L"           },
    Instruction { opcode: 0xb6, exec: Instruction::or8,    fmt: "OR (HL)"        },
    Instruction { opcode: 0xb7, exec: Instruction::or8,    fmt: "OR A"           },
    Instruction { opcode: 0xb8, exec: Instruction::cp,     fmt: "CP B"           },
    Instruction { opcode: 0xb9, exec: Instruction::cp,     fmt: "CP C"           },
    Instruction { opcode: 0xba, exec: Instruction::cp,     fmt: "CP D"           },
    Instruction { opcode: 0xbb, exec: Instruction::cp,     fmt: "CP E"           },
    Instruction { opcode: 0xbc, exec: Instruction::cp,     fmt: "CP H"           },
    Instruction { opcode: 0xbd, exec: Instruction::cp,     fmt: "CP L"           },
    Instruction { opcode: 0xbe, exec: Instruction::cp,     fmt: "CP (HL)"        },
    Instruction { opcode: 0xbf, exec: Instruction::cp,     fmt: "CP A"           },
    Instruction { opcode: 0xc0, exec: Instruction::ret,    fmt: "RET NZ"         },
    Instruction { opcode: 0xc1, exec: Instruction::pop,    fmt: "POP BC"         },
    Instruction { opcode: 0xc2, exec: Instruction::jp,     fmt: "JP NZ, a16"     },
    Instruction { opcode: 0xc3, exec: Instruction::jp,     fmt: "JP a16"         },
    Instruction { opcode: 0xc4, exec: Instruction::call,   fmt: "CALL NZ, a16"   },
    Instruction { opcode: 0xc5, exec: Instruction::push,   fmt: "PUSH BC"        },
    Instruction { opcode: 0xc6, exec: Instruction::add8,   fmt: "ADD A, d8"      },
    Instruction { opcode: 0xc7, exec: Instruction::rst,    fmt: "RST 00H"        },
    Instruction { opcode: 0xc8, exec: Instruction::ret,    fmt: "RET Z"          },
    Instruction { opcode: 0xc9, exec: Instruction::ret,    fmt: "RET"            },
    Instruction { opcode: 0xca, exec: Instruction::jp,     fmt: "JP Z, a16"      },
    Instruction { opcode: 0xcb, exec: Instruction::prefix, fmt: "PREFIX"         },
    Instruction { opcode: 0xcc, exec: Instruction::call,   fmt: "CALL Z, a16"    },
    Instruction { opcode: 0xcd, exec: Instruction::call,   fmt: "CALL a16"       },
    Instruction { opcode: 0xce, exec: Instruction::adc,    fmt: "ADC A, d8"      },
    Instruction { opcode: 0xcf, exec: Instruction::rst,    fmt: "RST 08H"        },
    Instruction { opcode: 0xd0, exec: Instruction::ret,    fmt: "RET NC"         },
    Instruction { opcode: 0xd1, exec: Instruction::pop,    fmt: "POP DE"         },
    Instruction { opcode: 0xd2, exec: Instruction::jp,     fmt: "JP NC, a16"     },
    Instruction { opcode: 0xd3, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xd4, exec: Instruction::call,   fmt: "CALL NC, a16"   },
    Instruction { opcode: 0xd5, exec: Instruction::push,   fmt: "PUSH DE"        },
    Instruction { opcode: 0xd6, exec: Instruction::sub,    fmt: "SUB d8"         },
    Instruction { opcode: 0xd7, exec: Instruction::rst,    fmt: "RST 10H"        },
    Instruction { opcode: 0xd8, exec: Instruction::ret,    fmt: "RET C"          },
    Instruction { opcode: 0xd9, exec: Instruction::reti,   fmt: "RETI"           },
    Instruction { opcode: 0xda, exec: Instruction::jp,     fmt: "JP C, a16"      },
    Instruction { opcode: 0xdb, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xdc, exec: Instruction::call,   fmt: "CALL C, a16"    },
    Instruction { opcode: 0xdd, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xde, exec: Instruction::sbc8,   fmt: "SBC A, d8"      },
    Instruction { opcode: 0xdf, exec: Instruction::rst,    fmt: "RST 18H"        },
    Instruction { opcode: 0xe0, exec: Instruction::ldh,    fmt: "LDH (a8), A"    },
    Instruction { opcode: 0xe1, exec: Instruction::pop,    fmt: "POP HL"         },
    Instruction { opcode: 0xe2, exec: Instruction::ldh,    fmt: "LDH (C), A"     },
    Instruction { opcode: 0xe3, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xe4, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xe5, exec: Instruction::push,   fmt: "PUSH HL"        },
    Instruction { opcode: 0xe6, exec: Instruction::and8,   fmt: "AND d8"         },
    Instruction { opcode: 0xe7, exec: Instruction::rst,    fmt: "RST 20H"        },
    Instruction { opcode: 0xe8, exec: Instruction::add16,  fmt: "ADD SP, r8"     },
    Instruction { opcode: 0xe9, exec: Instruction::jp,     fmt: "JP HL"          },
    Instruction { opcode: 0xea, exec: Instruction::ld8,    fmt: "LD (a16), A"    },
    Instruction { opcode: 0xeb, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xec, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xed, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xee, exec: Instruction::xor8,   fmt: "XOR d8"         },
    Instruction { opcode: 0xef, exec: Instruction::rst,    fmt: "RST 28H"        },
    Instruction { opcode: 0xf0, exec: Instruction::ldh,    fmt: "LDH A, (a8)"    },
    Instruction { opcode: 0xf1, exec: Instruction::pop,    fmt: "POP AF"         },
    Instruction { opcode: 0xf2, exec: Instruction::ldh,    fmt: "LD A, (C)"      },
    Instruction { opcode: 0xf3, exec: Instruction::di,     fmt: "DI"             },
    Instruction { opcode: 0xf4, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xf5, exec: Instruction::push,   fmt: "PUSH AF"        },
    Instruction { opcode: 0xf6, exec: Instruction::or8,    fmt: "OR d8"          },
    Instruction { opcode: 0xf7, exec: Instruction::rst,    fmt: "RST 30H"        },
    Instruction { opcode: 0xf8, exec: Instruction::ld16,   fmt: "LD HL, SP + r8" },
    Instruction { opcode: 0xf9, exec: Instruction::ld16,   fmt: "LD SP, HL"      },
    Instruction { opcode: 0xfa, exec: Instruction::ld8,    fmt: "LD A, (a16)"    },
    Instruction { opcode: 0xfb, exec: Instruction::ei,     fmt: "EI"             },
    Instruction { opcode: 0xfc, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xfd, exec: Instruction::unused, fmt: "UNUSED"         },
    Instruction { opcode: 0xfe, exec: Instruction::cp,     fmt: "CP d8"          },
    Instruction { opcode: 0xff, exec: Instruction::rst,    fmt: "RST 38H"        },
];

#[rustfmt::skip]
const PREFIX: [Instruction; 0x100] = [
    Instruction { opcode: 0x00, exec: Instruction::rlc,    fmt: "RLC B"          },
    Instruction { opcode: 0x01, exec: Instruction::rlc,    fmt: "RLC C"          },
    Instruction { opcode: 0x02, exec: Instruction::rlc,    fmt: "RLC D"          },
    Instruction { opcode: 0x03, exec: Instruction::rlc,    fmt: "RLC E"          },
    Instruction { opcode: 0x04, exec: Instruction::rlc,    fmt: "RLC H"          },
    Instruction { opcode: 0x05, exec: Instruction::rlc,    fmt: "RLC L"          },
    Instruction { opcode: 0x06, exec: Instruction::rlc,    fmt: "RLC (HL)"       },
    Instruction { opcode: 0x07, exec: Instruction::rlc,    fmt: "RLC A"          },
    Instruction { opcode: 0x08, exec: Instruction::rrc,    fmt: "RRC B"          },
    Instruction { opcode: 0x09, exec: Instruction::rrc,    fmt: "RRC C"          },
    Instruction { opcode: 0x0a, exec: Instruction::rrc,    fmt: "RRC D"          },
    Instruction { opcode: 0x0b, exec: Instruction::rrc,    fmt: "RRC E"          },
    Instruction { opcode: 0x0c, exec: Instruction::rrc,    fmt: "RRC H"          },
    Instruction { opcode: 0x0d, exec: Instruction::rrc,    fmt: "RRC L"          },
    Instruction { opcode: 0x0e, exec: Instruction::rrc,    fmt: "RRC (HL)"       },
    Instruction { opcode: 0x0f, exec: Instruction::rrc,    fmt: "RRC A"          },
    Instruction { opcode: 0x10, exec: Instruction::rl,     fmt: "RL B"           },
    Instruction { opcode: 0x11, exec: Instruction::rl,     fmt: "RL C"           },
    Instruction { opcode: 0x12, exec: Instruction::rl,     fmt: "RL D"           },
    Instruction { opcode: 0x13, exec: Instruction::rl,     fmt: "RL E"           },
    Instruction { opcode: 0x14, exec: Instruction::rl,     fmt: "RL H"           },
    Instruction { opcode: 0x15, exec: Instruction::rl,     fmt: "RL L"           },
    Instruction { opcode: 0x16, exec: Instruction::rl,     fmt: "RL (HL)"        },
    Instruction { opcode: 0x17, exec: Instruction::rl,     fmt: "RL A"           },
    Instruction { opcode: 0x18, exec: Instruction::rr,     fmt: "RR B"           },
    Instruction { opcode: 0x19, exec: Instruction::rr,     fmt: "RR C"           },
    Instruction { opcode: 0x1a, exec: Instruction::rr,     fmt: "RR D"           },
    Instruction { opcode: 0x1b, exec: Instruction::rr,     fmt: "RR E"           },
    Instruction { opcode: 0x1c, exec: Instruction::rr,     fmt: "RR H"           },
    Instruction { opcode: 0x1d, exec: Instruction::rr,     fmt: "RR L"           },
    Instruction { opcode: 0x1e, exec: Instruction::rr,     fmt: "RR (HL)"        },
    Instruction { opcode: 0x1f, exec: Instruction::rr,     fmt: "RR A"           },
    Instruction { opcode: 0x20, exec: Instruction::sla,    fmt: "SLA B"          },
    Instruction { opcode: 0x21, exec: Instruction::sla,    fmt: "SLA C"          },
    Instruction { opcode: 0x22, exec: Instruction::sla,    fmt: "SLA D"          },
    Instruction { opcode: 0x23, exec: Instruction::sla,    fmt: "SLA E"          },
    Instruction { opcode: 0x24, exec: Instruction::sla,    fmt: "SLA H"          },
    Instruction { opcode: 0x25, exec: Instruction::sla,    fmt: "SLA L"          },
    Instruction { opcode: 0x26, exec: Instruction::sla,    fmt: "SLA (HL)"       },
    Instruction { opcode: 0x27, exec: Instruction::sla,    fmt: "SLA A"          },
    Instruction { opcode: 0x28, exec: Instruction::sra,    fmt: "SRA B"          },
    Instruction { opcode: 0x29, exec: Instruction::sra,    fmt: "SRA C"          },
    Instruction { opcode: 0x2a, exec: Instruction::sra,    fmt: "SRA D"          },
    Instruction { opcode: 0x2b, exec: Instruction::sra,    fmt: "SRA E"          },
    Instruction { opcode: 0x2c, exec: Instruction::sra,    fmt: "SRA H"          },
    Instruction { opcode: 0x2d, exec: Instruction::sra,    fmt: "SRA L"          },
    Instruction { opcode: 0x2e, exec: Instruction::sra,    fmt: "SRA (HL)"       },
    Instruction { opcode: 0x2f, exec: Instruction::sra,    fmt: "SRA A"          },
    Instruction { opcode: 0x30, exec: Instruction::swap,   fmt: "SWAP B"         },
    Instruction { opcode: 0x31, exec: Instruction::swap,   fmt: "SWAP C"         },
    Instruction { opcode: 0x32, exec: Instruction::swap,   fmt: "SWAP D"         },
    Instruction { opcode: 0x33, exec: Instruction::swap,   fmt: "SWAP E"         },
    Instruction { opcode: 0x34, exec: Instruction::swap,   fmt: "SWAP H"         },
    Instruction { opcode: 0x35, exec: Instruction::swap,   fmt: "SWAP L"         },
    Instruction { opcode: 0x36, exec: Instruction::swap,   fmt: "SWAP (HL)"      },
    Instruction { opcode: 0x37, exec: Instruction::swap,   fmt: "SWAP A"         },
    Instruction { opcode: 0x38, exec: Instruction::srl,    fmt: "SRL B"          },
    Instruction { opcode: 0x39, exec: Instruction::srl,    fmt: "SRL C"          },
    Instruction { opcode: 0x3a, exec: Instruction::srl,    fmt: "SRL D"          },
    Instruction { opcode: 0x3b, exec: Instruction::srl,    fmt: "SRL E"          },
    Instruction { opcode: 0x3c, exec: Instruction::srl,    fmt: "SRL H"          },
    Instruction { opcode: 0x3d, exec: Instruction::srl,    fmt: "SRL L"          },
    Instruction { opcode: 0x3e, exec: Instruction::srl,    fmt: "SRL (HL)"       },
    Instruction { opcode: 0x3f, exec: Instruction::srl,    fmt: "SRL A"          },
    Instruction { opcode: 0x40, exec: Instruction::bit,    fmt: "BIT 0, B"       },
    Instruction { opcode: 0x41, exec: Instruction::bit,    fmt: "BIT 0, C"       },
    Instruction { opcode: 0x42, exec: Instruction::bit,    fmt: "BIT 0, D"       },
    Instruction { opcode: 0x43, exec: Instruction::bit,    fmt: "BIT 0, E"       },
    Instruction { opcode: 0x44, exec: Instruction::bit,    fmt: "BIT 0, H"       },
    Instruction { opcode: 0x45, exec: Instruction::bit,    fmt: "BIT 0, L"       },
    Instruction { opcode: 0x46, exec: Instruction::bit,    fmt: "BIT 0, (HL)"    },
    Instruction { opcode: 0x47, exec: Instruction::bit,    fmt: "BIT 0, A"       },
    Instruction { opcode: 0x48, exec: Instruction::bit,    fmt: "BIT 1, B"       },
    Instruction { opcode: 0x49, exec: Instruction::bit,    fmt: "BIT 1, C"       },
    Instruction { opcode: 0x4a, exec: Instruction::bit,    fmt: "BIT 1, D"       },
    Instruction { opcode: 0x4b, exec: Instruction::bit,    fmt: "BIT 1, E"       },
    Instruction { opcode: 0x4c, exec: Instruction::bit,    fmt: "BIT 1, H"       },
    Instruction { opcode: 0x4d, exec: Instruction::bit,    fmt: "BIT 1, L"       },
    Instruction { opcode: 0x4e, exec: Instruction::bit,    fmt: "BIT 1, (HL)"    },
    Instruction { opcode: 0x4f, exec: Instruction::bit,    fmt: "BIT 1, A"       },
    Instruction { opcode: 0x50, exec: Instruction::bit,    fmt: "BIT 2, B"       },
    Instruction { opcode: 0x51, exec: Instruction::bit,    fmt: "BIT 2, C"       },
    Instruction { opcode: 0x52, exec: Instruction::bit,    fmt: "BIT 2, D"       },
    Instruction { opcode: 0x53, exec: Instruction::bit,    fmt: "BIT 2, E"       },
    Instruction { opcode: 0x54, exec: Instruction::bit,    fmt: "BIT 2, H"       },
    Instruction { opcode: 0x55, exec: Instruction::bit,    fmt: "BIT 2, L"       },
    Instruction { opcode: 0x56, exec: Instruction::bit,    fmt: "BIT 2, (HL)"    },
    Instruction { opcode: 0x57, exec: Instruction::bit,    fmt: "BIT 2, A"       },
    Instruction { opcode: 0x58, exec: Instruction::bit,    fmt: "BIT 3, B"       },
    Instruction { opcode: 0x59, exec: Instruction::bit,    fmt: "BIT 3, C"       },
    Instruction { opcode: 0x5a, exec: Instruction::bit,    fmt: "BIT 3, D"       },
    Instruction { opcode: 0x5b, exec: Instruction::bit,    fmt: "BIT 3, E"       },
    Instruction { opcode: 0x5c, exec: Instruction::bit,    fmt: "BIT 3, H"       },
    Instruction { opcode: 0x5d, exec: Instruction::bit,    fmt: "BIT 3, L"       },
    Instruction { opcode: 0x5e, exec: Instruction::bit,    fmt: "BIT 3, (HL)"    },
    Instruction { opcode: 0x5f, exec: Instruction::bit,    fmt: "BIT 3, A"       },
    Instruction { opcode: 0x60, exec: Instruction::bit,    fmt: "BIT 4, B"       },
    Instruction { opcode: 0x61, exec: Instruction::bit,    fmt: "BIT 4, C"       },
    Instruction { opcode: 0x62, exec: Instruction::bit,    fmt: "BIT 4, D"       },
    Instruction { opcode: 0x63, exec: Instruction::bit,    fmt: "BIT 4, E"       },
    Instruction { opcode: 0x64, exec: Instruction::bit,    fmt: "BIT 4, H"       },
    Instruction { opcode: 0x65, exec: Instruction::bit,    fmt: "BIT 4, L"       },
    Instruction { opcode: 0x66, exec: Instruction::bit,    fmt: "BIT 4, (HL)"    },
    Instruction { opcode: 0x67, exec: Instruction::bit,    fmt: "BIT 4, A"       },
    Instruction { opcode: 0x68, exec: Instruction::bit,    fmt: "BIT 5, B"       },
    Instruction { opcode: 0x69, exec: Instruction::bit,    fmt: "BIT 5, C"       },
    Instruction { opcode: 0x6a, exec: Instruction::bit,    fmt: "BIT 5, D"       },
    Instruction { opcode: 0x6b, exec: Instruction::bit,    fmt: "BIT 5, E"       },
    Instruction { opcode: 0x6c, exec: Instruction::bit,    fmt: "BIT 5, H"       },
    Instruction { opcode: 0x6d, exec: Instruction::bit,    fmt: "BIT 5, L"       },
    Instruction { opcode: 0x6e, exec: Instruction::bit,    fmt: "BIT 5, (HL)"    },
    Instruction { opcode: 0x6f, exec: Instruction::bit,    fmt: "BIT 5, A"       },
    Instruction { opcode: 0x70, exec: Instruction::bit,    fmt: "BIT 6, B"       },
    Instruction { opcode: 0x71, exec: Instruction::bit,    fmt: "BIT 6, C"       },
    Instruction { opcode: 0x72, exec: Instruction::bit,    fmt: "BIT 6, D"       },
    Instruction { opcode: 0x73, exec: Instruction::bit,    fmt: "BIT 6, E"       },
    Instruction { opcode: 0x74, exec: Instruction::bit,    fmt: "BIT 6, H"       },
    Instruction { opcode: 0x75, exec: Instruction::bit,    fmt: "BIT 6, L"       },
    Instruction { opcode: 0x76, exec: Instruction::bit,    fmt: "BIT 6, (HL)"    },
    Instruction { opcode: 0x77, exec: Instruction::bit,    fmt: "BIT 6, A"       },
    Instruction { opcode: 0x78, exec: Instruction::bit,    fmt: "BIT 7, B"       },
    Instruction { opcode: 0x79, exec: Instruction::bit,    fmt: "BIT 7, C"       },
    Instruction { opcode: 0x7a, exec: Instruction::bit,    fmt: "BIT 7, D"       },
    Instruction { opcode: 0x7b, exec: Instruction::bit,    fmt: "BIT 7, E"       },
    Instruction { opcode: 0x7c, exec: Instruction::bit,    fmt: "BIT 7, H"       },
    Instruction { opcode: 0x7d, exec: Instruction::bit,    fmt: "BIT 7, L"       },
    Instruction { opcode: 0x7e, exec: Instruction::bit,    fmt: "BIT 7, (HL)"    },
    Instruction { opcode: 0x7f, exec: Instruction::bit,    fmt: "BIT 7, A"       },
    Instruction { opcode: 0x80, exec: Instruction::res,    fmt: "RES 0, B"       },
    Instruction { opcode: 0x81, exec: Instruction::res,    fmt: "RES 0, C"       },
    Instruction { opcode: 0x82, exec: Instruction::res,    fmt: "RES 0, D"       },
    Instruction { opcode: 0x83, exec: Instruction::res,    fmt: "RES 0, E"       },
    Instruction { opcode: 0x84, exec: Instruction::res,    fmt: "RES 0, H"       },
    Instruction { opcode: 0x85, exec: Instruction::res,    fmt: "RES 0, L"       },
    Instruction { opcode: 0x86, exec: Instruction::res,    fmt: "RES 0, (HL)"    },
    Instruction { opcode: 0x87, exec: Instruction::res,    fmt: "RES 0, A"       },
    Instruction { opcode: 0x88, exec: Instruction::res,    fmt: "RES 1, B"       },
    Instruction { opcode: 0x89, exec: Instruction::res,    fmt: "RES 1, C"       },
    Instruction { opcode: 0x8a, exec: Instruction::res,    fmt: "RES 1, D"       },
    Instruction { opcode: 0x8b, exec: Instruction::res,    fmt: "RES 1, E"       },
    Instruction { opcode: 0x8c, exec: Instruction::res,    fmt: "RES 1, H"       },
    Instruction { opcode: 0x8d, exec: Instruction::res,    fmt: "RES 1, L"       },
    Instruction { opcode: 0x8e, exec: Instruction::res,    fmt: "RES 1, (HL)"    },
    Instruction { opcode: 0x8f, exec: Instruction::res,    fmt: "RES 1, A"       },
    Instruction { opcode: 0x90, exec: Instruction::res,    fmt: "RES 2, B"       },
    Instruction { opcode: 0x91, exec: Instruction::res,    fmt: "RES 2, C"       },
    Instruction { opcode: 0x92, exec: Instruction::res,    fmt: "RES 2, D"       },
    Instruction { opcode: 0x93, exec: Instruction::res,    fmt: "RES 2, E"       },
    Instruction { opcode: 0x94, exec: Instruction::res,    fmt: "RES 2, H"       },
    Instruction { opcode: 0x95, exec: Instruction::res,    fmt: "RES 2, L"       },
    Instruction { opcode: 0x96, exec: Instruction::res,    fmt: "RES 2, (HL)"    },
    Instruction { opcode: 0x97, exec: Instruction::res,    fmt: "RES 2, A"       },
    Instruction { opcode: 0x98, exec: Instruction::res,    fmt: "RES 3, B"       },
    Instruction { opcode: 0x99, exec: Instruction::res,    fmt: "RES 3, C"       },
    Instruction { opcode: 0x9a, exec: Instruction::res,    fmt: "RES 3, D"       },
    Instruction { opcode: 0x9b, exec: Instruction::res,    fmt: "RES 3, E"       },
    Instruction { opcode: 0x9c, exec: Instruction::res,    fmt: "RES 3, H"       },
    Instruction { opcode: 0x9d, exec: Instruction::res,    fmt: "RES 3, L"       },
    Instruction { opcode: 0x9e, exec: Instruction::res,    fmt: "RES 3, (HL)"    },
    Instruction { opcode: 0x9f, exec: Instruction::res,    fmt: "RES 3, A"       },
    Instruction { opcode: 0xa0, exec: Instruction::res,    fmt: "RES 4, B"       },
    Instruction { opcode: 0xa1, exec: Instruction::res,    fmt: "RES 4, C"       },
    Instruction { opcode: 0xa2, exec: Instruction::res,    fmt: "RES 4, D"       },
    Instruction { opcode: 0xa3, exec: Instruction::res,    fmt: "RES 4, E"       },
    Instruction { opcode: 0xa4, exec: Instruction::res,    fmt: "RES 4, H"       },
    Instruction { opcode: 0xa5, exec: Instruction::res,    fmt: "RES 4, L"       },
    Instruction { opcode: 0xa6, exec: Instruction::res,    fmt: "RES 4, (HL)"    },
    Instruction { opcode: 0xa7, exec: Instruction::res,    fmt: "RES 4, A"       },
    Instruction { opcode: 0xa8, exec: Instruction::res,    fmt: "RES 5, B"       },
    Instruction { opcode: 0xa9, exec: Instruction::res,    fmt: "RES 5, C"       },
    Instruction { opcode: 0xaa, exec: Instruction::res,    fmt: "RES 5, D"       },
    Instruction { opcode: 0xab, exec: Instruction::res,    fmt: "RES 5, E"       },
    Instruction { opcode: 0xac, exec: Instruction::res,    fmt: "RES 5, H"       },
    Instruction { opcode: 0xad, exec: Instruction::res,    fmt: "RES 5, L"       },
    Instruction { opcode: 0xae, exec: Instruction::res,    fmt: "RES 5, (HL)"    },
    Instruction { opcode: 0xaf, exec: Instruction::res,    fmt: "RES 5, A"       },
    Instruction { opcode: 0xb0, exec: Instruction::res,    fmt: "RES 6, B"       },
    Instruction { opcode: 0xb1, exec: Instruction::res,    fmt: "RES 6, C"       },
    Instruction { opcode: 0xb2, exec: Instruction::res,    fmt: "RES 6, D"       },
    Instruction { opcode: 0xb3, exec: Instruction::res,    fmt: "RES 6, E"       },
    Instruction { opcode: 0xb4, exec: Instruction::res,    fmt: "RES 6, H"       },
    Instruction { opcode: 0xb5, exec: Instruction::res,    fmt: "RES 6, L"       },
    Instruction { opcode: 0xb6, exec: Instruction::res,    fmt: "RES 6, (HL)"    },
    Instruction { opcode: 0xb7, exec: Instruction::res,    fmt: "RES 6, A"       },
    Instruction { opcode: 0xb8, exec: Instruction::res,    fmt: "RES 7, B"       },
    Instruction { opcode: 0xb9, exec: Instruction::res,    fmt: "RES 7, C"       },
    Instruction { opcode: 0xba, exec: Instruction::res,    fmt: "RES 7, D"       },
    Instruction { opcode: 0xbb, exec: Instruction::res,    fmt: "RES 7, E"       },
    Instruction { opcode: 0xbc, exec: Instruction::res,    fmt: "RES 7, H"       },
    Instruction { opcode: 0xbd, exec: Instruction::res,    fmt: "RES 7, L"       },
    Instruction { opcode: 0xbe, exec: Instruction::res,    fmt: "RES 7, (HL)"    },
    Instruction { opcode: 0xbf, exec: Instruction::res,    fmt: "RES 7, A"       },
    Instruction { opcode: 0xc0, exec: Instruction::set,    fmt: "SET 0, B"       },
    Instruction { opcode: 0xc1, exec: Instruction::set,    fmt: "SET 0, C"       },
    Instruction { opcode: 0xc2, exec: Instruction::set,    fmt: "SET 0, D"       },
    Instruction { opcode: 0xc3, exec: Instruction::set,    fmt: "SET 0, E"       },
    Instruction { opcode: 0xc4, exec: Instruction::set,    fmt: "SET 0, H"       },
    Instruction { opcode: 0xc5, exec: Instruction::set,    fmt: "SET 0, L"       },
    Instruction { opcode: 0xc6, exec: Instruction::set,    fmt: "SET 0, (HL)"    },
    Instruction { opcode: 0xc7, exec: Instruction::set,    fmt: "SET 0, A"       },
    Instruction { opcode: 0xc8, exec: Instruction::set,    fmt: "SET 1, B"       },
    Instruction { opcode: 0xc9, exec: Instruction::set,    fmt: "SET 1, C"       },
    Instruction { opcode: 0xca, exec: Instruction::set,    fmt: "SET 1, D"       },
    Instruction { opcode: 0xcb, exec: Instruction::set,    fmt: "SET 1, E"       },
    Instruction { opcode: 0xcc, exec: Instruction::set,    fmt: "SET 1, H"       },
    Instruction { opcode: 0xcd, exec: Instruction::set,    fmt: "SET 1, L"       },
    Instruction { opcode: 0xce, exec: Instruction::set,    fmt: "SET 1, (HL)"    },
    Instruction { opcode: 0xcf, exec: Instruction::set,    fmt: "SET 1, A"       },
    Instruction { opcode: 0xd0, exec: Instruction::set,    fmt: "SET 2, B"       },
    Instruction { opcode: 0xd1, exec: Instruction::set,    fmt: "SET 2, C"       },
    Instruction { opcode: 0xd2, exec: Instruction::set,    fmt: "SET 2, D"       },
    Instruction { opcode: 0xd3, exec: Instruction::set,    fmt: "SET 2, E"       },
    Instruction { opcode: 0xd4, exec: Instruction::set,    fmt: "SET 2, H"       },
    Instruction { opcode: 0xd5, exec: Instruction::set,    fmt: "SET 2, L"       },
    Instruction { opcode: 0xd6, exec: Instruction::set,    fmt: "SET 2, (HL)"    },
    Instruction { opcode: 0xd7, exec: Instruction::set,    fmt: "SET 2, A"       },
    Instruction { opcode: 0xd8, exec: Instruction::set,    fmt: "SET 3, B"       },
    Instruction { opcode: 0xd9, exec: Instruction::set,    fmt: "SET 3, C"       },
    Instruction { opcode: 0xda, exec: Instruction::set,    fmt: "SET 3, D"       },
    Instruction { opcode: 0xdb, exec: Instruction::set,    fmt: "SET 3, E"       },
    Instruction { opcode: 0xdc, exec: Instruction::set,    fmt: "SET 3, H"       },
    Instruction { opcode: 0xdd, exec: Instruction::set,    fmt: "SET 3, L"       },
    Instruction { opcode: 0xde, exec: Instruction::set,    fmt: "SET 3, (HL)"    },
    Instruction { opcode: 0xdf, exec: Instruction::set,    fmt: "SET 3, A"       },
    Instruction { opcode: 0xe0, exec: Instruction::set,    fmt: "SET 4, B"       },
    Instruction { opcode: 0xe1, exec: Instruction::set,    fmt: "SET 4, C"       },
    Instruction { opcode: 0xe2, exec: Instruction::set,    fmt: "SET 4, D"       },
    Instruction { opcode: 0xe3, exec: Instruction::set,    fmt: "SET 4, E"       },
    Instruction { opcode: 0xe4, exec: Instruction::set,    fmt: "SET 4, H"       },
    Instruction { opcode: 0xe5, exec: Instruction::set,    fmt: "SET 4, L"       },
    Instruction { opcode: 0xe6, exec: Instruction::set,    fmt: "SET 4, (HL)"    },
    Instruction { opcode: 0xe7, exec: Instruction::set,    fmt: "SET 4, A"       },
    Instruction { opcode: 0xe8, exec: Instruction::set,    fmt: "SET 5, B"       },
    Instruction { opcode: 0xe9, exec: Instruction::set,    fmt: "SET 5, C"       },
    Instruction { opcode: 0xea, exec: Instruction::set,    fmt: "SET 5, D"       },
    Instruction { opcode: 0xeb, exec: Instruction::set,    fmt: "SET 5, E"       },
    Instruction { opcode: 0xec, exec: Instruction::set,    fmt: "SET 5, H"       },
    Instruction { opcode: 0xed, exec: Instruction::set,    fmt: "SET 5, L"       },
    Instruction { opcode: 0xee, exec: Instruction::set,    fmt: "SET 5, (HL)"    },
    Instruction { opcode: 0xef, exec: Instruction::set,    fmt: "SET 5, A"       },
    Instruction { opcode: 0xf0, exec: Instruction::set,    fmt: "SET 6, B"       },
    Instruction { opcode: 0xf1, exec: Instruction::set,    fmt: "SET 6, C"       },
    Instruction { opcode: 0xf2, exec: Instruction::set,    fmt: "SET 6, D"       },
    Instruction { opcode: 0xf3, exec: Instruction::set,    fmt: "SET 6, E"       },
    Instruction { opcode: 0xf4, exec: Instruction::set,    fmt: "SET 6, H"       },
    Instruction { opcode: 0xf5, exec: Instruction::set,    fmt: "SET 6, L"       },
    Instruction { opcode: 0xf6, exec: Instruction::set,    fmt: "SET 6, (HL)"    },
    Instruction { opcode: 0xf7, exec: Instruction::set,    fmt: "SET 6, A"       },
    Instruction { opcode: 0xf8, exec: Instruction::set,    fmt: "SET 7, B"       },
    Instruction { opcode: 0xf9, exec: Instruction::set,    fmt: "SET 7, C"       },
    Instruction { opcode: 0xfa, exec: Instruction::set,    fmt: "SET 7, D"       },
    Instruction { opcode: 0xfb, exec: Instruction::set,    fmt: "SET 7, E"       },
    Instruction { opcode: 0xfc, exec: Instruction::set,    fmt: "SET 7, H"       },
    Instruction { opcode: 0xfd, exec: Instruction::set,    fmt: "SET 7, L"       },
    Instruction { opcode: 0xfe, exec: Instruction::set,    fmt: "SET 7, (HL)"    },
    Instruction { opcode: 0xff, exec: Instruction::set,    fmt: "SET 7, A"       },
];
