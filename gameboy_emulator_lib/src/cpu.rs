use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::{Bus, Memory},
    cartridge::Cartridge,
    interrupt::Interrupts,
    utils::{reset_bit, word_to_bytes, Opts},
};

use self::{operation::Operation, registers::Registers};

mod operation;
mod registers;

pub struct CPU {
    pub registers: Registers,
    pub bus: Rc<RefCell<Bus>>,
    pub cycles: u64,
    pub ime: bool,
    pub halted: bool,
    pub enable_ime_next_cycle: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cycles {
    N4 = 4,
}

impl CPU {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        CPU {
            registers: Registers::new(),
            cycles: 0,
            bus,
            ime: false,
            halted: false,
            enable_ime_next_cycle: false,
        }
    }

    fn has_interrupt(&self) -> bool {
        let flag = self.bus.borrow().read(0xFF0F);
        let enable = self.bus.borrow().read(0xFFFF);
        (flag & enable) > 0
    }

    pub fn step(&mut self) -> u64 {
        let cur_cycles = self.cycles;

        if self.halted && self.has_interrupt() {
            self.tick();
            return self.cycles - cur_cycles;
        }

        if self.has_interrupt() {
            self.halted = false;
        }

        if self.ime && self.has_interrupt() {
            self.handle_interrupt();
            self.ime = false;
            self.halted = false;
        } else {
            self.execute();
        }

        self.cycles - cur_cycles
    }

    fn handle_interrupt(&mut self) {
        self.tick();
        self.tick();

        let (pc_high, pc_low) = word_to_bytes(self.registers.pc);

        let flag = self.bus.borrow().read(0xFF0F);
        let enable = self.bus.borrow().read(0xFFFF);

        let it_type = Interrupts::interrupt_type(enable, flag);
        let address = Interrupts::interrupt_addr(it_type);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, pc_high);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.write_byte(self.registers.sp, pc_low);
        self.registers.pc = address;
        self.tick();

        let flag_resetted = reset_bit(flag, it_type as usize);
        self.bus.borrow_mut().write(0xFF0F, flag_resetted);
    }

    pub fn tick(&mut self) {
        self.add_cycles(Cycles::N4);

        if self.enable_ime_next_cycle {
            self.ime = true;
            self.enable_ime_next_cycle = false;
        }
    }

    fn add_cycles(&mut self, n_cycles: Cycles) {
        self.cycles += n_cycles as u64;
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.borrow().read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.tick();
        byte
    }

    fn read_byte_bus(&mut self, addr: u16) -> u8 {
        let byte = self.bus.borrow().read(addr);
        self.tick();
        byte
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.bus.borrow_mut().write(addr, byte);
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
            self.bus.borrow().read(self.registers.pc),
            self.bus.borrow().read(self.registers.pc + 1),
            self.bus.borrow().read(self.registers.pc + 2),
            self.bus.borrow().read(self.registers.pc + 3),
        );
    }

    fn execute(&mut self) {
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

        Operation::execute(self, inst);
    }
}
