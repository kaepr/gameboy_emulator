use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        InstructionReturn, CPU,
    },
    utils::is_bit_set,
};

use super::opcodes::{BitDest, BitPos};

macro_rules! fetch_value {
    ($cpu: ident, $dest: ident) => {
        match $dest {
            BitDest::B => $cpu.registers.b,
            BitDest::C => $cpu.registers.c,
            BitDest::D => $cpu.registers.d,
            BitDest::E => $cpu.registers.e,
            BitDest::H => $cpu.registers.h,
            BitDest::L => $cpu.registers.l,
            BitDest::HL => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                $cpu.bus.read(addr)
            }
            BitDest::A => $cpu.registers.a,
        }
    };
}

macro_rules! set_value {
    ($cpu: ident, $dest: ident, $value: ident) => {
        match $dest {
            BitDest::B => $cpu.registers.b = $value,
            BitDest::C => $cpu.registers.c = $value,
            BitDest::D => $cpu.registers.d = $value,
            BitDest::E => $cpu.registers.e = $value,
            BitDest::H => $cpu.registers.h = $value,
            BitDest::L => $cpu.registers.l = $value,
            BitDest::HL => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                $cpu.bus.write(addr, $value);
            }
            BitDest::A => $cpu.registers.a = $value,
        }
    };
}

pub fn res(cpu: &mut CPU, pos: BitPos, dest: BitDest) -> InstructionReturn {
    let bit_pos: u8 = pos.into();

    let value = fetch_value!(cpu, dest);
    let value = value & !(1 << bit_pos);

    set_value!(cpu, dest, value);

    let n_cycles = if let BitDest::HL = dest { 16 } else { 8 };

    InstructionReturn {
        n_cycles,
        n_bytes: 2,
    }
}

pub fn set(cpu: &mut CPU, pos: BitPos, dest: BitDest) -> InstructionReturn {
    let bit_pos: u8 = pos.into();

    let value = fetch_value!(cpu, dest);
    let value = value | (1 << bit_pos);

    set_value!(cpu, dest, value);

    let n_cycles = if let BitDest::HL = dest { 16 } else { 8 };

    InstructionReturn {
        n_cycles,
        n_bytes: 2,
    }
}

pub fn bit(cpu: &mut CPU, pos: BitPos, src: BitDest) -> InstructionReturn {
    let bit_pos: u8 = pos.into();

    let value = fetch_value!(cpu, src);

    cpu.registers.f.reset_flag(FlagType::Zero);
    cpu.registers.f.reset_flag(FlagType::Sub);
    cpu.registers.f.set_flag(FlagType::HalfCarry);

    if !is_bit_set(value, bit_pos) {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    let n_cycles = if let BitDest::HL = src { 12 } else { 8 };

    InstructionReturn {
        n_cycles,
        n_bytes: 2,
    }
}
