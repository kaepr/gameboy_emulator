use crate::utils::is_bit_set;

/// Lcd Control Register
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Lcdc {
    /// ### Bit 7
    /// Controls whether lcd is on and the ppu is active
    /// 0 -> full access to vram, oam etc
    /// Sreen disabled means another color ( the lowest shade )
    /// Re enabling this flag means that PPU will start to work
    /// but screen will stay blank during the first frame
    pub enable_lcd: bool,
    /// ### Window Tile Map Area Bit 6
    /// Which tilemap the window uses
    /// - 0 = 9800-9BFF
    /// - 1 = 9C00 - 9FFF
    pub window_tile_map_area: bool,
    /// ### Bit 5
    /// Decides whether the window is enabled or not
    pub window_enable: bool,
    /// ### Bit 4
    /// Controls which addressing mode that background and window use
    pub bg_tile_data_area: bool,
    /// ### Bit 3
    /// Which tilemap the background uses
    /// - 0 = 9800-9BFF
    /// - 1 = 9C00-9FFF
    pub bg_tile_map_area: bool,
    /// ### Bit 2
    /// Defines sprite size
    /// - 0 = 8x8
    /// - 1 = 8x16
    pub obj_size: bool,
    /// ### Bit 1
    /// Decides whether sprites are displayed or not
    pub obj_enable: bool,
    /// ### Bit 0
    /// 0 -> both background and window become blank
    /// ignores window enable bit
    /// only sprites displayed in such case ( if sprites are enabled that it is )
    pub bg_priority: bool,
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

/// Lcd Status Register
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stat {
    /// Bit 7
    unused: bool,
    /// Bit 6
    /// If this is enabled, and lyc = ly
    /// then interrupt occurs
    pub lyc_ly_eq_interrupt: bool,
    /// Bit 5
    pub oam_interrupt: bool,
    /// Bit 4
    pub vblank_interrupt: bool,
    /// Bit 3
    pub hblank_interrupt: bool,
    /// Bit 2
    /// Enabled if ly == lyc
    pub lyc_ly_eq_flag: bool,
    /// Bit 1-0
    /// Mode Flag
    mode_bit_1: bool,
    mode_bit_0: bool,
}

impl From<Stat> for u8 {
    fn from(stat: Stat) -> u8 {
        u8::from(stat.unused) << 7
            | u8::from(stat.lyc_ly_eq_interrupt) << 6
            | u8::from(stat.oam_interrupt) << 5
            | u8::from(stat.vblank_interrupt) << 4
            | u8::from(stat.hblank_interrupt) << 3
            | u8::from(stat.lyc_ly_eq_flag) << 2
            | u8::from(stat.mode_bit_1) << 1
            | u8::from(stat.mode_bit_0) << 0
    }
}

impl From<u8> for Stat {
    fn from(byte: u8) -> Stat {
        Stat {
            unused: is_bit_set(byte, 7),
            lyc_ly_eq_interrupt: is_bit_set(byte, 6),
            oam_interrupt: is_bit_set(byte, 5),
            vblank_interrupt: is_bit_set(byte, 4),
            hblank_interrupt: is_bit_set(byte, 3),
            lyc_ly_eq_flag: is_bit_set(byte, 2),
            mode_bit_1: is_bit_set(byte, 1),
            mode_bit_0: is_bit_set(byte, 0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    LcdTransfer = 3,
}

impl Stat {
    pub fn new(byte: u8) -> Self {
        byte.into()
    }

    pub fn set_lyc_ly_eq_flag(&mut self, flag: bool) {
        self.lyc_ly_eq_flag = flag;
    }

    pub fn get_mode(&self) -> Mode {
        match (self.mode_bit_1, self.mode_bit_0) {
            (true, true) => Mode::LcdTransfer,
            (true, false) => Mode::OamSearch,
            (false, true) => Mode::VBlank,
            (false, false) => Mode::HBlank,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        match mode {
            Mode::HBlank => {
                self.mode_bit_1 = false;
                self.mode_bit_0 = false;
            }
            Mode::VBlank => {
                self.mode_bit_1 = false;
                self.mode_bit_0 = true;
            }
            Mode::OamSearch => {
                self.mode_bit_1 = true;
                self.mode_bit_0 = false;
            }
            Mode::LcdTransfer => {
                self.mode_bit_1 = true;
                self.mode_bit_0 = true;
            }
        }
    }
}

/// Palette
/// Array of colors
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Palette {
    bit_7: bool,
    bit_6: bool,
    bit_5: bool,
    bit_4: bool,
    bit_3: bool,
    bit_2: bool,
    bit_1: bool,
    bit_0: bool,
}

impl From<Palette> for u8 {
    fn from(plt: Palette) -> u8 {
        u8::from(plt.bit_7) << 7
            | u8::from(plt.bit_6) << 6
            | u8::from(plt.bit_5) << 5
            | u8::from(plt.bit_4) << 4
            | u8::from(plt.bit_3) << 3
            | u8::from(plt.bit_2) << 2
            | u8::from(plt.bit_1) << 1
            | u8::from(plt.bit_0) << 0
    }
}

impl From<u8> for Palette {
    fn from(byte: u8) -> Palette {
        Palette {
            bit_7: is_bit_set(byte, 7),
            bit_6: is_bit_set(byte, 6),
            bit_5: is_bit_set(byte, 5),
            bit_4: is_bit_set(byte, 4),
            bit_3: is_bit_set(byte, 3),
            bit_2: is_bit_set(byte, 2),
            bit_1: is_bit_set(byte, 1),
            bit_0: is_bit_set(byte, 0),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
}

impl Color {
    pub fn get_color_index(low: bool, high: bool) -> usize {
        match (low, high) {
            (true, true) => 3,
            (true, false) => 2,
            (false, true) => 1,
            (false, false) => 0,
        }
    }
}

impl Palette {
    pub fn new(byte: u8) -> Self {
        byte.into()
    }

    pub fn get_color(&self, idx: usize) -> Color {
        match idx {
            0 => self.get_val(self.bit_1, self.bit_0),
            1 => self.get_val(self.bit_3, self.bit_2),
            2 => self.get_val(self.bit_5, self.bit_4),
            3 => self.get_val(self.bit_7, self.bit_6),
            _ => unreachable!("Index should only be 0..3"),
        }
    }

    fn get_val(&self, high: bool, low: bool) -> Color {
        match (high, low) {
            (true, true) => Color::C3,
            (true, false) => Color::C2,
            (false, true) => Color::C1,
            (false, false) => Color::C0,
        }
    }
}
