use crate::{bus::Bus, cartridge::Cartridge, cpu::CPU, interrupt::Interrupts, io::timer::Timer};

pub struct Context {
    cpu: CPU,
    bus: Bus,
    cartridge: Cartridge,
    timer: Timer,
    interrupts: Interrupts,
}

impl Context {
    pub fn new() -> Self {
        Context {
            cpu: todo!(),
            bus: todo!(),
            cartridge: todo!(),
            timer: todo!(),
            interrupts: Interrupts::new(),
        }
    }

    pub fn step() {
        loop {}
    }
}
