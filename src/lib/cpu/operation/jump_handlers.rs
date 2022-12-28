use crate::cpu::InstructionReturn;

use super::opcodes::Flags;

pub fn jp(f: Flags) -> InstructionReturn {
    match f {
        Flags::NIL => todo!(),
        Flags::Z => todo!(),
        Flags::C => todo!(),
        Flags::NZ => todo!(),
        Flags::NC => todo!(),
    };

    InstructionReturn {
        n_cycles: todo!(),
        n_bytes: todo!(),
    }
    
}
