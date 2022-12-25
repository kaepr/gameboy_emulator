use std::fmt;

use crate::bus::Bus;

use super::CPU;

pub mod handlers;

pub struct Instruction {
    inst_name: String,
    n_cycles: usize, // writing cycles in T state form, thus multiple of 4
    n_bytes: usize,
    handler: fn(cpu: &mut CPU, bus: &mut Bus),
}

impl Instruction {
    //  pub fn get_inst(op: Operation) -> Self {
    //      match op {
    //          Operation::Misc(m) => match m {
    //              MiscOp::NOP => Instruction {
    //                  inst_name: "NOP".into(),
    //                  n_cycles: 4,
    //                  n_bytes: 1,
    //                  handler: |cpu, bus| {},
    //              },
    //              MiscOp::STOP => todo!(),
    //              MiscOp::HALT => todo!(),
    //              MiscOp::PREFIX => Instruction {
    //                  inst_name: "PREFIX".into(),
    //                  n_cycles: 4,
    //                  n_bytes: 1,
    //                  handler: |cpu, bus| {},
    //              },
    //              MiscOp::EI => todo!(),
    //              MiscOp::DI => todo!(),
    //          },
    //          Operation::Load8(_) => todo!(),
    //          Operation::Load16(_) => todo!(),
    //          Operation::ALU16(_) => todo!(),
    //          Operation::ALU8(_) => todo!(),
    //          Operation::Bit(o) => match o {
    //              BitOp::RLCA => todo!(),
    //              BitOp::RRCA => todo!(),
    //              BitOp::RLA => todo!(),
    //              BitOp::RRA => todo!(),
    //              BitOp::RLC(_) => todo!(),
    //              BitOp::RRC(_) => todo!(),
    //              BitOp::RL(_) => todo!(),
    //              BitOp::RR(_) => todo!(),
    //              BitOp::SLA(_) => todo!(),
    //              BitOp::SRA(_) => todo!(),
    //              BitOp::SWAP(_) => todo!(),
    //              BitOp::SRL(_) => todo!(),
    //              BitOp::BIT(pos, dest) => todo!(),
    //              BitOp::RES(pos, dest) => res_bit(pos, dest),
    //              BitOp::SET(pos, dest) => set_bit(pos, dest),
    //          },
    //          Operation::Jump(_) => todo!(),
    //          _ => panic!("Unknown / Unimplemented function called {:?}!", op)
    //      }
    //  }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"Inst: {}, n_cycles: {}, n_bytes: {}"#,
            self.inst_name, self.n_cycles, self.n_bytes
        )
    }
}
