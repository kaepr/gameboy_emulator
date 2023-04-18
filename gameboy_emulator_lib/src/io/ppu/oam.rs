use crate::bus::Memory;

#[derive(Copy, PartialEq, Debug, Clone)]
pub struct OamEntry {
    y_pos: u8,
    x_pos: u8,
    tile_idx: u8,
    flags: u8,
}

impl OamEntry {
    pub fn new() -> Self {
        OamEntry {
            y_pos: 0,
            x_pos: 0,
            tile_idx: 0,
            flags: 0,
        }
    }

    pub fn get_field(&self, field_pos: usize) -> u8 {
        match field_pos {
            0 => self.y_pos,
            1 => self.x_pos,
            2 => self.tile_idx,
            3 => self.flags,
            _ => panic!("Shouldn't happen"),
        }
    }

    pub fn set_field(&mut self, byte: u8, field_pos: usize) {
        match field_pos {
            0 => self.y_pos = byte,
            1 => self.x_pos = byte,
            2 => self.tile_idx = byte,
            3 => self.flags = byte,
            _ => panic!("Shouldn't happen"),
        }
    }
}
