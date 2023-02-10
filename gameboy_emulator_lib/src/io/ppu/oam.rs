use crate::bus::Memory;

pub struct OamEntry {
    y_pos: u8,
    x_pos: u8,
    tile_idx: u8,
    flags: u8
}

impl Memory for OamEntry {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, byte: u8) {
        todo!()
    }
}
