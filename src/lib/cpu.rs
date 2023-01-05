use crate::bus::Bus;

use self::{
    operation::{
        bit_handlers::{bit, res, set},
        jump_handlers::jp,
        load16_handlers::{ld as ld16, pop, push},
        load8_handlers::{ld as ld8, ldh},
        misc_handlers::{di, nop},
        opcodes::{BitOp, JumpOp, Load16Op, Load8Op, MiscOp},
        Operation,
    },
    registers::Registers,
};

mod operation;
mod registers;

pub struct CPU {
    pub registers: Registers,
    pub bus: Bus,
    pub cycles: u64,
    pub ime: bool,
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
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            cycles: 0,
            bus: Bus::new(),
            ime: false,
        }
    }

    pub fn step(&mut self) {
        self.execute();
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

        println!(
            "PC: 0x{:04x} | Inst: {:?} 0x{:02x}",
            self.registers.pc, inst, opcode
        );

        let InstructionReturn {
            n_bytes,
            n_cycles,
            return_type,
        } = self.execute_inst(inst);

        match return_type {
            ReturnType::Jumped => self.cycles += n_cycles,
            ReturnType::NotJumped => {
                self.registers.pc += n_bytes;
                self.cycles += n_cycles;
            }
        };
    }

    fn execute_inst(&mut self, inst: Operation) -> InstructionReturn {
        match inst {
            Operation::Misc(o) => match o {
                MiscOp::NOP => nop(self),
                MiscOp::STOP => todo!(),
                MiscOp::HALT => todo!(),
                MiscOp::PREFIX => todo!(),
                MiscOp::EI => todo!(),
                MiscOp::DI => di(self),
            },
            Operation::Load8(o) => match o {
                Load8Op::LD(dest, src) => ld8(self, dest, src),
                Load8Op::LDH(dest, src) => ldh(self, dest, src),
            },
            Operation::Load16(o) => match o {
                Load16Op::LD(dest, src) => ld16(self, dest, src),
                Load16Op::POP(dest) => pop(self, dest),
                Load16Op::PUSH(dest) => push(self, dest),
            },
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
            Operation::Jump(o) => match o {
                JumpOp::RETI => todo!(),
                JumpOp::JR(_) => todo!(),
                JumpOp::JPToHL => todo!(),
                JumpOp::JP(f) => jp(self, f),
                JumpOp::RET(_) => todo!(),
                JumpOp::CALL(_) => todo!(),
                JumpOp::RST(_) => todo!(),
            },
        }
    }
}
