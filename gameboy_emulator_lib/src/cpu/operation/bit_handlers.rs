use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        CPU,
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
    let acc = cpu.registers.a;
    cpu.registers.f.reset_flags();

    let carry = (acc & 0b10000000) > 0;

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = acc.rotate_left(1);
}

pub fn rla(cpu: &mut CPU) {
    let mut acc = cpu.registers.a;
    cpu.registers.f.reset_flags();

    let bit_0 = cpu.registers.f.carry;

    let carry = (acc & 0b10000000) > 0;

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    acc = acc.rotate_left(1);

    if !bit_0 {
        acc = acc & 0b11111110;
    } else {
        acc = acc | 0b11111111;
    }

    cpu.registers.a = acc;
}

pub fn rrca(cpu: &mut CPU) {
    let acc = cpu.registers.a;
    cpu.registers.f.reset_flags();

    let carry = acc & 0b00000001 > 0;

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = acc.rotate_right(1);
}

pub fn rra(cpu: &mut CPU) {
    let mut acc = cpu.registers.a;
    cpu.registers.f.reset_flags();

    let bit_7 = cpu.registers.f.carry;

    let carry = (acc & 0b00000001) > 0;

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    acc = acc.rotate_right(1);

    if !bit_7 {
        acc = acc & 0b01111111;
    } else {
        acc = acc | 0b11111111;
    }

    cpu.registers.a = acc;
}

fn rlc_helper(byte: u8) -> (u8, bool) {
    let res = byte.rotate_left(1);
    let carry = (res & 0b00000001) > 0;
    (res, carry)
}

pub fn rlc(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
    let (res, carry) = rlc_helper(fetch_value!(cpu, dest));
    cpu.registers.a = res;

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }
}

pub fn rl(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}

pub fn rrc(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}

pub fn rr(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}

pub fn sla(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}

pub fn sra(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}

pub fn swap(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}

pub fn srl(cpu: &mut CPU, dest: BitDest) {
    cpu.registers.f.reset_flags();
}
