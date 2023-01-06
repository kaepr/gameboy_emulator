use crate::{
    cpu::{
        self, Cycles, InstructionReturn,
        ReturnType::{Jumped, NotJumped},
        CPU,
    },
    utils::{le_bytes_to_word, word_to_bytes},
};

use super::opcodes::JumpCondition;

macro_rules! to_jump {
    ($cpu: ident, $jump_condition: ident) => {
        match $jump_condition {
            JumpCondition::NIL => true, // always take this action
            JumpCondition::Z => $cpu.registers.f.zero,
            JumpCondition::C => $cpu.registers.f.carry,
            JumpCondition::NZ => !$cpu.registers.f.zero,
            JumpCondition::NC => !$cpu.registers.f.carry,
        }
    };
}

pub fn call(cpu: &mut CPU, flag: JumpCondition) {
    let lo = cpu.fetch_byte();
    let hi = cpu.fetch_byte();
    let addr = le_bytes_to_word(lo, hi);

    let to_jump = to_jump!(cpu, flag);

    if to_jump {
        cpu.add_cycles(Cycles::N4);
        cpu.registers.sp -= 1;
        let (pc_high, pc_low) = word_to_bytes(cpu.registers.pc);
        cpu.write_byte(cpu.registers.sp, pc_high);
        cpu.registers.sp -= 1;
        cpu.write_byte(cpu.registers.sp, pc_low);
        cpu.registers.pc = addr;
    }
}

pub fn jp(cpu: &mut CPU, flag: JumpCondition) {
    let lo = cpu.fetch_byte();
    let hi = cpu.fetch_byte();
    let addr = le_bytes_to_word(lo, hi);

    let to_jump = to_jump!(cpu, flag);

    if to_jump {
        cpu.add_cycles(Cycles::N4);
        cpu.registers.pc = addr;
    }
}
