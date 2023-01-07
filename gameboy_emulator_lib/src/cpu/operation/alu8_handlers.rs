use crate::cpu::{
    registers::{flags::FlagType, Reg16},
    CPU,
};

use super::opcodes::ALU8Dest;

macro_rules! get_reg {
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
            _ => panic!("invalid enum choice"),
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
    let (result, is_half) = inc_byte(get_reg!(cpu, dest));
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
    let (result, is_half) = dec_byte(get_reg!(cpu, dest));
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
