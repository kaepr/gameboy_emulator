use crate::bus::Bus;

use self::operations::Operation;

use super::CPU;

mod operations;


///
/// Below link used as a reference for constructing enums
/// https://rgbds.gbdev.io/docs/v0.6.0/gbz80.7/
///

pub struct Instruction {
    inst_name: String,
    n_cycles: usize,
    n_bytes: usize,
    handler: fn(cpu: &mut CPU, bus: &mut Bus),
}

impl Instruction {
    pub fn get_inst(op: Operation) -> String {
        todo!()
    }
}
