use core::fmt;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MiscOp {
    NOP,
    STOP,
    HALT,
    PREFIX,
    EI,
    DI,
}

// Load 8 =================
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Load8Dest {
    AddrC,
    Unsigned8,
    Addr16Bit,
    BC,
    B,
    A,
    C,
    DE,
    D,
    E,
    HLI,
    H,
    L,
    HLD,
    HL,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Load8Src {
    Unsigned8,
    AddrC,
    Addr16Bit,
    A,
    Direct8Bit,
    BC,
    DE,
    HLI,
    HLD,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load8Op {
    LD(Load8Dest, Load8Src),
    LDH(Load8Dest, Load8Src),
}

// Load 16 =================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load16Dest {
    BC,
    Addr16Bit,
    DE,
    HL,
    SP,
    AF,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load16Src {
    Direct16Bit, // Immediate little endian 16-bit data
    SP,
    HL,
    SPr8,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load16Op {
    LD(Load16Dest, Load16Src),
    POP(Load16Dest),
    PUSH(Load16Dest),
}

// ALU 16 ===================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU16Dest {
    BC,
    HL,
    DE,
    SP,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU16Src {
    Signed8,
    NIL,
    BC,
    DE,
    HL,
    SP,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU16Op {
    INC(ALU16Dest),
    ADD(ALU16Dest, ALU16Src),
    DEC(ALU16Dest),
}

// ALU 8 ======================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU8Dest {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
    Direct8Bit,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU8Src {
    NIL,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
    Direct8Bit,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU8Op {
    DAA,
    CPL,
    SCF,
    CCF,
    INC(ALU8Dest),
    DEC(ALU8Dest),
    SUB(ALU8Dest),
    AND(ALU8Dest),
    XOR(ALU8Dest),
    OR(ALU8Dest),
    CP(ALU8Dest),
    ADD(ALU8Dest, ALU8Src),
    ADC(ALU8Dest, ALU8Src),
    SBC(ALU8Dest, ALU8Src),
}

// BIT
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BitDest {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BitPos {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl From<BitPos> for u8 {
    fn from(pos: BitPos) -> u8 {
        match pos {
            BitPos::B0 => 0,
            BitPos::B1 => 1,
            BitPos::B2 => 2,
            BitPos::B3 => 3,
            BitPos::B4 => 4,
            BitPos::B5 => 5,
            BitPos::B6 => 6,
            BitPos::B7 => 7,
        }
    }
}

impl Display for BitDest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BitDest::HL => write!(f, "(HL)"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BitOp {
    RLCA,
    RRCA,
    RLA,
    RRA,
    RLC(BitDest),
    RRC(BitDest),
    RL(BitDest),
    RR(BitDest),
    SLA(BitDest),
    SRA(BitDest),
    SWAP(BitDest),
    SRL(BitDest),
    BIT(BitPos, BitDest),
    RES(BitPos, BitDest),
    SET(BitPos, BitDest),
}

// JUMP

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum JumpCondition {
    NIL,
    Z,
    C,
    NZ,
    NC,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RSTTarget {
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum JumpOp {
    RETI,
    JR(JumpCondition),
    JPToHL,
    JP(JumpCondition),
    RET(JumpCondition),
    CALL(JumpCondition),
    RST(RSTTarget),
}
