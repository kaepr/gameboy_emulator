use crate::cartridge::Cartridge;

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&self, address: u16, byte: u8);
}

pub struct Bus {
    cartridge: Cartridge,
    memory: [u8; 0x10000],
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut memory = [0; 0x10000];
        memory[0xFF44] = 0x90; // used for blarrgs test
        Bus { cartridge, memory }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read(address),
            _ => self.memory[address as usize],
        }
    }

    pub fn write(&mut self, address: u16, byte: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write(address, byte),
            _ => self.memory[address as usize] = byte,
        }
    }
}
