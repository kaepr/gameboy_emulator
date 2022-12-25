use crate::cpu::registers::Reg16;

use super::Instruction;

// pub fn res_bit(pos: BitPos, dest: BitDest) -> Instruction {
// let bit_pos: u8 = pos.into();
// let n_cycles = if let BitDest::HL = dest { 16 } else { 4 };

// Instruction {
//     inst_name: format!("RES {}, {}", bit_pos, dest),
//     n_cycles,
//     n_bytes: 2,
//     handler: |cpu, bus| {
//         let value = match dest {
//             BitDest::B => cpu.registers.b,
//             BitDest::C => cpu.registers.c,
//             BitDest::D => cpu.registers.d,
//             BitDest::E => cpu.registers.e,
//             BitDest::H => cpu.registers.h,
//             BitDest::L => cpu.registers.l,
//             BitDest::HL => {
//                 let addr = cpu.registers.get_reg_pair(Reg16::HL);
//                 bus.read(addr)
//             }
//             BitDest::A => cpu.registers.a,
//         };

//         let value = value & !(1 << bit_pos);

//         match dest {
//             BitDest::B => cpu.registers.b = value,
//             BitDest::C => cpu.registers.c = value,
//             BitDest::D => cpu.registers.d = value,
//             BitDest::E => cpu.registers.e = value,
//             BitDest::H => cpu.registers.h = value,
//             BitDest::L => cpu.registers.l = value,
//             BitDest::HL => {
//                 let addr = cpu.registers.get_reg_pair(Reg16::HL);
//                 bus.write(addr, value);
//             }
//             BitDest::A => cpu.registers.a = value,
//         }
//     },
// }
// }

// pub fn set_bit(pos: BitPos, dest: BitDest) -> Instruction {
// let bit_pos: u8 = pos.into();
// let n_cycles = if let BitDest::HL = dest { 16 } else { 4 };

// Instruction {
//     inst_name: format!("RES {}, {}", bit_pos, dest),
//     n_cycles,
//     n_bytes: 2,
//     handler: |cpu, bus| {
//         let value = match dest {
//             BitDest::B => cpu.registers.b,
//             BitDest::C => cpu.registers.c,
//             BitDest::D => cpu.registers.d,
//             BitDest::E => cpu.registers.e,
//             BitDest::H => cpu.registers.h,
//             BitDest::L => cpu.registers.l,
//             BitDest::HL => {
//                 let addr = cpu.registers.get_reg_pair(Reg16::HL);
//                 bus.read(addr)
//             }
//             BitDest::A => cpu.registers.a,
//         };

//         let value = value | (1 << bit_pos);

//         match dest {
//             BitDest::B => cpu.registers.b = value,
//             BitDest::C => cpu.registers.c = value,
//             BitDest::D => cpu.registers.d = value,
//             BitDest::E => cpu.registers.e = value,
//             BitDest::H => cpu.registers.h = value,
//             BitDest::L => cpu.registers.l = value,
//             BitDest::HL => {
//                 let addr = cpu.registers.get_reg_pair(Reg16::HL);
//                 bus.write(addr, value);
//             }
//             BitDest::A => cpu.registers.a = value,
//         }
//     },
// }
// }
