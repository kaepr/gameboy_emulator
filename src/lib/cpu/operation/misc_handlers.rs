use crate::cpu::{InstructionReturn, CPU};

pub fn nop(cpu: &mut CPU) -> InstructionReturn {
    InstructionReturn {
        n_cycles: 4,
        n_bytes: 1,
    }
}
