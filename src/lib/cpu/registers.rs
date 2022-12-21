use std::fmt;

use crate::utils::bytes_to_word;

use self::flags::Flags;

mod flags;

pub struct Registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x00,
            f: Flags::new(),
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000, // TODO!: Change this later to correct value
            sp: 0x0000,
        }
    }

    fn get_reg_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::AF => bytes_to_word(self.a, self.f.into()),
            RegisterPair::BC => bytes_to_word(self.b, self.c),
            RegisterPair::DE => bytes_to_word(self.d, self.e),
            RegisterPair::HL => bytes_to_word(self.h, self.l),
            RegisterPair::SP => self.sp,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"
Register Data
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
