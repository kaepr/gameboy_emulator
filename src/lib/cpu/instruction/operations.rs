///
/// Below link used as a reference for constructing enums
/// https://gbdev.io/gb-opcodes/optables/
///

const PREFIX_INST: u8 = 0xCB;

pub enum MiscOp {
    NOP,
}

// Load 8 =================
pub enum Load8Dest {
    BC,
    B,
    A,
    C,
}

pub enum Load8Source {
    A,
    Direct8Bit,
    BC,
}

pub enum Load8Op {
    LD(Load8Dest, Load8Source),
}

// Load 16 =================
pub enum Load16Dest {
    BC,
    Addr16Bit,
}

pub enum Load16Source {
    Direct16Bit, // Immediate little endian 16-bit data
    SP,
}

pub enum Load16Op {
    LD(Load16Dest, Load16Source),
}

// ALU 16 ===================
pub enum ALU16Dest {
    BC,
    HL,
}

pub enum ALU16Source {
    NIL,
    BC,
}

pub enum ALU16Op {
    INC(ALU16Dest, ALU16Source),
    ADD(ALU16Dest, ALU16Source),
    DEC(ALU16Dest, ALU16Source),
}

// ALU 8 ======================
pub enum ALU8Dest {
    B,
    C,
}

pub enum ALU8Source {
    NIL,
}

pub enum ALU8Op {
    INC(ALU8Dest, ALU8Source),
    DEC(ALU8Dest, ALU8Source),
}

// BIT
pub enum BitOp {
    RLCA,
    RRCA,
}

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

        // Opcode [Destination] [Source]
        match opcode {
            0x0000 => Some(Operation::Misc(MiscOp::NOP)),
            0x0001 => Some(Operation::Load16(Load16Op::LD(
                Load16Dest::BC,
                Load16Source::Direct16Bit,
            ))),
            0x0002 => Some(Operation::Load8(Load8Op::LD(Load8Dest::BC, Load8Source::A))),
            0x0003 => Some(Operation::ALU16(ALU16Op::INC(
                ALU16Dest::BC,
                ALU16Source::NIL,
            ))),
            0x0004 => Some(Operation::ALU8(ALU8Op::INC(ALU8Dest::B, ALU8Source::NIL))),
            0x0005 => Some(Operation::ALU8(ALU8Op::DEC(ALU8Dest::B, ALU8Source::NIL))),
            0x0006 => Some(Operation::Load8(Load8Op::LD(
                Load8Dest::B,
                Load8Source::Direct8Bit,
            ))),
            0x0007 => Some(Operation::BIT(BitOp::RLCA)),
            0x0008 => Some(Operation::Load16(Load16Op::LD(
                Load16Dest::Addr16Bit,
                Load16Source::SP,
            ))),
            0x0009 => Some(Operation::ALU16(ALU16Op::ADD(
                ALU16Dest::HL,
                ALU16Source::BC,
            ))),
            0x000A => Some(Operation::Load8(Load8Op::LD(Load8Dest::A, Load8Source::BC))),
            0x000B => Some(Operation::ALU16(ALU16Op::DEC(
                ALU16Dest::BC,
                ALU16Source::NIL,
            ))),
            0x000C => Some(Operation::ALU8(ALU8Op::INC(ALU8Dest::C, ALU8Source::NIL))),
            0x000D => Some(Operation::ALU8(ALU8Op::DEC(ALU8Dest::C, ALU8Source::NIL))),
            0x000E => Some(Operation::Load8(Load8Op::LD(
                Load8Dest::C,
                Load8Source::Direct8Bit,
            ))),
            0x000F => Some(Operation::BIT(BitOp::RRCA)),
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
    use super::Operation;

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
}
