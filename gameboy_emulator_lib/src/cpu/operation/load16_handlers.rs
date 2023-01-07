use core::panic;

use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        Cycles, CPU,
    },
    utils::{le_bytes_to_word, word_to_bytes},
};

use super::opcodes::{Load16Dest, Load16Src};

pub fn ld(cpu: &mut CPU, dest: Load16Dest, src: Load16Src) {
    let res = match src {
        Load16Src::Direct16Bit => {
            let lo = cpu.fetch_byte();
            let hi = cpu.fetch_byte();
            le_bytes_to_word(lo, hi)
        }
        Load16Src::SP => cpu.registers.sp,
        Load16Src::HL => cpu.registers.get_reg_pair(Reg16::HL),
        Load16Src::SPr8 => {
            let operand = cpu.fetch_byte();
            let r8 = operand as i8;
            let sp = cpu.registers.sp;
            let res = sp.wrapping_add(r8 as u16);

            cpu.registers.f.reset_flags();

            if r8 > 0 {
                if ((sp & 0xFF) + operand as u16) > 0xFF {
                    cpu.registers.f.set_flag(FlagType::Carry);
                }

                if ((sp & 0xF) + (operand as u16 & 0xF)) > 0xF {
                    cpu.registers.f.set_flag(FlagType::HalfCarry);
                }
            } else {
                if (res & 0xFF) < (sp & 0xFF) {
                    cpu.registers.f.set_flag(FlagType::Carry);
                }

                if (res & 0xF) < (sp & 0xF) {
                    cpu.registers.f.set_flag(FlagType::HalfCarry);
                }
            }

            res
        }
    };

    match dest {
        Load16Dest::BC => {
            cpu.registers.set_reg_pair(res, Reg16::BC);
        }
        Load16Dest::Addr16Bit => {
            let lo = cpu.fetch_byte();
            let hi = cpu.fetch_byte();
            let addr = le_bytes_to_word(lo, hi);
            cpu.write_word(addr, res);
        }
        Load16Dest::DE => {
            cpu.registers.set_reg_pair(res, Reg16::DE);
        }
        Load16Dest::HL => {
            cpu.registers.set_reg_pair(res, Reg16::HL);
        }
        Load16Dest::SP => {
            cpu.registers.set_reg_pair(res, Reg16::SP);
            cpu.add_cycles(Cycles::N4);
        }
        _ => panic!("Invalid choice of enum variant"),
    };
}

pub fn pop(cpu: &mut CPU, dest: Load16Dest) {
    let lo = cpu.read_byte_bus(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let hi = cpu.read_byte_bus(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);

    let word = le_bytes_to_word(lo, hi);

    match dest {
        Load16Dest::BC => cpu.registers.set_reg_pair(word, Reg16::BC),
        Load16Dest::DE => cpu.registers.set_reg_pair(word, Reg16::DE),
        Load16Dest::HL => cpu.registers.set_reg_pair(word, Reg16::HL),
        Load16Dest::AF => cpu.registers.set_reg_pair(word, Reg16::AF),
        _ => panic!("invalid choice"),
    };
}

pub fn push(cpu: &mut CPU, dest: Load16Dest) {
    let word = match dest {
        Load16Dest::BC => cpu.registers.get_reg_pair(Reg16::BC),
        Load16Dest::DE => cpu.registers.get_reg_pair(Reg16::DE),
        Load16Dest::HL => cpu.registers.get_reg_pair(Reg16::HL),
        Load16Dest::AF => cpu.registers.get_reg_pair(Reg16::AF),
        _ => panic!("invalid choice"),
    };

    let (hi, lo) = word_to_bytes(word);

    cpu.add_cycles(Cycles::N4);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.write_byte(cpu.registers.sp, hi);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.write_byte(cpu.registers.sp, lo);
}
