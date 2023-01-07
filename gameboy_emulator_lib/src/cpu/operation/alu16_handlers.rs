use super::opcodes::ALU16Dest;
use crate::cpu::{
    registers::{flags::FlagType, Reg16},
    Cycles, CPU,
};

macro_rules! get_pair {
    ($cpu:ident, $dest: ident) => {
        match $dest {
            ALU16Dest::BC => $cpu.registers.get_reg_pair(Reg16::BC),
            ALU16Dest::HL => $cpu.registers.get_reg_pair(Reg16::HL),
            ALU16Dest::DE => $cpu.registers.get_reg_pair(Reg16::DE),
            ALU16Dest::SP => $cpu.registers.get_reg_pair(Reg16::SP),
        }
    };
}

macro_rules! set_pair {
    ($cpu:ident, $dest: ident, $value: ident) => {
        match $dest {
            ALU16Dest::BC => $cpu.registers.set_reg_pair($value, Reg16::BC),
            ALU16Dest::HL => $cpu.registers.set_reg_pair($value, Reg16::HL),
            ALU16Dest::DE => $cpu.registers.set_reg_pair($value, Reg16::DE),
            ALU16Dest::SP => $cpu.registers.set_reg_pair($value, Reg16::SP),
        }
    };
}
pub fn inc(cpu: &mut CPU, dest: ALU16Dest) {
    let word = get_pair!(cpu, dest);
    let res = word.wrapping_add(1);
    cpu.add_cycles(Cycles::N4);
    set_pair!(cpu, dest, res);
}

pub fn dec(cpu: &mut CPU, dest: ALU16Dest) {
    let word = get_pair!(cpu, dest);
    let res = word.wrapping_sub(1);
    cpu.add_cycles(Cycles::N4);
    set_pair!(cpu, dest, res);
}
