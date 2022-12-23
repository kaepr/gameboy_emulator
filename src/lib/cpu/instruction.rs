use super::CPU;

pub struct Instruction {
    inst_name: String,
    n_cycles: usize,
    n_bytes: usize,
    handler: fn(cpu: &mut CPU)
}


