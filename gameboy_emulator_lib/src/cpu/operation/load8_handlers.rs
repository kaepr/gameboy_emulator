use crate::{
    cpu::{registers::Reg16, CPU},
    utils::le_bytes_to_word,
};

use super::opcodes::{Load8Dest, Load8Src};

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
                $cpu.read_byte_bus(addr)
            }
            Load8Src::BC => {
                let addr = $cpu.registers.get_reg_pair(Reg16::BC);
                $cpu.read_byte_bus(addr)
            }
            Load8Src::DE => {
                let addr = $cpu.registers.get_reg_pair(Reg16::DE);
                $cpu.read_byte_bus(addr)
            }
            Load8Src::HLI => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                let value = $cpu.read_byte_bus(addr);
                $cpu.registers.set_reg_pair(addr + 1, Reg16::HL);
                value
            }
            Load8Src::HLD => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                let value = $cpu.read_byte_bus(addr);
                $cpu.registers.set_reg_pair(addr - 1, Reg16::HL);
                value
            }
            Load8Src::Direct8Bit => $cpu.fetch_byte(),
            Load8Src::AddrC => {
                let addr = 0xFF00 | ($cpu.registers.c as u16);
                $cpu.read_byte_bus(addr)
            }
            Load8Src::Addr16Bit => {
                let lo = $cpu.fetch_byte();
                let hi = $cpu.fetch_byte();
                let addr = le_bytes_to_word(lo, hi);
                $cpu.read_byte_bus(addr)
            }
            _ => panic!("invalid enum variants"),
        }
    };
}

pub fn ld(cpu: &mut CPU, dest: Load8Dest, src: Load8Src) {
    match dest {
        Load8Dest::AddrC => {
            let byte = reg_src!(cpu, src);
            let addr = 0xFF00 | (cpu.registers.c as u16);
            cpu.write_byte(addr, byte);
        }
        Load8Dest::Addr16Bit => {
            let value = reg_src!(cpu, src);
            let lo = cpu.fetch_byte();
            let hi = cpu.fetch_byte();
            let addr = le_bytes_to_word(lo, hi);
            cpu.write_byte(addr, value);
        }
        Load8Dest::A => {
            cpu.registers.a = reg_src!(cpu, src);
        }
        Load8Dest::HLI => {
            let byte = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::HL);
            cpu.write_byte(addr, byte);
            cpu.registers.set_reg_pair(addr + 1, Reg16::HL);
        }
        Load8Dest::HLD => {
            let byte = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::HL);
            cpu.write_byte(addr, byte);
            cpu.registers.set_reg_pair(addr - 1, Reg16::HL);
        }
        Load8Dest::BC => {
            let byte = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::BC);
            cpu.write_byte(addr, byte);
        }
        Load8Dest::DE => {
            let byte = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::DE);
            cpu.write_byte(addr, byte);
        }
        Load8Dest::HL => {
            let byte = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::HL);
            cpu.write_byte(addr, byte)
        }
        Load8Dest::B => {
            cpu.registers.b = reg_src!(cpu, src);
        }
        Load8Dest::C => {
            cpu.registers.c = reg_src!(cpu, src);
        }
        Load8Dest::D => {
            cpu.registers.d = reg_src!(cpu, src);
        }
        Load8Dest::E => {
            cpu.registers.e = reg_src!(cpu, src);
        }
        Load8Dest::H => {
            cpu.registers.h = reg_src!(cpu, src);
        }
        Load8Dest::L => {
            cpu.registers.l = reg_src!(cpu, src);
        }
        _ => panic!("Invalid enum variants passed"),
    };
}

pub fn ldh(cpu: &mut CPU, dest: Load8Dest, _src: Load8Src) {
    match dest {
        Load8Dest::Unsigned8 => {
            let d8 = cpu.fetch_byte();
            let addr = 0xFF00 | (d8 as u16);
            cpu.write_byte(addr, cpu.registers.a);
        }
        Load8Dest::A => {
            let d8 = cpu.fetch_byte();
            let addr = 0xFF00 | (d8 as u16);
            let byte = cpu.read_byte_bus(addr);
            cpu.registers.a = byte;
        }
        _ => panic!("Invalid enum variant"),
    };
}
