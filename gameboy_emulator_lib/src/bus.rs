use crate::{rom::Rom, utils::bytes_to_word};

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&self, address: u16, byte: u8);
}

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

    pub fn load_cart(&mut self, cart: &Rom) {
        // Currently load the entire ROM
        // Will be changed later
        for addr in 0x0000..cart.data.len() - 1 {
            self.write(addr as u16, cart.data[addr]);
        }
    }
}
