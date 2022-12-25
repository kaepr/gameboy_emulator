use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Flags {
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BIT_POS: u8 = 7;
const SUB_FLAG_BIT_POS: u8 = 6;
const HALF_CARRY_BIT_POS: u8 = 5;
const CARRY_BIT_POS: u8 = 4;

pub enum FlagType {
    Zero,
    Sub,
    HalfCarry,
    Carry,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            zero: true,
            sub: false,
            half_carry: true,
            carry: true,
        }
    }

    pub fn reset_flag(&mut self, flag_type: FlagType) {
        match flag_type {
            FlagType::Zero => self.zero = false,
            FlagType::Sub => self.sub = false,
            FlagType::HalfCarry => self.half_carry = false,
            FlagType::Carry => self.carry = false,
        }
    }

    pub fn set_flag(&mut self, flag_type: FlagType) {
        match flag_type {
            FlagType::Zero => self.zero = true,
            FlagType::Sub => self.sub = true,
            FlagType::HalfCarry => self.half_carry = true,
            FlagType::Carry => self.carry = true,
        }
    }
}

fn get_bool_val(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

impl From<Flags> for u8 {
    fn from(flag: Flags) -> u8 {
        (get_bool_val(flag.zero) << ZERO_FLAG_BIT_POS)
            | (get_bool_val(flag.sub) << SUB_FLAG_BIT_POS)
            | (get_bool_val(flag.half_carry) << HALF_CARRY_BIT_POS)
            | (get_bool_val(flag.carry) << CARRY_BIT_POS)
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Flags {
        let zero = (byte & (1 << ZERO_FLAG_BIT_POS)) > 0;
        let sub = (byte & (1 << SUB_FLAG_BIT_POS)) > 0;
        let half_carry = (byte & (1 << HALF_CARRY_BIT_POS)) > 0;
        let carry = (byte & (1 << CARRY_BIT_POS)) > 0;

        Flags {
            zero,

            sub,
            half_carry,
            carry,
        }
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"
zero       : {}
sub        : {}
half_carry : {}
carry      : {}"#,
            self.zero, self.sub, self.half_carry, self.carry
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{FlagType, Flags};

    #[test]
    fn test_from_trait_impl() {
        let f = Flags {
            zero: true,
            sub: false,
            half_carry: true,
            carry: true,
        };

        let res: u8 = f.into();
        let res_flag: Flags = res.into();

        assert_eq!(res, 0b10110000);
        assert_eq!(res_flag, f);
    }

    #[test]
    fn test_set_flag() {
        let mut f = Flags {
            zero: true,
            sub: false,
            half_carry: true,
            carry: true,
        };

        f.reset_flag(FlagType::Zero);
        f.set_flag(FlagType::Sub);
        f.set_flag(FlagType::HalfCarry);
        f.set_flag(FlagType::Carry);

        let res: u8 = f.into();
        assert_eq!(res, 0b01110000);
    }
}
