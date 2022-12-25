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
            0x0000 => misc!(NOP),
            0x0001 => load16!(LD, BC, Direct16Bit),
            0x0002 => load8!(LD, BC, A),
            0x0003 => alu16!(INC, BC),
            0x0004 => alu8!(INC, B),
            0x0005 => alu8!(DEC, B),
            0x0006 => load8!(LD, B, Direct8Bit),
            0x0007 => bit!(RLCA),
            0x0008 => load16!(LD, Addr16Bit, SP),
            0x0009 => alu16!(ADD, HL, BC),
            0x000A => load8!(LD, A, BC),
            0x000B => alu16!(DEC, BC),
            0x000C => alu8!(INC, C),
            0x000D => alu8!(DEC, C),
            0x000E => load8!(LD, C, Direct8Bit),
            0x000F => bit!(RRCA),

            0x0010 => misc!(STOP),
            0x0011 => load16!(LD, DE, Direct16Bit),
            0x0012 => load8!(LD, DE, A),
            0x0013 => alu16!(INC, DE),
            0x0014 => alu8!(INC, D),
            0x0015 => alu8!(DEC, D),
            0x0016 => load8!(LD, D, Direct8Bit),
            0x0017 => bit!(RLA),
            0x0018 => jump!(JR, NIL),
            0x0019 => alu16!(ADD, HL, DE),
            0x001A => load8!(LD, A, DE),
            0x001B => alu16!(DEC, DE),
            0x001C => alu8!(INC, E),
            0x001D => alu8!(DEC, E),
            0x001E => load8!(LD, E, Direct8Bit),
            0x001F => bit!(RRA),

            0x0020 => jump!(JR, NZ),
            0x0021 => load16!(LD, HL, Direct16Bit),
            0x0022 => load8!(LD, HLI, A),
            0x0023 => alu16!(INC, HL),
            0x0024 => alu8!(INC, H),
            0x0025 => alu8!(DEC, H),
            0x0026 => load8!(LD, H, Direct8Bit),
            0x0027 => alu8!(DAA),
            0x0028 => jump!(JR, Z),
            0x0029 => alu16!(ADD, HL, HL),
            0x002A => load8!(LD, A, HLI),
            0x002B => alu16!(DEC, HL),
            0x002C => alu8!(INC, L),
            0x002D => alu8!(DEC, L),
            0x002E => load8!(LD, L, Direct8Bit),
            0x002F => alu8!(CPL),

            0x0030 => jump!(JR, NC),
            0x0031 => load16!(LD, SP, Direct16Bit),
            0x0032 => load8!(LD, HLD, A),
            0x0033 => alu16!(INC, SP),
            0x0034 => alu8!(INC, HL),
            0x0035 => alu8!(DEC, HL),
            0x0036 => load8!(LD, HL, Direct8Bit),
            0x0037 => alu8!(SCF),
            0x0038 => jump!(JR, C),
            0x0039 => alu16!(ADD, HL, SP),
            0x003A => load8!(LD, A, HLD),
            0x003B => alu16!(DEC, SP),
            0x003C => alu8!(INC, A),
            0x003D => alu8!(DEC, A),
            0x003E => load8!(LD, A, Direct8Bit),
            0x003F => alu8!(CCF),

            0x0040 => load8!(LD, B, B),
            0x0041 => load8!(LD, B, C),
            0x0042 => load8!(LD, B, D),
            0x0043 => load8!(LD, B, E),
            0x0044 => load8!(LD, B, H),
            0x0045 => load8!(LD, B, L),
            0x0046 => load8!(LD, B, HL),
            0x0047 => load8!(LD, B, A),
            0x0048 => load8!(LD, C, B),
            0x0049 => load8!(LD, C, C),
            0x004A => load8!(LD, C, D),
            0x004B => load8!(LD, C, E),
            0x004C => load8!(LD, C, H),
            0x004D => load8!(LD, C, L),
            0x004E => load8!(LD, C, HL),
            0x004F => load8!(LD, C, A),

            0x0050 => load8!(LD, D, B),
            0x0051 => load8!(LD, D, C),
            0x0052 => load8!(LD, D, D),
            0x0053 => load8!(LD, D, E),
            0x0054 => load8!(LD, D, H),
            0x0055 => load8!(LD, D, L),
            0x0056 => load8!(LD, D, HL),
            0x0057 => load8!(LD, D, A),
            0x0058 => load8!(LD, E, B),
            0x0059 => load8!(LD, E, C),
            0x005A => load8!(LD, E, D),
            0x005B => load8!(LD, E, E),
            0x005C => load8!(LD, E, H),
            0x005D => load8!(LD, E, L),
            0x005E => load8!(LD, E, HL),
            0x005F => load8!(LD, E, A),

            0x0060 => load8!(LD, H, B),
            0x0061 => load8!(LD, H, C),
            0x0062 => load8!(LD, H, D),
            0x0063 => load8!(LD, H, E),
            0x0064 => load8!(LD, H, H),
            0x0065 => load8!(LD, H, L),
            0x0066 => load8!(LD, H, HL),
            0x0067 => load8!(LD, H, A),
            0x0068 => load8!(LD, L, B),
            0x0069 => load8!(LD, L, C),
            0x006A => load8!(LD, L, D),
            0x006B => load8!(LD, L, E),
            0x006C => load8!(LD, L, H),
            0x006D => load8!(LD, L, L),
            0x006E => load8!(LD, L, HL),
            0x006F => load8!(LD, L, A),

            0x0070 => load8!(LD, HL, B),
            0x0071 => load8!(LD, HL, C),
            0x0072 => load8!(LD, HL, D),
            0x0073 => load8!(LD, HL, E),
            0x0074 => load8!(LD, HL, H),
            0x0075 => load8!(LD, HL, L),
            0x0076 => misc!(HALT),
            0x0077 => load8!(LD, HL, A),
            0x0078 => load8!(LD, A, B),
            0x0079 => load8!(LD, A, C),
            0x007A => load8!(LD, A, D),
            0x007B => load8!(LD, A, E),
            0x007C => load8!(LD, A, H),
            0x007D => load8!(LD, A, L),
            0x007E => load8!(LD, A, HL),
            0x007F => load8!(LD, A, A),

            0x0080 => alu8!(ADD, A, B),
            0x0081 => alu8!(ADD, A, C),
            0x0082 => alu8!(ADD, A, D),
            0x0083 => alu8!(ADD, A, E),
            0x0084 => alu8!(ADD, A, H),
            0x0085 => alu8!(ADD, A, L),
            0x0086 => alu8!(ADD, A, HL),
            0x0087 => alu8!(ADD, A, A),
            0x0088 => alu8!(ADC, A, B),
            0x0089 => alu8!(ADC, A, C),
            0x008A => alu8!(ADC, A, D),
            0x008B => alu8!(ADC, A, E),
            0x008C => alu8!(ADC, A, H),
            0x008D => alu8!(ADC, A, L),
            0x008E => alu8!(ADC, A, HL),
            0x008F => alu8!(ADC, A, A),

            0x0090 => alu8!(SUB, B),
            0x0091 => alu8!(SUB, C),
            0x0092 => alu8!(SUB, D),
            0x0093 => alu8!(SUB, E),
            0x0094 => alu8!(SUB, H),
            0x0095 => alu8!(SUB, L),
            0x0096 => alu8!(SUB, HL),
            0x0097 => alu8!(SUB, A),
            0x0098 => alu8!(SBC, A, B),
            0x0099 => alu8!(SBC, A, C),
            0x009A => alu8!(SBC, A, D),
            0x009B => alu8!(SBC, A, E),
            0x009C => alu8!(SBC, A, H),
            0x009D => alu8!(SBC, A, L),
            0x009E => alu8!(SBC, A, HL),
            0x009F => alu8!(SBC, A, A),

            0x00A0 => alu8!(AND, B),
            0x00A1 => alu8!(AND, C),
            0x00A2 => alu8!(AND, D),
            0x00A3 => alu8!(AND, E),
            0x00A4 => alu8!(AND, H),
            0x00A5 => alu8!(AND, L),
            0x00A6 => alu8!(AND, HL),
            0x00A7 => alu8!(AND, A),
            0x00A8 => alu8!(XOR, B),
            0x00A9 => alu8!(XOR, C),
            0x00AA => alu8!(XOR, D),
            0x00AB => alu8!(XOR, E),
            0x00AC => alu8!(XOR, H),
            0x00AD => alu8!(XOR, L),
            0x00AE => alu8!(XOR, HL),
            0x00AF => alu8!(XOR, A),

            0x00B0 => alu8!(OR, B),
            0x00B1 => alu8!(OR, C),
            0x00B2 => alu8!(OR, D),
            0x00B3 => alu8!(OR, E),
            0x00B4 => alu8!(OR, H),
            0x00B5 => alu8!(OR, L),
            0x00B6 => alu8!(OR, HL),
            0x00B7 => alu8!(OR, A),
            0x00B8 => alu8!(CP, B),
            0x00B9 => alu8!(CP, C),
            0x00BA => alu8!(CP, D),
            0x00BB => alu8!(CP, E),
            0x00BC => alu8!(CP, H),
            0x00BD => alu8!(CP, L),
            0x00BE => alu8!(CP, HL),
            0x00BF => alu8!(CP, A),

            0x00C0 => jump!(RET, NZ),
            0x00C1 => load16!(POP, BC),
            0x00C2 => jump!(JP, NZ),
            0x00C3 => jump!(JP, NIL),
            0x00C4 => jump!(CALL, NZ),
            0x00C5 => load16!(PUSH, BC),
            0x00C6 => alu8!(ADD, A, Direct8Bit),
            0x00C7 => jump!(RST, X00),
            0x00C8 => jump!(RET, Z),
            0x00C9 => jump!(RET, NIL),
            0x00CA => jump!(JP, Z),
            0x00CB => misc!(PREFIX),
            0x00CC => jump!(CALL, Z),
            0x00CD => jump!(CALL, NIL),
            0x00CE => alu8!(ADC, A, Direct8Bit),
            0x00CF => jump!(RST, X08),

            0x00D0 => jump!(RET, NC),
            0x00D1 => load16!(POP, DE),
            0x00D2 => jump!(JP, NC),
            0x00D3 => None,
            0x00D4 => jump!(CALL, NC),
            0x00D5 => load16!(PUSH, DE),
            0x00D6 => alu8!(SUB, Direct8Bit),
            0x00D7 => jump!(RST, X10),
            0x00D8 => jump!(RET, C),
            0x00D9 => jump!(RETI),
            0x00DA => jump!(JP, C),
            0x00DB => None,
            0x00DC => jump!(CALL, C),
            0x00DD => None,
            0x00DE => alu8!(SBC, A, Direct8Bit),
            0x00DF => jump!(RST, X18),

            0x00E0 => load8!(LDH, Unsigned8, A),
            0x00E1 => load16!(POP, HL),
            0x00E2 => load8!(LD, AddrC, A),
            0x00E3 => None,
            0x00E4 => None,
            0x00E5 => load16!(PUSH, HL),
            0x00E6 => alu8!(AND, Direct8Bit),
            0x00E7 => jump!(RST, X20),
            0x00E8 => alu16!(ADD, SP, Signed8),
            0x00E9 => jump!(JPToHL),
            0x00EA => load8!(LD, Addr16Bit, A),
            0x00EB => None,
            0x00EC => None,
            0x00ED => None,
            0x00EE => alu8!(XOR, Direct8Bit),
            0x00EF => jump!(RST, X28),

            0x00F0 => load8!(LDH, A, Unsigned8),
            0x00F1 => load16!(POP, AF),
            0x00F2 => load8!(LD, A, AddrC),
            0x00F3 => misc!(DI),
            0x00F4 => None,
            0x00F5 => load16!(PUSH, AF),
            0x00F6 => alu8!(OR, Direct8Bit),
            0x00F7 => jump!(RST, X30),
            0x00F8 => load16!(LD, HL, SPr8),
            0x00F9 => load16!(LD, SP, HL),
            0x00FA => load8!(LD, A, Addr16Bit),
            0x00FB => misc!(EI),
            0x00FC => None,
            0x00FD => None,
            0x00FE => alu8!(CP, Direct8Bit),
            0x00FF => jump!(RST, X38),

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
