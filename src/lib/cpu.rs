use crate::{bus::Bus, utils::bytes_to_word};

use self::instruction::Instruction;
use self::registers::Registers;

mod instruction;
mod registers;

pub struct CPU {
    registers: Registers,
    bus: Bus,
}

impl CPU {
    // fn new() -> Self {
    //     CPU {}
    // }

    fn execute(&self, inst: Instruction) {}
}

// #[cfg(test)]
// mod tests {}
