use crate::cpu::{registers::Reg8, InstructionReturn, ReturnType, CPU};

use super::opcodes::{Load8Dest, Load8Src};

pub fn ld(cpu: &mut CPU, dest: Load8Dest, src: Load8Src) -> InstructionReturn {
    InstructionReturn {
        n_cycles: (),
        n_bytes: (),
        return_type: ReturnType::NotJumped,
    }
}

pub fn ldh(cpu: &mut CPU, dest: Load8Dest, src: Load8Src) -> InstructionReturn {
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
