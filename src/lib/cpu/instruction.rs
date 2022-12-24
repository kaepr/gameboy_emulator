use std::fmt;

use crate::bus::Bus;

use self::operations::Operation;

use super::CPU;

mod operations;

pub struct Instruction {
    inst_name: String,
    n_cycles: usize,
    n_bytes: usize,
    handler: fn(cpu: &mut CPU, bus: &mut Bus),
}

impl Instruction {
    pub fn get_inst(op: Operation) -> Self {
        todo!()
    }
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
