use crate::cpu::{InstructionReturn, ReturnType::NotJumped, CPU};

pub fn nop(cpu: &mut CPU) -> InstructionReturn {
    InstructionReturn {
        n_cycles: 4,
        n_bytes: 1,
        return_type: NotJumped,
    }
}

pub fn di(cpu: &mut CPU) -> InstructionReturn {
    cpu.ime = false; // TODO!: prohibit maskable interuppts
    
    InstructionReturn {
        n_cycles: 4,
        n_bytes: 1,
        return_type: NotJumped,
    }
}
