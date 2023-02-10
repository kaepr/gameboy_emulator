use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::{
        ranges::{OAM_SIZE, VRAM_SIZE},
        Memory,
    },
    interrupt::Interrupts,
};

pub mod oam;

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    interrupts: Rc<RefCell<Interrupts>>,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, byte: u8) {
        todo!()
    }
}

impl PPU {
    pub fn new(interrupts: Rc<RefCell<Interrupts>>) -> Self {

        PPU {
            interrupts,
            oam: [0; OAM_SIZE],
            vram: [0; VRAM_SIZE],
        }
    }

    pub fn tick(&mut self) {}
}
