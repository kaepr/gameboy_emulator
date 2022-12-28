use std::fmt;

use crate::utils::{bytes_to_word, word_to_bytes};

use self::flags::Flags;

pub mod flags;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: Flags::new(),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn get_reg_pair(&self, pair: Reg16) -> u16 {
        match pair {
            Reg16::AF => bytes_to_word(self.a, self.f.into()),
            Reg16::BC => bytes_to_word(self.b, self.c),
            Reg16::DE => bytes_to_word(self.d, self.e),
            Reg16::HL => bytes_to_word(self.h, self.l),
            Reg16::SP => self.sp,
        }
    }

    pub fn get_flags(&self) -> Flags {
        self.f
    }

    pub fn set_reg(&mut self, value: u8, target: Reg8) {
        match target {
            Reg8::A => self.a = value,
            Reg8::F => self.f = value.into(),
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
        }
    }

    pub fn set_reg_pair(&mut self, value: u16, pair: Reg16) {
        let (high, low) = word_to_bytes(value);

        match pair {
            // Used very rarely
            Reg16::AF => {
                self.a = high;
                self.f = low.into();
            }
            Reg16::BC => {
                self.b = high;
                self.c = low;
            }
            Reg16::DE => {
                self.d = high;
                self.e = low;
            }
            Reg16::HL => {
                self.h = high;
                self.l = low;
            }
            Reg16::SP => {
                self.sp = value;
            }
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"
PC: 0x{:04x} SP: 0x{:04x}
A: 0x{:02x} 
{}
B: 0x{:02x} C: 0x{:02x}
D: 0x{:02x} E: 0x{:02x}
H: 0x{:02x} L: 0x{:02x}"#,
            self.pc, self.sp, self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l
        )
    }
}
