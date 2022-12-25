use crate::{alu16, alu8, bit, jump, load16, load8, misc};


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
    Jump(JumpOp),
}

impl Operation {
    pub fn get_operation(opcode: u8, prefixed: bool) -> Option<Operation> {
        let opcode = Self::construct_opcode(opcode, prefixed);

        // Opcode [Destination] [Src]
        match opcode {
            0xCB00 => ,
            0xCB01 => ,
            0xCB02 => ,
            0xCB03 => ,
            0xCB04 => ,
            0xCB05 => ,
            0xCB06 => ,
            0xCB07 => ,
            0xCB08 => ,
            0xCB09 => ,
            0xCB0A => ,
            0xCB0B => ,
            0xCB0C => ,
            0xCB0D => ,
            0xCB0E => ,
            0xCB0F => ,

            0xCB10 => ,
            0xCB11 => ,
            0xCB12 => ,
            0xCB13 => ,
            0xCB14 => ,
            0xCB15 => ,
            0xCB16 => ,
            0xCB17 => ,
            0xCB18 => ,
            0xCB19 => ,
            0xCB1A => ,
            0xCB1B => ,
            0xCB1C => ,
            0xCB1D => ,
            0xCB1E => ,
            0xCB1F => ,

            0xCB20 => ,
            0xCB21 => ,
            0xCB22 => ,
            0xCB23 => ,
            0xCB24 => ,
            0xCB25 => ,
            0xCB26 => ,
            0xCB27 => ,
            0xCB28 => ,
            0xCB29 => ,
            0xCB2A => ,
            0xCB2B => ,
            0xCB2C => ,
            0xCB2D => ,
            0xCB2E => ,
            0xCB2F => ,

            0xCB30 => ,
            0xCB31 => ,
            0xCB32 => ,
            0xCB33 => ,
            0xCB34 => ,
            0xCB35 => ,
            0xCB36 => ,
            0xCB37 => ,
            0xCB38 => ,
            0xCB39 => ,
            0xCB3A => ,
            0xCB3B => ,
            0xCB3C => ,
            0xCB3D => ,
            0xCB3E => ,
            0xCB3F => ,

            0xCB40 => ,
            0xCB41 => ,
            0xCB42 => ,
            0xCB43 => ,
            0xCB44 => ,
            0xCB45 => ,
            0xCB46 => ,
            0xCB47 => ,
            0xCB48 => ,
            0xCB49 => ,
            0xCB4A => ,
            0xCB4B => ,
            0xCB4C => ,
            0xCB4D => ,
            0xCB4E => ,
            0xCB4F => ,

            0xCB50 => ,
            0xCB51 => ,
            0xCB52 => ,
            0xCB53 => ,
            0xCB54 => ,
            0xCB55 => ,
            0xCB56 => ,
            0xCB57 => ,
            0xCB58 => ,
            0xCB59 => ,
            0xCB5A => ,
            0xCB5B => ,
            0xCB5C => ,
            0xCB5D => ,
            0xCB5E => ,
            0xCB5F => ,

            0xCB60 => ,
            0xCB61 => ,
            0xCB62 => ,
            0xCB63 => ,
            0xCB64 => ,
            0xCB65 => ,
            0xCB66 => ,
            0xCB67 => ,
            0xCB68 => ,
            0xCB69 => ,
            0xCB6A => ,
            0xCB6B => ,
            0xCB6C => ,
            0xCB6D => ,
            0xCB6E => ,
            0xCB6F => ,

            0xCB70 => ,
            0xCB71 => ,
            0xCB72 => ,
            0xCB73 => ,
            0xCB74 => ,
            0xCB75 => ,
            0xCB76 => ,
            0xCB77 => ,
            0xCB78 => ,
            0xCB79 => ,
            0xCB7A => ,
            0xCB7B => ,
            0xCB7C => ,
            0xCB7D => ,
            0xCB7E => ,
            0xCB7F => ,

            0xCB80 => ,
            0xCB81 => ,
            0xCB82 => ,
            0xCB83 => ,
            0xCB84 => ,
            0xCB85 => ,
            0xCB86 => ,
            0xCB87 => ,
            0xCB88 => ,
            0xCB89 => ,
            0xCB8A => ,
            0xCB8B => ,
            0xCB8C => ,
            0xCB8D => ,
            0xCB8E => ,
            0xCB8F => ,

            0xCB90 => ,
            0xCB91 => ,
            0xCB92 => ,
            0xCB93 => ,
            0xCB94 => ,
            0xCB95 => ,
            0xCB96 => ,
            0xCB97 => ,
            0xCB98 => ,
            0xCB99 => ,
            0xCB9A => ,
            0xCB9B => ,
            0xCB9C => ,
            0xCB9D => ,
            0xCB9E => ,
            0xCB9F => ,

            0xCBA0 => ,
            0xCBA1 => ,
            0xCBA2 => ,
            0xCBA3 => ,
            0xCBA4 => ,
            0xCBA5 => ,
            0xCBA6 => ,
            0xCBA7 => ,
            0xCBA8 => ,
            0xCBA9 => ,
            0xCBAA => ,
            0xCBAB => ,
            0xCBAC => ,
            0xCBAD => ,
            0xCBAE => ,
            0xCBAF => ,

            0xCBB0 => ,
            0xCBB1 => ,
            0xCBB2 => ,
            0xCBB3 => ,
            0xCBB4 => ,
            0xCBB5 => ,
            0xCBB6 => ,
            0xCBB7 => ,
            0xCBB8 => ,
            0xCBB9 => ,
            0xCBBA => ,
            0xCBBB => ,
            0xCBBC => ,
            0xCBBD => ,
            0xCBBE => ,
            0xCBBF => ,

            0xCBC0 => ,
            0xCBC1 => ,
            0xCBC2 => ,
            0xCBC3 => ,
            0xCBC4 => ,
            0xCBC5 => ,
            0xCBC6 => ,
            0xCBC7 => ,
            0xCBC8 => ,
            0xCBC9 => ,
            0xCBCA => ,
            0xCBCB => ,
            0xCBCC => ,
            0xCBCD => ,
            0xCBCE => ,
            0xCBCF => ,

            0xCBD0 => ,
            0xCBD1 => ,
            0xCBD2 => ,
            0xCBD3 => ,
            0xCBD4 => ,
            0xCBD5 => ,
            0xCBD6 => ,
            0xCBD7 => ,
            0xCBD8 => ,
            0xCBD9 => ,
            0xCBDA => ,
            0xCBDB => ,
            0xCBDC => ,
            0xCBDD => ,
            0xCBDE => ,
            0xCBDF => ,

            0xCBE0 => ,
            0xCBE1 => ,
            0xCBE2 => ,
            0xCBE3 => ,
            0xCBE4 => ,
            0xCBE5 => ,
            0xCBE6 => ,
            0xCBE7 => ,
            0xCBE8 => ,
            0xCBE9 => ,
            0xCBEA => ,
            0xCBEB => ,
            0xCBEC => ,
            0xCBED => ,
            0xCBEE => ,
            0xCBEF => ,

            0xCBF0 => ,
            0xCBF1 => ,
            0xCBF2 => ,
            0xCBF3 => ,
            0xCBF4 => ,
            0xCBF5 => ,
            0xCBF6 => ,
            0xCBF7 => ,
            0xCBF8 => ,
            0xCBF9 => ,
            0xCBFA => ,
            0xCBFB => ,
            0xCBFC => ,
            0xCBFD => ,
            0xCBFE => ,
            0xCBFF => ,

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
