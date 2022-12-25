use crate::{bus::Bus, utils::bytes_to_word};

use self::instruction::Instruction;
use self::operation::Operation;
use self::registers::Registers;

mod instruction;
mod registers;
mod operation;

pub struct CPU {
    registers: Registers,
    bus: Bus,
    cycles: u64,
}

impl CPU {
    // fn new() -> Self {
    //     CPU {}
    // }
    //

    pub fn step(&mut self) {
        todo!();
    }

    fn execute(&self) {
        let mut opcode = self.bus.read(self.registers.pc);
        let prefixed = Operation::is_prefix(opcode);

        if prefixed {
            opcode = self.bus.read(self.registers.pc + 1);
        }

        let op = Operation::get_operation(opcode, prefixed);
    
    }
}

// #[cfg(test)]
// mod tests {}
