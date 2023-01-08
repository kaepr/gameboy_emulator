use crate::cpu::CPU;

pub struct Interrupts;

pub enum InterruptType {
    VBLANK = 0,
    LCDSTAT = 1,
    TIMER = 2,
    SERIAL = 3,
    JOYPAD = 4,
}

impl Interrupts {
    pub fn handle_interrupt(cpu: &mut CPU) {}
}
