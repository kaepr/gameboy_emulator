use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        CPU,
    },
    utils::{
        is_bit_set, reset_bit, rotate_left_helper, rotate_right_helper, set_bit, swap_nibbles,
    },
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
                $cpu.read_byte_bus(addr)
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
                $cpu.write_byte(addr, $value);
            }
            BitDest::A => $cpu.registers.a = $value,
        }
    };
}

pub fn res(cpu: &mut CPU, pos: BitPos, dest: BitDest) {
    let bit_pos: u8 = pos.into();

    let value = fetch_value!(cpu, dest);
    let value = value & !(1 << bit_pos);

    set_value!(cpu, dest, value);
}

pub fn set(cpu: &mut CPU, pos: BitPos, dest: BitDest) {
    let bit_pos: u8 = pos.into();

    let value = fetch_value!(cpu, dest);
    let value = value | (1 << bit_pos);

    set_value!(cpu, dest, value);
}

pub fn bit(cpu: &mut CPU, pos: BitPos, src: BitDest) {
    let bit_pos: u8 = pos.into();

    let value = fetch_value!(cpu, src);

    cpu.registers.f.reset_flag(FlagType::Zero);
    cpu.registers.f.reset_flag(FlagType::Sub);
    cpu.registers.f.set_flag(FlagType::HalfCarry);

    if !is_bit_set(value, bit_pos) {
        cpu.registers.f.set_flag(FlagType::Zero);
    }
}

pub fn rlca(cpu: &mut CPU) {
    let (res, carry) = rotate_left_helper(cpu.registers.a, cpu.registers.f.carry, false);
    cpu.registers.f.reset_flags();

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = res;
}

pub fn rla(cpu: &mut CPU) {
    let (res, carry) = rotate_left_helper(cpu.registers.a, cpu.registers.f.carry, true);
    cpu.registers.f.reset_flags();

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = res;
}

pub fn rrca(cpu: &mut CPU) {
    let (res, carry) = rotate_right_helper(cpu.registers.a, cpu.registers.f.carry, false);
    cpu.registers.f.reset_flags();

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = res;
}

pub fn rra(cpu: &mut CPU) {
    let (res, carry) = rotate_right_helper(cpu.registers.a, cpu.registers.f.carry, true);
    cpu.registers.f.reset_flags();

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = res;
}

pub fn rlc(cpu: &mut CPU, dest: BitDest) {
    let (res, carry) = rotate_left_helper(fetch_value!(cpu, dest), cpu.registers.f.carry, false);
    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn rl(cpu: &mut CPU, dest: BitDest) {
    let (res, carry) = rotate_left_helper(fetch_value!(cpu, dest), cpu.registers.f.carry, true);
    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn rrc(cpu: &mut CPU, dest: BitDest) {
    let (res, carry) = rotate_right_helper(fetch_value!(cpu, dest), cpu.registers.f.carry, false);
    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn rr(cpu: &mut CPU, dest: BitDest) {
    let (res, carry) = rotate_right_helper(fetch_value!(cpu, dest), cpu.registers.f.carry, true);
    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn sla(cpu: &mut CPU, dest: BitDest) {
    let byte = fetch_value!(cpu, dest);
    cpu.registers.f.reset_flags();

    let carry = is_bit_set(byte, 7);

    let res = byte << 1;

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn sra(cpu: &mut CPU, dest: BitDest) {
    let byte = fetch_value!(cpu, dest);
    let bit_7 = is_bit_set(byte, 7);
    let carry = is_bit_set(byte, 0);

    let mut res = byte >> 1;

    if bit_7 {
        res = set_bit(res, 7);
    }

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn swap(cpu: &mut CPU, dest: BitDest) {
    let res = swap_nibbles(fetch_value!(cpu, dest));
    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}

pub fn srl(cpu: &mut CPU, dest: BitDest) {
    let byte = fetch_value!(cpu, dest);
    cpu.registers.f.reset_flags();
    let res = byte >> 1;

    let carry = is_bit_set(byte, 0);

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    set_value!(cpu, dest, res);
}
