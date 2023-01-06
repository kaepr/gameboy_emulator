use core::panic;

use crate::{
    cpu::{
        registers::{flags::FlagType, Reg16},
        InstructionReturn, ReturnType, CPU,
    },
    utils::le_bytes_to_word,
};

use super::opcodes::{Load16Dest, Load16Src};

pub fn ld(cpu: &mut CPU, dest: Load16Dest, src: Load16Src) -> InstructionReturn {
    let (res, n_cycles, n_bytes) = match src {
        Load16Src::Direct16Bit => {
            let lo = cpu.bus.read(cpu.registers.pc + 1);
            let hi = cpu.bus.read(cpu.registers.pc + 2);

            (le_bytes_to_word(lo, hi), 12, 3)
        }
        Load16Src::SP => (cpu.registers.sp, 20, 3),
        Load16Src::HL => (cpu.registers.get_reg_pair(Reg16::HL), 8, 1),
        Load16Src::SPr8 => {
            let operand = cpu.bus.read(cpu.registers.pc + 1);
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

            (res, 12, 2)
        }
    };

    match dest {
        Load16Dest::BC => {
            cpu.registers.set_reg_pair(res, Reg16::BC);
        }
        Load16Dest::Addr16Bit => {
            let lo = cpu.bus.read(cpu.registers.pc + 1);
            let hi = cpu.bus.read(cpu.registers.pc + 2);
            let addr = le_bytes_to_word(lo, hi);
            cpu.bus.write16(addr, res);
        }
        Load16Dest::DE => {
            cpu.registers.set_reg_pair(res, Reg16::DE);
        }
        Load16Dest::HL => {
            cpu.registers.set_reg_pair(res, Reg16::HL);
        }
        Load16Dest::SP => {
            cpu.registers.set_reg_pair(res, Reg16::SP);
        }
        _ => panic!("Invalid choice of enum variant"),
    };

    InstructionReturn {
        n_cycles,
        n_bytes,
        return_type: ReturnType::NotJumped,
    }
}

pub fn pop(cpu: &mut CPU, dest: Load16Dest) -> InstructionReturn {
    todo!()
}

pub fn push(cpu: &mut CPU, dest: Load16Dest) -> InstructionReturn {
    todo!()
}
