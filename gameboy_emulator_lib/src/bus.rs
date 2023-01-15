use std::{cell::RefCell, rc::Rc};

use crate::{
    cartridge::Cartridge,
    interrupt::Interrupts,
    io::{ppu::PPU, serial::Serial, timer::Timer},
};

pub struct Bus {
    pub cartridge: Cartridge,
    pub timer: Timer,
    pub serial: Serial,
    pub ppu: PPU,
    pub interrupts: Rc<RefCell<Interrupts>>,
    memory: [u8; 0x10000],
}

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}

impl Memory for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read(address),
            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F | 0xFFFF => self.interrupts.borrow().read(address),
            _ => self.memory[address as usize],
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write(address, byte),
            0xFF01..=0xFF02 => self.serial.write(address, byte),
            0xFF04..=0xFF07 => self.timer.write(address, byte),
            0xFF0F | 0xFFFF => self.interrupts.borrow_mut().write(address, byte),
            _ => self.memory[address as usize] = byte,
        }
    }
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut memory = [0; 0x10000];

        // setting for blarrgs output
        memory[0xFF44] = 0x90;

        let interrupts = Rc::new(RefCell::new(Interrupts::new()));

        Bus {
            cartridge,
            memory,
            timer: Timer::new(interrupts.clone()),
            serial: Serial::new(interrupts.clone()),
            ppu: PPU::new(interrupts.clone()),
            interrupts,
        }
    }

    pub fn tick(&mut self) {
        self.timer.tick();
    }
}
