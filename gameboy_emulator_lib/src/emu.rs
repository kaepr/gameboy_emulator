use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::{Bus, Memory},
    cartridge::Cartridge,
    cpu::CPU,
    interrupt::Interrupts,
    io::timer::Timer,
    utils::Opts,
};

pub struct EmuContext {
    pub cpu: CPU,
    pub bus: Rc<RefCell<Bus>>,
    pub opts: Opts,
}

impl EmuContext {
    pub fn new(cart: Cartridge, opts: Opts) -> Self {
        let bus = Rc::new(RefCell::new(Bus::new(cart)));

        EmuContext {
            cpu: CPU::new(bus.clone()),
            bus,
            opts,
        }
    }

    fn print_debug(&self) {
        println!(
            "{} ({:02X} {:02X} {:02X} {:02X})",
            self.cpu.registers,
            self.bus.borrow().read(self.cpu.registers.pc),
            self.bus.borrow().read(self.cpu.registers.pc + 1),
            self.bus.borrow().read(self.cpu.registers.pc + 2),
            self.bus.borrow().read(self.cpu.registers.pc + 3),
        );
    }

    pub fn step(&mut self) {
        loop {
            if self.opts.show_debug_info {
                self.print_debug();
            }
            let mut n_cycles = self.cpu.step();

            // println!("n_cycles {}", n_cycles);
            n_cycles /= 4;

            for _ in 0..n_cycles {
                self.bus.borrow_mut().tick();
            }

            if self.opts.show_serial_output {
                self.bus.borrow_mut().serial.print_serial_data();
            }
        }
    }
}
