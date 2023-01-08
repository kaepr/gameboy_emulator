use crate::bus::Memory;

use self::header::CartridgeHeader;

mod header;

pub struct Cartridge {
    pub header: CartridgeHeader,
    pub data: Vec<u8>,
}

impl Memory for Cartridge {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&self, address: u16, byte: u8) {
        todo!()
    }
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        let header_data = &data[0x0100..=0x014F0];

        Cartridge {
            header: CartridgeHeader::new(header_data),
            data,
        }
    }
}
