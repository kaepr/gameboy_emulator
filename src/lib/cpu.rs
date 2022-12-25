use crate::bus::Bus;

use self::operation::Operation;
use self::operation::{
    bit_handlers::{bit, res, set},
    opcodes::BitOp,
};
use self::registers::Registers;

mod instruction;
mod operation;
mod registers;

pub struct CPU {
    registers: Registers,
    bus: Bus,
    cycles: u64,
}

pub struct InstructionReturn {
    n_cycles: u8,
    n_bytes: u8,
}

impl CPU {
    // fn new() -> Self {
    //     CPU {}
    // }
    //

    pub fn step(&mut self) {
        todo!();
    }

    fn execute(&mut self) {
        let mut opcode = self.bus.read(self.registers.pc);
        let prefixed = Operation::is_prefix(opcode);

        if prefixed {
            opcode = self.bus.read(self.registers.pc + 1);
        }

        let op = Operation::get_operation(opcode, prefixed);

        let inst = match op {
            Some(o) => o,
            None => panic!("Unknown opcode {}, prefixed {}", opcode, prefixed),
        };

        self.execute_inst(inst);
    }

    fn execute_inst(&mut self, inst: Operation) -> InstructionReturn {
        match inst {
            Operation::Misc(_) => todo!(),
            Operation::Load8(_) => todo!(),
            Operation::Load16(_) => todo!(),
            Operation::ALU16(_) => todo!(),
            Operation::ALU8(_) => todo!(),
            Operation::Bit(o) => match o {
                BitOp::RLCA => todo!(),
                BitOp::RRCA => todo!(),
                BitOp::RLA => todo!(),
                BitOp::RRA => todo!(),
                BitOp::RLC(_) => todo!(),
                BitOp::RRC(_) => todo!(),
                BitOp::RL(_) => todo!(),
                BitOp::RR(_) => todo!(),
                BitOp::SLA(_) => todo!(),
                BitOp::SRA(_) => todo!(),
                BitOp::SWAP(_) => todo!(),
                BitOp::SRL(_) => todo!(),
                BitOp::BIT(pos, dest) => bit(self, pos, dest),
                BitOp::RES(pos, dest) => res(self, pos, dest),
                BitOp::SET(pos, dest) => set(self, pos, dest),
            },
            Operation::Jump(_) => todo!(),
        }
    }
}

// #[cfg(test)]
// mod tests {}
