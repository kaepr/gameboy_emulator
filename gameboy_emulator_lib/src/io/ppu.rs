use std::{cell::RefCell, rc::Rc};

mod registers;

use crate::{
    bus::{
        ranges::{OAM_COUNT, OAM_END, OAM_SIZE, OAM_START, VRAM_SIZE},
        Memory,
    },
    interrupt::Interrupts,
};

use self::{oam::OamEntry, registers::Lcdc};

pub mod oam;

enum Mode {
    OamScan,
    VramScan,
    HBlank,
    VBlank,
}

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    oam: [OamEntry; 40],
    mode: Mode,
    // LCD Control
    lcdc: Lcdc,
    interrupts: Rc<RefCell<Interrupts>>,
}

impl Memory for PPU {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc.into(),
            OAM_START..=OAM_END => {
                let addr = address - OAM_START;
                let field_idx = (addr % 4) as usize;
                let idx = (addr / 4) as usize;
                self.oam[idx].get_field(field_idx)
            }
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0xFF40 => {
                self.lcdc = Lcdc::new(byte);
                // TODO: Write logic here
            }
            OAM_START..=OAM_END => {
                let addr = address - OAM_START;
                let field_idx = (addr % 4) as usize;
                let idx = (addr / 4) as usize;
                self.oam[idx].set_field(byte, field_idx);
            }
            _ => unreachable!(),
        }
    }
}

impl PPU {
    pub fn new(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        PPU {
            interrupts,
            oam: [OamEntry::new(); OAM_COUNT],
            vram: [0; VRAM_SIZE],
            mode: Mode::OamScan,
            lcdc: Lcdc::new(0x00),
        }
    }

    pub fn tick(&mut self) {}
}
