

///
/// Below link used as a reference for constructing enums
/// https://gbdev.io/gb-opcodes/optables/
/// 

pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum RegisterPairTarget {
    BC,
    DE,
    HL,
}

pub enum ALU8BitTarget {
    Registers(RegisterTarget),
    ReadByteFromHL,
    DirectByte,
}

pub enum ALU16BitTarget {
    RegisterPair(RegisterPairTarget),
}

pub enum IncDec8BitTarget {
    Registers(RegisterTarget),
    ReadByteFromHL,
}

pub enum MiscOperation {
    CCF,
    CPL,
    DAA,
    DI,
    EI,
    HALT,
    NOP,
    SCF,
    STOP,
}

pub enum ALU8BitOperation {
    ADC(ALU8BitTarget),
    ADD(ALU8BitTarget),
    AND(ALU8BitTarget),
    CP(ALU8BitTarget),
    DEC(IncDec8BitTarget),
    INC(IncDec8BitTarget),
    OR(ALU8BitTarget),
    SBC(ALU8BitTarget),
    SUB(ALU8BitTarget),
    XOR(ALU8BitTarget),
}

pub enum ALU16BitOperation {
    ADD(ALU16BitTarget),
    DEC(ALU16BitTarget),
    INC(ALU16BitTarget),
}

pub enum Operation {
    Misc(MiscOperation),
    ALU8Bit(ALU8BitOperation),
    ALU16Bit(ALU16BitOperation),
    BitOpn,
    BitShiftOpn,
    Load,
    StackOpn,
}

