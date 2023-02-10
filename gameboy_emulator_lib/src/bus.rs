use std::{cell::RefCell, rc::Rc};

use crate::{
    cartridge::Cartridge,
    interrupt::Interrupts,
    io::{ppu::PPU, serial::Serial, timer::Timer},
};

use self::ranges::{
    CART_END, CART_START, INTERRUPT_ENABLE, LCD_END, LCD_START, OAM_END, OAM_START, SERIAL_END,
    SERIAL_START, TIMER_END, TIMER_START, VRAM_END, VRAM_START, WRAM_END, WRAM_START, WRAM_SIZE, HRAM_SIZE,
};

pub mod ranges;

pub struct Bus {
    pub cartridge: Cartridge,
    pub timer: Timer,
    pub serial: Serial,
    pub ppu: PPU,
    pub interrupts: Rc<RefCell<Interrupts>>,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    memory: [u8; 0x10000],
}

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}

impl Memory for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            CART_START..=CART_END => self.cartridge.read(address),
            WRAM_START..=WRAM_END => self.wram[(address - WRAM_START) as usize],
            SERIAL_START..=SERIAL_END => self.serial.read(address),
            TIMER_START..=TIMER_END => self.timer.read(address),
            VRAM_START..=VRAM_END | LCD_START..=LCD_END | OAM_START..=OAM_END => {
                self.ppu.read(address)
            }
            INTERRUPT_ENABLE | INTERRUPT_ENABLE => self.interrupts.borrow().read(address),
            _ => self.memory[address as usize],
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            CART_START..=CART_END => self.cartridge.write(address, byte),
            WRAM_START..=WRAM_END => self.wram[(address - WRAM_START) as usize] = byte,
            SERIAL_START..=SERIAL_END => self.serial.write(address, byte),
            TIMER_START..=TIMER_END => self.timer.write(address, byte),
            VRAM_START..=VRAM_END | LCD_START..=LCD_END | OAM_START..=OAM_END => {
                self.ppu.write(address, byte)
            }
            INTERRUPT_ENABLE | INTERRUPT_ENABLE => {
                self.interrupts.borrow_mut().write(address, byte)
            }
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
            timer: Timer::new(interrupts.clone()),
            serial: Serial::new(interrupts.clone()),
            ppu: PPU::new(interrupts.clone()),
            interrupts,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            memory,
        }
    }

    pub fn tick(&mut self) {
        self.timer.tick();
        self.ppu.tick();
    }
}
