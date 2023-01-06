use crate::{bus::Bus, utils::word_to_bytes};

use self::{
    operation::{
        bit_handlers::{bit, res, set},
        jump_handlers::{call, jp},
        load16_handlers::{ld as ld16, pop, push},
        load8_handlers::{ld as ld8, ldh},
        misc_handlers::{di, nop},
        opcodes::{BitOp, JumpOp, Load16Op, Load8Op, MiscOp},
        Operation,
    },
    registers::{Reg8, Registers},
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

    fn add_cycles(&mut self, n_cycles: Cycles) {
        self.cycles += n_cycles as u64;
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.add_cycles(Cycles::N4);
        byte
    }

    pub fn read_byte_bus(&mut self, addr: u16) -> u8 {
        let byte = self.bus.read(addr);
        self.add_cycles(Cycles::N4);
        byte
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.bus.write(addr, byte);
        self.add_cycles(Cycles::N4);
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        let (hi, lo) = word_to_bytes(word);
        self.write_byte(addr, lo);
        self.write_byte(addr + 1, hi);
    }

    fn execute(&mut self) {
        print!("PC: {:#06X}", self.registers.pc,);

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

        println!(" => Inst: {:?} {:#06X}", inst, opcode);

        self.execute_inst(inst);
    }

    fn execute_inst(&mut self, inst: Operation) {
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
                JumpOp::CALL(f) => call(self, f),
                JumpOp::RST(_) => todo!(),
            },
        }
    }
}
