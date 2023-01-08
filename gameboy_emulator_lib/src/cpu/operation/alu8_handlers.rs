use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        CPU,
    },
    utils::HalfCarryCheck,
};

use super::opcodes::{ALU8Dest, ALU8Src};

macro_rules! get_reg_dest {
    ($cpu: ident, $dest: ident) => {
        match $dest {
            ALU8Dest::A => $cpu.registers.a,
            ALU8Dest::B => $cpu.registers.b,
            ALU8Dest::C => $cpu.registers.c,
            ALU8Dest::D => $cpu.registers.d,
            ALU8Dest::E => $cpu.registers.e,
            ALU8Dest::H => $cpu.registers.h,
            ALU8Dest::L => $cpu.registers.l,
            ALU8Dest::HL => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                $cpu.read_byte_bus(addr)
            }
            ALU8Dest::Direct8Bit => $cpu.fetch_byte(),
        }
    };
}

macro_rules! get_reg_src {
    ($cpu: ident, $src: ident) => {
        match $src {
            ALU8Src::A => $cpu.registers.a,
            ALU8Src::B => $cpu.registers.b,
            ALU8Src::C => $cpu.registers.c,
            ALU8Src::D => $cpu.registers.d,
            ALU8Src::E => $cpu.registers.e,
            ALU8Src::H => $cpu.registers.h,
            ALU8Src::L => $cpu.registers.l,
            ALU8Src::HL => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                $cpu.read_byte_bus(addr)
            }
            ALU8Src::Direct8Bit => $cpu.fetch_byte(),
        }
    };
}

macro_rules! set_reg {
    ($cpu: ident, $dest: ident, $value: ident) => {
        match $dest {
            ALU8Dest::A => {
                $cpu.registers.a = $value;
            }
            ALU8Dest::B => {
                $cpu.registers.b = $value;
            }
            ALU8Dest::C => {
                $cpu.registers.c = $value;
            }
            ALU8Dest::D => {
                $cpu.registers.d = $value;
            }
            ALU8Dest::E => {
                $cpu.registers.e = $value;
            }
            ALU8Dest::H => {
                $cpu.registers.h = $value;
            }
            ALU8Dest::L => {
                $cpu.registers.l = $value;
            }
            ALU8Dest::HL => {
                let addr = $cpu.registers.get_reg_pair(Reg16::HL);
                $cpu.write_byte(addr, $value)
            }
            _ => panic!("invalid enum choice"),
        }
    };
}

fn inc_byte(byte: u8) -> (u8, bool) {
    let res = byte.wrapping_add(1);
    let is_half = (((byte & 0x0F) + (1 & 0x0F)) & 0x10) == 0x10;
    (res, is_half)
}

pub fn inc(cpu: &mut CPU, dest: ALU8Dest) {
    let (result, is_half) = inc_byte(get_reg_dest!(cpu, dest));
    set_reg!(cpu, dest, result);

    cpu.registers.f.reset_flag(FlagType::Zero);
    cpu.registers.f.reset_flag(FlagType::Sub);
    cpu.registers.f.reset_flag(FlagType::HalfCarry);

    if result == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if is_half {
        cpu.registers.f.set_flag(FlagType::HalfCarry);
    }
}

fn dec_byte(byte: u8) -> (u8, bool) {
    let res = byte.wrapping_sub(1);
    let is_half = (byte & 0x0F) < (res & 0x0F);
    (res, is_half)
}

pub fn dec(cpu: &mut CPU, dest: ALU8Dest) {
    let (result, is_half) = dec_byte(get_reg_dest!(cpu, dest));
    set_reg!(cpu, dest, result);

    cpu.registers.f.reset_flag(FlagType::Zero);
    cpu.registers.f.reset_flag(FlagType::HalfCarry);

    cpu.registers.f.set_flag(FlagType::Sub);

    if result == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if is_half {
        cpu.registers.f.set_flag(FlagType::HalfCarry);
    }
}

pub fn or(cpu: &mut CPU, dest: ALU8Dest) {
    let byte = get_reg_dest!(cpu, dest);
    let acc = cpu.registers.a;
    let res = byte | acc;
    cpu.registers.a = res;

    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }
}

fn update_sub_cp(cpu: &mut CPU, byte: u8) -> u8 {
    let res = cpu.registers.a.wrapping_sub(byte);

    cpu.registers.f.reset_flags();
    cpu.registers.f.set_flag(FlagType::Sub);

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if cpu.registers.a.half_carry_sub(byte) {
        cpu.registers.f.set_flag(FlagType::HalfCarry);
    }

    if (cpu.registers.a & 0xFF) < (byte & 0xFF) {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    res
}

pub fn sub(cpu: &mut CPU, dest: ALU8Dest) {
    let byte = get_reg_dest!(cpu, dest);
    let res = update_sub_cp(cpu, byte);
    cpu.registers.a = res;
}

pub fn cp(cpu: &mut CPU, dest: ALU8Dest) {
    let byte = get_reg_dest!(cpu, dest);
    update_sub_cp(cpu, byte);
}

pub fn and(cpu: &mut CPU, dest: ALU8Dest) {
    let byte = get_reg_dest!(cpu, dest);
    let res = cpu.registers.a & byte;
    cpu.registers.a = res;

    cpu.registers.f.reset_flags();
    cpu.registers.f.set_flag(FlagType::HalfCarry);

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }
}

pub fn xor(cpu: &mut CPU, dest: ALU8Dest) {
    let byte = get_reg_dest!(cpu, dest);
    let res = cpu.registers.a ^ byte;
    cpu.registers.a = res;

    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }
}

pub fn add(cpu: &mut CPU, _dest: ALU8Dest, src: ALU8Src) {
    let byte = get_reg_src!(cpu, src);
    let acc = cpu.registers.a;

    let (res, carry) = acc.overflowing_add(byte);
    cpu.registers.a = res;

    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    if acc.half_carry_add(byte) {
        cpu.registers.f.set_flag(FlagType::HalfCarry);
    }
}

pub fn adc(cpu: &mut CPU, _dest: ALU8Dest, src: ALU8Src) {
    let byte = get_reg_src!(cpu, src);
    let acc = cpu.registers.a;

    let (res, carry) = acc
        .wrapping_add(cpu.registers.f.carry as u8)
        .overflowing_add(byte);

    cpu.registers.a = res;

    cpu.registers.f.reset_flags();

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    if acc.half_carry_add(byte) {
        cpu.registers.f.set_flag(FlagType::HalfCarry);
    }
}

pub fn sbc(cpu: &mut CPU, _dest: ALU8Dest, src: ALU8Src) {
    let byte = get_reg_src!(cpu, src);
    let acc = cpu.registers.a;
    let carry = cpu.registers.f.carry as u8;

    let res = acc.wrapping_sub(byte).wrapping_sub(carry);

    cpu.registers.f.reset_flags();
    cpu.registers.f.set_flag(FlagType::Sub);

    if res == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if (acc & 0x0F) < ((byte & 0x0F) + carry) {
        cpu.registers.f.set_flag(FlagType::HalfCarry);
    }

    if (acc as u16 & 0xFF) < ((byte as u16 & 0xFF) + carry as u16) {
        cpu.registers.f.set_flag(FlagType::Carry);
    }

    cpu.registers.a = res;
}

pub fn cpl(cpu: &mut CPU) {
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.f.set_flag(FlagType::Sub);
    cpu.registers.f.set_flag(FlagType::HalfCarry);
}

pub fn ccf(cpu: &mut CPU) {
    cpu.registers.f.reset_flag(FlagType::Sub);
    cpu.registers.f.reset_flag(FlagType::HalfCarry);

    if cpu.registers.f.carry {
        cpu.registers.f.reset_flag(FlagType::Carry);
    } else {
        cpu.registers.f.set_flag(FlagType::Carry);
    }
}

pub fn scf(cpu: &mut CPU) {
    cpu.registers.f.reset_flag(FlagType::Sub);
    cpu.registers.f.reset_flag(FlagType::HalfCarry);
    cpu.registers.f.set_flag(FlagType::Carry);
}

/// https://ehaskins.com/2018-01-30%20Z80%20DAA/
/// Mooneye gb
pub fn daa(cpu: &mut CPU) {
    let sub = cpu.registers.f.sub;
    let carry = cpu.registers.f.carry;
    let half_carry = cpu.registers.f.half_carry;

    let mut acc = cpu.registers.a;
    let mut is_carry = false;

    if !sub {
        if carry || (acc > 0x99) {
            acc = acc.wrapping_add(0x60);
            is_carry = true;
        }

        if half_carry || ((acc & 0x0F) > 0x09) {
            acc = acc.wrapping_add(0x06);
        }
    } else if carry {
        is_carry = true;
        let to_add = if half_carry { 0x9A } else { 0xA0 };
        acc = acc.wrapping_add(to_add);
    } else if half_carry {
        acc = acc.wrapping_add(0xFA);
    }

    cpu.registers.a = acc;

    cpu.registers.f.reset_flag(FlagType::Zero);
    cpu.registers.f.reset_flag(FlagType::HalfCarry);
    cpu.registers.f.reset_flag(FlagType::Carry);

    if acc == 0 {
        cpu.registers.f.set_flag(FlagType::Zero);
    }

    if is_carry {
        cpu.registers.f.set_flag(FlagType::Carry);
    }
}
