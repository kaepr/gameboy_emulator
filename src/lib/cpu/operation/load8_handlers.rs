use crate::{
    cpu::{
        registers::{Reg16, Reg8},
        InstructionReturn, ReturnType, CPU,
    },
    utils::le_bytes_to_word,
};

use super::opcodes::{Load8Dest, Load8Src};

macro_rules! reg_dest {
    ($x: expr, $reg_type: ident) => {
        match $reg_type {
            Load8Dest::B => $x.b,
            Load8Dest::C => $x.b,
            Load8Dest::D => $x.b,
            Load8Dest::E => $x.b,
            Load8Dest::H => $x.b,
            Load8Dest::L => $x.b,
            _ => panic!("invalid enum variant"),
        }
    };
}

macro_rules! reg_src {
    ($cpu: ident, $reg_type: ident) => {
        match $reg_type {
            Load8Src::A => $cpu.registers.a,
            Load8Src::B => $cpu.registers.b,
            Load8Src::C => $cpu.registers.c,
            Load8Src::D => $cpu.registers.d,
            Load8Src::E => $cpu.registers.e,
            Load8Src::H => $cpu.registers.h,
            Load8Src::L => $cpu.registers.l,
            Load8Src::HL => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                let value = $cpu.bus.read(addr);
                value
            }
            Load8Src::BC => {
                let addr = $cpu.registers.get_reg_pair(Reg16::BC);
                let value = $cpu.bus.read(addr);
                value
            }
            Load8Src::DE => {
                let addr = $cpu.registers.get_reg_pair(Reg16::DE);
                let value = $cpu.bus.read(addr);
                value
            }
            Load8Src::HLI => {
                let mut addr = $cpu.registers.get_reg_pair(Reg16::HL);
                let value = $cpu.bus.read(addr);
                addr += 1;
                $cpu.registers.set_reg_pair(addr, Reg16::HL);
                value
            }
            Load8Src::HLD => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                let value = $cpu.bus.read(addr);
                addr -= 1;
                $cpu.registers.set_reg_pair(addr, Reg16::HL);
                value
            }
            Load8Src::Direct8Bit => {
                let value = $cpu.bus.read($cpu.registers.pc + 1);
                value
            }
            Load8Src::AddrC => 5,
            Load8Src::Addr16Bit => {
                let lo = $cpu.bus.read($cpu.registers.pc + 1);
                let hi = $cpu.bus.read($cpu.registers.pc + 2);
                let addr = le_bytes_to_word(lo, hi);
                let value = $cpu.bus.read(addr);
                value
            }
            _ => panic!("invalid enum variants"),
        }
    };
}

pub fn ld(cpu: &mut CPU, dest: Load8Dest, src: Load8Src) -> InstructionReturn {
    match dest {
        Load8Dest::AddrC => todo!(),
        Load8Dest::Addr16Bit => todo!(),
        Load8Dest::BC => todo!(),
        Load8Dest::A => todo!(),
        Load8Dest::HLI => todo!(),
        Load8Dest::HLD => todo!(),
        Load8Dest::DE => todo!(),
        Load8Dest::HL => todo!(),
        Load8Dest::B => todo!(),
        Load8Dest::C => todo!(),
        Load8Dest::D => todo!(),
        Load8Dest::E => todo!(),
        Load8Dest::H => todo!(),
        Load8Dest::L => {
            reg_dest!(cpu.registers, dest) = 5;
            // cpu.registers.l = reg_src!(cpu, src);
        }
        _ => panic!("Invalid enum variants passed"),
    };

    InstructionReturn {
        n_cycles: 0,
        n_bytes: 0,
        return_type: ReturnType::NotJumped,
    }
}

pub fn ldh(cpu: &mut CPU, dest: Load8Dest, src: Load8Src) -> InstructionReturn {
    match dest {
        Load8Dest::Unsigned8 => {
            let operand = cpu.bus.read(cpu.registers.pc + 1);
            let addr = 0xFF00 | (operand as u16);
            cpu.bus.write(addr, cpu.registers.a);
        }
        Load8Dest::A => {
            let operand = cpu.bus.read(cpu.registers.pc + 1);
            let addr = 0xFF00 | (operand as u16);
            let value = cpu.bus.read(addr);
            cpu.registers.set_reg(value, Reg8::A);
        }
        _ => panic!("Invalid enum variant"),
    };

    InstructionReturn {
        n_cycles: 12,
        n_bytes: 2,
        return_type: ReturnType::NotJumped,
    }
}
