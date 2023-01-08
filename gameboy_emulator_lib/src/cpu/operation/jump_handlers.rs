use crate::{
    cpu::{registers::Reg16, Cycles, CPU},
    utils::{le_bytes_to_word, word_to_bytes},
};

use super::opcodes::{JumpCondition, RSTTarget};

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
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        let (pc_high, pc_low) = word_to_bytes(cpu.registers.pc);
        cpu.write_byte(cpu.registers.sp, pc_high);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
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

pub fn jr(cpu: &mut CPU, flag: JumpCondition) {
    let s8 = cpu.fetch_byte();
    let to_jump = to_jump!(cpu, flag);

    if to_jump {
        cpu.add_cycles(Cycles::N4);
        cpu.registers.pc = cpu.registers.pc.wrapping_add((s8 as i8) as u16);
    }
}

pub fn ret(cpu: &mut CPU, flag: JumpCondition) {
    match flag {
        JumpCondition::NIL => {
            let lo = cpu.read_byte_bus(cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
            let hi = cpu.read_byte_bus(cpu.registers.sp);
            cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
            cpu.add_cycles(Cycles::N4);
            cpu.registers.pc = le_bytes_to_word(lo, hi);
        }
        _ => {
            let to_jump = to_jump!(cpu, flag);
            cpu.add_cycles(Cycles::N4);

            if to_jump {
                let lo = cpu.read_byte_bus(cpu.registers.sp);
                cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
                let hi = cpu.read_byte_bus(cpu.registers.sp);
                cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
                cpu.add_cycles(Cycles::N4);
                cpu.registers.pc = le_bytes_to_word(lo, hi);
            }
        }
    }
}

pub fn reti(cpu: &mut CPU) {
    let lo = cpu.read_byte_bus(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    let hi = cpu.read_byte_bus(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    cpu.add_cycles(Cycles::N4);
    cpu.registers.pc = le_bytes_to_word(lo, hi);
    cpu.ime = true;
}

pub fn jp_hl(cpu: &mut CPU) {
    let addr = cpu.registers.get_reg_pair(Reg16::HL);
    cpu.registers.pc = addr;
}

pub fn rst(cpu: &mut CPU, target: RSTTarget) {
    let addr = le_bytes_to_word(target as u8, 0x00);

    cpu.add_cycles(Cycles::N4);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);

    let (pc_high, pc_low) = word_to_bytes(cpu.registers.pc);

    cpu.write_byte(cpu.registers.sp, pc_high);
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    cpu.write_byte(cpu.registers.sp, pc_low);

    cpu.registers.pc = addr;
}
