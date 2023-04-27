use crate::utils::BitPosCheck;

/// ### OamEntry
/// Data for each individual sprite
#[derive(Copy, PartialEq, Debug, Clone)]
pub struct OamEntry {
    /// Sprite's vertical position on screen + 16
    ///
    /// y_pos = 0 hides the the sprite
    pub y_pos: u8,
    /// Sprite's horizontal position on screen + 8
    ///
    /// x_pos = 0 hides the the sprite
    pub x_pos: u8,
    /// tile idx which is used to fetch tile data from vram
    /// In 8x8 mode just one fetch is enough
    ///
    /// In 8x16 mode, every two 2 tiles form a sprite
    /// byte specifies the top index of the sprite
    /// LSB of the idx is ignored.
    pub tile_idx: u8,
    /// - bit 7: background and window over object ( 0 = no, 1 = yes )
    /// - bit 6: y flip
    /// - bit 5: x flip
    /// - bit 4: palette number
    /// - bit 3 - bit 0: unused for original gameboy
    pub flags: u8,
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

    pub fn bg_priority(&self) -> bool {
        self.flags.is_bit_set(7)
    }

    pub fn y_flipped(&self) -> bool {
        self.flags.is_bit_set(6)
    }

    pub fn x_flipped(&self) -> bool {
        self.flags.is_bit_set(5)
    }

    pub fn get_palette_number(&self) -> usize {
        match self.flags.is_bit_set(4) {
            true => 1,
            false => 0,
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
