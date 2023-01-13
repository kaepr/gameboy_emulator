use crate::cpu::CPU;

pub fn nop(_cpu: &mut CPU) {}

pub fn di(cpu: &mut CPU) {
    cpu.ime = false;
}

pub fn ei(cpu: &mut CPU) {
    cpu.enable_ime_next_cycle = true;
}

pub fn stop(cpu: &mut CPU) {
    cpu.tick();
}

pub fn halt(cpu: &mut CPU) {
    cpu.halted = true;
}
