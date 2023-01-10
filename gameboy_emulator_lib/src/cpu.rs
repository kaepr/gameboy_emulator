use crate::{
    bus::{Bus, Memory},
    cartridge::Cartridge,
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
    pub debug_info: String,
    pub opts: Opts,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cycles {
    N4 = 4,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ReturnType {
    Jumped,
    NotJumped,
}

pub struct InstructionReturn {
    pub n_cycles: u64,
    pub n_bytes: u16,
    pub return_type: ReturnType,
}

impl CPU {
    pub fn new(cartridge: Cartridge, opts: Opts) -> Self {
        CPU {
            registers: Registers::new(),
            cycles: 0,
            bus: Bus::new(cartridge),
            ime: false,
            debug_info: "".to_string(),
            opts,
        }
    }

    pub fn step(&mut self) {
        self.execute();
    }

    pub fn tick(&mut self) {
        self.add_cycles(Cycles::N4);
        self.bus.timer.tick();
        self.bus.timer.tick();
        self.bus.timer.tick();
        self.bus.timer.tick();
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

    fn write_byte(&mut self, addr: u16, byte: u8) {
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
            let b = self.bus.read(0xFF02);
            if b == 0x81 {
                let c = self.bus.read(0xFF01);
                self.debug_info.push(c as char);
                println!("{}", self.debug_info);
                self.bus.write(0xFF02, 0);
            }
        }

        Operation::execute(self, inst);
    }
}
