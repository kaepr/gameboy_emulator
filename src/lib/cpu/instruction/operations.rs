use crate::{alu16, alu8, bit, load16, load8, misc};

use self::opcodes::*;

///
/// Below link used as a reference for constructing enums
/// https://gbdev.io/gb-opcodes/optables/
///

const PREFIX_INST: u8 = 0xCB;

mod opcode_macros;
mod opcodes;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Misc(MiscOp),
    Load8(Load8Op),
    Load16(Load16Op),
    ALU16(ALU16Op),
    ALU8(ALU8Op),
    BIT(BitOp),
}

impl Operation {
    pub fn get_operation(opcode: u8, prefixed: bool) -> Option<Operation> {
        let opcode = Self::construct_opcode(opcode, prefixed);

        // Opcode [Destination] [Src]
        match opcode {
            0x0000 => misc!(NOP),
            0x0001 => load16!(LD, BC, Direct16Bit),
            0x0002 => load8!(LD, BC, A),
            0x0003 => alu16!(INC, BC, NIL),
            0x0004 => alu8!(INC, B, NIL),
            0x0005 => alu8!(DEC, B, NIL),
            0x0006 => load8!(LD, B, Direct8Bit),
            0x0007 => bit!(RLCA),
            0x0008 => load16!(LD, Addr16Bit, SP),
            0x0009 => alu16!(ADD, HL, BC),
            0x000A => load8!(LD, A, BC),
            0x000B => alu16!(DEC, BC, NIL),
            0x000C => alu8!(INC, C, NIL),
            0x000D => alu8!(DEC, C, NIL),
            0x000E => load8!(LD, C, Direct8Bit),
            0x000F => bit!(RRCA),
            _ => None,
        }
    }

    fn is_prefix(opcode: u8) -> bool {
        opcode == PREFIX_INST
    }

    fn construct_opcode(opcode: u8, prefixed: bool) -> u16 {
        if prefixed {
            0xCB00 | (opcode as u16)
        } else {
            opcode as u16
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instruction::operations::opcodes::{Load16Dest, Load16Src};

    use super::{opcodes::Load16Op, Operation};

    #[test]
    fn test_construct_opcode() {
        let op = 0x12;
        let res_1 = Operation::construct_opcode(op, false);
        let res_2 = Operation::construct_opcode(op, true);

        assert_eq!(res_1, 0x0012);
        assert_eq!(res_2, 0xCB12);

        let op = 0xcb;
        let res_1 = Operation::construct_opcode(op, false);
        let res_2 = Operation::construct_opcode(op, true);

        assert_eq!(res_1, 0x00cb);
        assert_eq!(res_2, 0xCBcb);
    }

    #[test]
    fn test_macros() {
        let op = Operation::get_operation(0x01, false).unwrap();

        assert_eq!(
            op,
            Operation::Load16(Load16Op::LD(Load16Dest::BC, Load16Src::Direct16Bit))
        );
    }
}
