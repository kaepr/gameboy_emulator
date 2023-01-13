use crate::{
    bus::{Bus, Memory},
    cartridge::Cartridge,
    interrupt::Interrupts,
    utils::{word_to_bytes, Opts},
};

use self::{operation::Operation, registers::Registers};

mod operation;
mod registers;

pub struct CPU {
    pub registers: Registers,
    pub bus: Bus,
    pub cycles: u64,
    pub ime: bool,
    pub opts: Opts,
    pub halted: bool,
    pub enable_ime_next_cycle: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cycles {
    N4 = 4,
}

impl CPU {
    pub fn new(cartridge: Cartridge, opts: Opts) -> Self {
        CPU {
            registers: Registers::new(),
            cycles: 0,
            bus: Bus::new(cartridge),
            ime: false,
            opts,
            halted: false,
            enable_ime_next_cycle: false,
        }
    }

    pub fn step(&mut self) {
        Interrupts::process_interrupt_request(self);

        if self.halted && !Interrupts::pending_interrupt(self) {
            self.tick();
            return;
        }

        if Interrupts::pending_interrupt(self) {
            self.halted = false;
        }

        if Interrupts::has_interrupt(self) {
            Interrupts::handle_interrupt(self);
            self.ime = false;
            self.halted = false;
        } else {
            self.execute();
        }
    }

    pub fn tick(&mut self) {
        self.add_cycles(Cycles::N4);
        self.bus.timer.tick();

        if self.enable_ime_next_cycle {
            self.ime = true;
            self.enable_ime_next_cycle = false;
        }
    }

    fn add_cycles(&mut self, n_cycles: Cycles) {
        self.cycles += n_cycles as u64;
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.tick();
        byte
    }

    fn read_byte_bus(&mut self, addr: u16) -> u8 {
        let byte = self.bus.read(addr);
        self.tick();
        byte
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.bus.write(addr, byte);
        self.tick();
    }

    fn write_word(&mut self, addr: u16, word: u16) {
        let (hi, lo) = word_to_bytes(word);
        self.write_byte(addr, lo);
        self.write_byte(addr + 1, hi);
    }

    fn print_debug(&self) {
        println!(
            "{} ({:02X} {:02X} {:02X} {:02X})",
            self.registers,
            self.bus.read(self.registers.pc),
            self.bus.read(self.registers.pc + 1),
            self.bus.read(self.registers.pc + 2),
            self.bus.read(self.registers.pc + 3),
        );
    }

    fn execute(&mut self) {
        if self.opts.show_debug_info {
            self.print_debug();
        }

        let mut opcode = self.fetch_byte();
        let prefixed = Operation::is_prefix(opcode);

        if prefixed {
            opcode = self.fetch_byte();
        }

        let op = Operation::get_operation(opcode, prefixed);

        let inst = match op {
            Some(o) => o,
            None => panic!("Unknown opcode {}, prefixed {}", opcode, prefixed),
        };

        if self.opts.show_serial_output {
            self.bus.serial.print_serial_data();
        }

        Operation::execute(self, inst);
    }
}
