use crate::cpu::CPU;

pub fn nop(cpu: &mut CPU) {}

pub fn di(cpu: &mut CPU) {
    cpu.ime = false; 
}
