#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MiscOp {
    NOP,
}

// Load 8 =================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load8Dest {
    BC,
    B,
    A,
    C,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load8Src {
    A,
    Direct8Bit,
    BC,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load8Op {
    LD(Load8Dest, Load8Src),
}

// Load 16 =================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load16Dest {
    BC,
    Addr16Bit,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load16Src {
    Direct16Bit, // Immediate little endian 16-bit data
    SP,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Load16Op {
    LD(Load16Dest, Load16Src),
}

// ALU 16 ===================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU16Dest {
    BC,
    HL,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU16Src {
    NIL,
    BC,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU16Op {
    INC(ALU16Dest, ALU16Src),
    ADD(ALU16Dest, ALU16Src),
    DEC(ALU16Dest, ALU16Src),
}

// ALU 8 ======================
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU8Dest {
    B,
    C,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU8Src {
    NIL,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ALU8Op {
    INC(ALU8Dest, ALU8Src),
    DEC(ALU8Dest, ALU8Src),
}

// BIT
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BitOp {
    RLCA,
    RRCA,
}
