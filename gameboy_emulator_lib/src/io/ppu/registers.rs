use crate::utils::is_bit_set;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Lcdc {
    /// Bit 7
    enable_lcd: bool,
    /// Window Tile Map Area Bit 6
    /// 0 = 9800-9BFF , 1 = 9C00 - 9FFF
    window_tile_map_area: bool,
    /// Bit 5
    window_enable: bool,
    /// Bit 4
    /// 0 = 8800-97FF, 1 = 8000-8FFF
    bg_tile_data_area: bool,
    /// Bit 3
    /// 0 = 9800-9BFF, 1 = 9C00-9FFF
    bg_tile_map_area: bool,
    /// Bit 2
    /// 0 = 8x8, 1 = 8x16
    obj_size: bool,
    /// Bit 1
    obj_enable: bool,
    /// Bit 0
    bg_priority: bool,
}

impl From<Lcdc> for u8 {
    fn from(lcdc: Lcdc) -> u8 {
        u8::from(lcdc.enable_lcd) << 7
            | u8::from(lcdc.window_tile_map_area) << 6
            | u8::from(lcdc.window_enable) << 5
            | u8::from(lcdc.bg_tile_data_area) << 4
            | u8::from(lcdc.bg_tile_map_area) << 3
            | u8::from(lcdc.obj_size) << 2
            | u8::from(lcdc.obj_enable) << 1
            | u8::from(lcdc.bg_priority) << 0
    }
}

impl From<u8> for Lcdc {
    fn from(byte: u8) -> Lcdc {
        Lcdc {
            enable_lcd: is_bit_set(byte, 7),
            window_tile_map_area: is_bit_set(byte, 6),
            window_enable: is_bit_set(byte, 5),
            bg_tile_data_area: is_bit_set(byte, 4),
            bg_tile_map_area: is_bit_set(byte, 3),
            obj_size: is_bit_set(byte, 2),
            obj_enable: is_bit_set(byte, 1),
            bg_priority: is_bit_set(byte, 0),
        }
    }
}

impl Lcdc {
    pub fn new(byte: u8) -> Self {
        byte.into()
    }

    pub fn is_lcd_enabled(&self) -> bool {
        self.enable_lcd
    }
}
