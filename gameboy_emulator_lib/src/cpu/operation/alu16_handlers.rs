use super::opcodes::{ALU16Dest, ALU16Src};
use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        Cycles, CPU,
    },
    utils::HalfCarryCheck,
};

macro_rules! get_pair_dest {
    ($cpu:ident, $dest: ident) => {
        match $dest {
            ALU16Dest::BC => $cpu.registers.get_reg_pair(Reg16::BC),
            ALU16Dest::HL => $cpu.registers.get_reg_pair(Reg16::HL),
            ALU16Dest::DE => $cpu.registers.get_reg_pair(Reg16::DE),
            ALU16Dest::SP => $cpu.registers.get_reg_pair(Reg16::SP),
        }
    };
}

macro_rules! get_pair_src {
    ($cpu:ident, $src: ident) => {
        match $src {
            ALU16Src::BC => $cpu.registers.get_reg_pair(Reg16::BC),
            ALU16Src::HL => $cpu.registers.get_reg_pair(Reg16::HL),
            ALU16Src::DE => $cpu.registers.get_reg_pair(Reg16::DE),
            ALU16Src::SP => $cpu.registers.get_reg_pair(Reg16::SP),
            _ => panic!("invalid enum variant passesd"),
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
    let word = get_pair_dest!(cpu, dest);
    let res = word.wrapping_add(1);
    cpu.add_cycles(Cycles::N4);
    set_pair!(cpu, dest, res);
}

pub fn dec(cpu: &mut CPU, dest: ALU16Dest) {
    let word = get_pair_dest!(cpu, dest);
    let res = word.wrapping_sub(1);
    cpu.add_cycles(Cycles::N4);
    set_pair!(cpu, dest, res);
}

pub fn add(cpu: &mut CPU, dest: ALU16Dest, src: ALU16Src) {
    if dest == ALU16Dest::SP {
        let r8 = cpu.fetch_byte();
        let sp = cpu.registers.sp;

        let res = sp.wrapping_add((r8 as i8) as u16);

        cpu.registers.f.reset_flags();

        if (r8 as i8) > 0 {
            if ((sp & 0xFF) + (r8 as u16)) > 0xFF {
                cpu.registers.f.set_flag(FlagType::Carry);
            }

            if ((sp & 0x0F) + (r8 as u16 & 0x0F)) > 0x0F {
                cpu.registers.f.set_flag(FlagType::HalfCarry);
            }
        } else {
            if (res & 0xFF) < (sp & 0xFF) {
                cpu.registers.f.set_flag(FlagType::Carry);
            }

            if res.half_carry_sub(sp) {
                cpu.registers.f.set_flag(FlagType::HalfCarry);
            }
        }

        cpu.add_cycles(Cycles::N4);
        cpu.add_cycles(Cycles::N4);

        cpu.registers.sp = res;
    } else {
        let orig = get_pair_dest!(cpu, dest);
        let word = get_pair_src!(cpu, src);
        let (res, carry) = orig.overflowing_add(word);

        cpu.add_cycles(Cycles::N4);
        set_pair!(cpu, dest, res);

        cpu.registers.f.reset_flag(FlagType::Sub);
        cpu.registers.f.reset_flag(FlagType::HalfCarry);
        cpu.registers.f.reset_flag(FlagType::Carry);

        if orig.half_carry_add(word) {
            cpu.registers.f.set_flag(FlagType::HalfCarry);
        }

        if carry {
            cpu.registers.f.set_flag(FlagType::Carry);
        }
    }
}
