use crate::{cartridge::Cartridge, utils::bytes_to_word};

pub struct Bus {
    memory: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: [0; 0x10000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr + 1);
        bytes_to_word(high, low)
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        self.write(addr, (value & 0x00FF) as u8);
        self.write(addr + 1, (value >> 8) as u8);
    }

    pub fn load_cart(&mut self, cart: &Cartridge) {
        // Currently load the entire ROM
        // Will be changed later
        for addr in 0x0000..cart.data.len() - 1 {
            self.write(addr as u16, cart.data[addr]);
        }
    }
}
