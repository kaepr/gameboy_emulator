use crate::bus::Memory;

use self::header::CartridgeHeader;

mod header;

pub struct Cartridge {
    pub header: CartridgeHeader,
    pub data: Vec<u8>,
    pub bank0: Vec<u8>,
    pub bankn: Vec<u8>,
}

impl Memory for Cartridge {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.bank0[address as usize],
            0x4000..=0x7FFF => self.bankn[(address - 0x4000) as usize],
            _ => panic!("invalid cart memory accessed"),
        }
    }

    fn write(&mut self, _address: u16, _byte: u8) {}
}

impl Cartridge {
    pub const BANK_N_START: u16 = 0x4000;

    pub fn new(data: Vec<u8>) -> Self {
        Cartridge {
            header: CartridgeHeader::new(&data[0x0100..=0x14F0]),
            bankn: data[0x4000..=0x7FFF].to_vec(),
            bank0: data[0x0000..=0x3FFF].to_vec(),
            data,
        }
    }
}
