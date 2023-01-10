use crate::{cartridge::Cartridge, io::timer::Timer};

pub struct Bus {
    pub cartridge: Cartridge,
    pub timer: Timer,
    pub memory: [u8; 0x10000],
}

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}

impl Memory for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),
            _ => self.memory[address as usize],
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write(address, byte),
            0xFF04..=0xFF07 => self.timer.write(address, byte),
            _ => self.memory[address as usize] = byte,
        }
    }
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut memory = [0; 0x10000];
        memory[0xFF44] = 0x90; // used for blarrgs test
        Bus {
            cartridge,
            memory,
            timer: Timer::new(),
        }
    }
}
