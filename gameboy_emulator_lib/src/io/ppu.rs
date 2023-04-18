use std::{cell::RefCell, rc::Rc};

pub mod oam;
pub mod registers;

use crate::{
    bus::{
        ranges::{OAM_COUNT, OAM_END, OAM_START, VRAM_END, VRAM_SIZE, VRAM_START},
        Memory,
    },
    interrupt::Interrupts,
};

use self::{
    oam::OamEntry,
    registers::{Lcdc, Palette, Stat},
};

pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    LcdTransfer = 3,
}

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    oam: [OamEntry; 40],
    lcdc: Lcdc,
    /// LCD Y Coordinate
    ly: u8,
    /// LY Compare
    lyc: u8,
    stat: Stat,
    /// Viewport positions
    scy: u8,
    scx: u8,
    /// Window positions
    wy: u8,
    wx: u8,
    dma: u8,
    // Background Palette
    bg_palette: Palette,
    /// Object Palette
    obj_palette_0: Palette,
    obj_palette_1: Palette,
    interrupts: Rc<RefCell<Interrupts>>,
}

#[inline(always)]
fn get_oam_idx(address: u16) -> (usize, usize) {
    let addr = address - OAM_START;
    ((addr / 4) as usize, (addr % 4) as usize)
}

impl Memory for PPU {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc.into(),
            0xFF41 => self.stat.into(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bg_palette.into(),
            0xFF48 => self.obj_palette_0.into(),
            0xFF49 => self.obj_palette_1.into(),
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            OAM_START..=OAM_END => {
                let (idx, field_idx) = get_oam_idx(address);
                self.oam[idx].get_field(field_idx)
            }
            VRAM_START..=VRAM_END => self.vram[(address - VRAM_START) as usize],
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0xFF40 => self.lcdc = Lcdc::new(byte),
            0xFF41 => self.stat = Stat::new(byte),
            0xFF42 => self.scy = byte,
            0xFF43 => self.scx = byte,
            0xFF44 => self.ly = byte,
            0xFF45 => self.lyc = byte,
            0xFF46 => {
                self.dma = byte;
                self.start_dma_transfer(byte);
            }
            0xFF47 => self.bg_palette = byte.into(),
            0xFF48 => self.obj_palette_0 = byte.into(),
            0xFF49 => self.obj_palette_1 = byte.into(),
            0xFF4A => self.wy = byte,
            0xFF4B => self.wx = byte,
            OAM_START..=OAM_END => {
                let (idx, field_idx) = get_oam_idx(address);
                self.oam[idx].set_field(byte, field_idx);
            }
            VRAM_START..=VRAM_END => self.vram[(address - VRAM_START) as usize] = byte,
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
            lcdc: 0x00.into(),
            ly: 0x00,
            lyc: 0x00,
            stat: 0x00.into(),
            scy: 0x00,
            scx: 0x00,
            wy: 0x00,
            wx: 0x00,
            dma: 0x00,
            bg_palette: 0x00.into(),
            obj_palette_0: 0x00.into(),
            obj_palette_1: 0x00.into(),
        }
    }

    pub fn tick(&mut self) {}

    fn start_dma_transfer(&mut self, byte: u8) {
        todo!("write dma transfer")
    }
}
