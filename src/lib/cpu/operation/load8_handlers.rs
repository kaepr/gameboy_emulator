use crate::{
    cpu::{
        registers::{Reg16, Reg8},
        InstructionReturn, ReturnType, CPU,
    },
    utils::le_bytes_to_word,
};

use super::opcodes::{Load8Dest, Load8Src};

macro_rules! _reg_dest {
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
                let mut addr = $cpu.registers.get_reg_pair(Reg16::HL);
                let value = $cpu.bus.read(addr);
                addr -= 1;
                $cpu.registers.set_reg_pair(addr, Reg16::HL);
                value
            }
            Load8Src::Direct8Bit => {
                let value = $cpu.bus.read($cpu.registers.pc + 1);
                value
            }
            Load8Src::AddrC => {
                let val_at_c = $cpu.registers.c;
                let addr = 0xFF00 | (val_at_c as u16);
                let value = $cpu.bus.read(addr);
                value
            }
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

fn common_reg_return(src: Load8Src) -> (u16, u64) {
    if src == Load8Src::Direct8Bit {
        (2, 8)
    } else if src == Load8Src::HL {
        (1, 8)
    } else {
        (1, 4)
    }
}

fn hl_return(src: Load8Src) -> (u16, u64) {
    if src == Load8Src::Direct8Bit {
        (2, 12)
    } else {
        (1, 8)
    }
}

fn a_return(src: Load8Src) -> (u16, u64) {
    match src {
        Load8Src::Addr16Bit => (3, 16),
        Load8Src::Direct8Bit => (2, 8),
        Load8Src::AddrC
        | Load8Src::BC
        | Load8Src::DE
        | Load8Src::HLI
        | Load8Src::HLD
        | Load8Src::HL => (1, 8),
        Load8Src::A
        | Load8Src::B
        | Load8Src::C
        | Load8Src::D
        | Load8Src::E
        | Load8Src::H
        | Load8Src::L => (1, 4),
        _ => panic!("invalid enum choice"),
    }
}

pub fn ld(cpu: &mut CPU, dest: Load8Dest, src: Load8Src) -> InstructionReturn {
    let (n_bytes, n_cycles) = match dest {
        Load8Dest::AddrC => {
            let value = reg_src!(cpu, src);
            let addr = 0xFF00 | (cpu.registers.c as u16);
            cpu.bus.write(addr, value);
            (1, 8)
        }
        Load8Dest::Addr16Bit => {
            let value = reg_src!(cpu, src);
            let lo = cpu.bus.read(cpu.registers.pc + 1);
            let hi = cpu.bus.read(cpu.registers.pc + 2);
            let addr = le_bytes_to_word(lo, hi);
            cpu.bus.write(addr, value);
            (3, 16)
        }
        Load8Dest::A => {
            let value = reg_src!(cpu, src);
            cpu.registers.a = value;
            a_return(src)
        }
        Load8Dest::HLI => {
            let value = reg_src!(cpu, src);
            let mut addr = cpu.registers.get_reg_pair(Reg16::HL);
            cpu.bus.write(addr, value);
            addr += 1;
            cpu.registers.set_reg_pair(addr, Reg16::HL);
            (1, 8)
        }
        Load8Dest::HLD => {
            let value = reg_src!(cpu, src);
            let mut addr = cpu.registers.get_reg_pair(Reg16::HL);
            cpu.bus.write(addr, value);
            addr -= 1;
            cpu.registers.set_reg_pair(addr, Reg16::HL);
            (1, 8)
        }
        Load8Dest::BC => {
            let value = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::BC);
            cpu.bus.write(addr, value);
            (1, 8)
        }
        Load8Dest::DE => {
            let value = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::DE);
            cpu.bus.write(addr, value);
            (1, 8)
        }
        Load8Dest::HL => {
            let value = reg_src!(cpu, src);
            let addr = cpu.registers.get_reg_pair(Reg16::HL);
            cpu.bus.write(addr, value);
            hl_return(src)
        }
        Load8Dest::B => {
            cpu.registers.b = reg_src!(cpu, src);
            common_reg_return(src)
        }
        Load8Dest::C => {
            cpu.registers.c = reg_src!(cpu, src);
            common_reg_return(src)
        }
        Load8Dest::D => {
            cpu.registers.d = reg_src!(cpu, src);
            common_reg_return(src)
        }
        Load8Dest::E => {
            cpu.registers.e = reg_src!(cpu, src);
            common_reg_return(src)
        }
        Load8Dest::H => {
            cpu.registers.h = reg_src!(cpu, src);
            common_reg_return(src)
        }
        Load8Dest::L => {
            cpu.registers.l = reg_src!(cpu, src);
            common_reg_return(src)
        }
        _ => panic!("Invalid enum variants passed"),
    };

    InstructionReturn {
        n_cycles,
        n_bytes,
        return_type: ReturnType::NotJumped,
    }
}

pub fn ldh(cpu: &mut CPU, dest: Load8Dest, _src: Load8Src) -> InstructionReturn {
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
