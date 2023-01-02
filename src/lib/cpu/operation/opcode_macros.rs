#[macro_export]
macro_rules! misc {
    ($op: ident) => {
        Some(Operation::Misc(MiscOp::$op))
    };
}

#[macro_export]
macro_rules! load16 {
    ($op: ident, $dest: ident) => {
        Some(Operation::Load16(Load16Op::$op(Load16Dest::$dest)))
    };
    ($op: ident, $dest: ident, $src: ident) => {
        Some(Operation::Load16(Load16Op::$op(
            Load16Dest::$dest,
            Load16Src::$src,
        )))
    };
}

#[macro_export]
macro_rules! load8 {
    ($op: ident, $dest: ident, $src: ident) => {
        Some(Operation::Load8(Load8Op::$op(
            Load8Dest::$dest,
            Load8Src::$src,
        )))
    };
}

#[macro_export]
macro_rules! alu16 {
    ($op: ident, $dest: ident) => {
        Some(Operation::ALU16(ALU16Op::$op(ALU16Dest::$dest)))
    };
    ($op: ident, $dest: ident, $src: ident) => {
        Some(Operation::ALU16(ALU16Op::$op(
            ALU16Dest::$dest,
            ALU16Src::$src,
        )))
    };
}

#[macro_export]
macro_rules! alu8 {
    ($op: ident) => {
        Some(Operation::ALU8(ALU8Op::$op))
    };
    ($op: ident, $dest: ident) => {
        Some(Operation::ALU8(ALU8Op::$op(ALU8Dest::$dest)))
    };
    ($op: ident, $dest: ident, $src: ident) => {
        Some(Operation::ALU8(ALU8Op::$op(ALU8Dest::$dest, ALU8Src::$src)))
    };
}

#[macro_export]
macro_rules! bit {
    ($op: ident) => {
        Some(Operation::Bit(BitOp::$op))
    };
    ($op: ident, $dest: ident) => {
        Some(Operation::Bit(BitOp::$op(BitDest::$dest)))
    };
    ($op: ident, $pos: ident, $dest: ident) => {
        Some(Operation::Bit(BitOp::$op(BitPos::$pos, BitDest::$dest)))
    };
}

#[macro_export]
macro_rules! jump {
    ($op: ident) => {
        Some(Operation::Jump(JumpOp::$op))
    };
    (RST, $rst_target: ident) => {
        Some(Operation::Jump(JumpOp::RST(RSTTarget::$rst_target)))
    };
    ($op: ident, $jump_condition: ident) => {
        Some(Operation::Jump(JumpOp::$op(JumpCondition::$jump_condition)))
    };
}
