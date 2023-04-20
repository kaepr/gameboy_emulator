use std::{cell::RefCell, rc::Rc};

use crate::{
    cartridge::Cartridge,
    interrupt::Interrupts,
    io::{ppu::PPU, serial::Serial, timer::Timer},
};

use self::ranges::{
    CART_END, CART_START, HRAM_END, HRAM_SIZE, HRAM_START, INTERRUPT_ENABLE, INTERRUPT_FLAG,
    LCD_END, LCD_START, OAM_END, OAM_START, SERIAL_END, SERIAL_START, TIMER_END, TIMER_START,
    VRAM_END, VRAM_START, WRAM_END, WRAM_SIZE, WRAM_START,
};

pub mod ranges;

pub struct Bus {
    pub cartridge: Cartridge,
    pub timer: Timer,
    pub serial: Serial,
    pub ppu: PPU,
    pub interrupts: Rc<RefCell<Interrupts>>,
    start_dma_transfer: bool,
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
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize],
            INTERRUPT_ENABLE | INTERRUPT_FLAG => self.interrupts.borrow().read(address),
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
            HRAM_START..=HRAM_END => self.hram[(address - HRAM_START) as usize] = byte,
            INTERRUPT_ENABLE | INTERRUPT_FLAG => self.interrupts.borrow_mut().write(address, byte),
            _ => self.memory[address as usize] = byte,
        }
    }
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let memory = [0x00; 0x10000];
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
            start_dma_transfer: false,
        }
    }

    pub fn tick(&mut self) {
        self.timer.tick();
        self.ppu.tick();

        self.dma_transfer();
    }

    fn dma_transfer(&mut self) {
        // dma started inside PPU
        if self.ppu.dma_mode && !self.start_dma_transfer {
            self.start_dma_transfer = true;
            let mut src = self.ppu.dma as u16 * 0x100;
            let mut dest = 0xFE00;

            for _ in 0..160 {
                self.write(dest, self.read(src));
                dest += 1;
                src += 1;
            }
        } else if !self.ppu.dma_mode {
            self.start_dma_transfer = false;
        }
    }
}
