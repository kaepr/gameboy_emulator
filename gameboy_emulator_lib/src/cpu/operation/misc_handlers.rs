use crate::cpu::CPU;

pub fn nop(_cpu: &mut CPU) {}

pub fn di(cpu: &mut CPU) {
    cpu.ime = false;
}
