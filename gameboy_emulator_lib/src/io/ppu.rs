use std::{cell::RefCell, rc::Rc};

pub mod fetcher;
pub mod oam;
pub mod registers;

use crate::interrupt::InterruptType;
use crate::{
    bus::{
        ranges::{OAM_COUNT, OAM_END, OAM_START, VRAM_END, VRAM_SIZE, VRAM_START},
        Memory,
    },
    interrupt::Interrupts,
};

use self::fetcher::Pixel;
use self::registers::Color;
use self::{
    oam::OamEntry,
    registers::{Lcdc, Mode, Palette, Stat},
};

pub const VBLANK_LINE_LIMIT: u8 = 144;
pub const MAX_LINE_LIMIT: u8 = 154;

pub const OAM_TICK_LIMIT: u64 = 80;
pub const LCD_TRANSFER_TICK_LIMIT: u64 = 172;
pub const HBLANK_TICK_LIMIT: u64 = 456;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

/// PPU
/// https://gbdev.io/pandocs/Rendering.html
/// PPU manipulates Tiles, which are 8x8 squares
/// Each Tile assigns a color id to each pixel ( 0 - 3 )
/// This color changes depending on whether its in background or window
/// or if its a sprite tile. It also uses a palette
///
/// Gameboy has three layers
/// - Background
/// - Window
/// - Sprite
///
/// ### Background
/// It is composed of a tilemap. Tilemap contains references to tiles. Can be scrolled using scx, scy registers.
///
/// ### Window
/// Similar to background. No transparency, always a rectangle. Only top left pixel position can be controlled.
/// Used as a fixed status bar.
///
/// ### Sprites ( Objects )
/// Used for objects that move separate from the background. NPCs, players etc.
/// 8x8 or 8x16 ( depending on the flag ), can be rendered anywhere.
pub struct PPU {
    cycles: u64,
    ticks: u64,
    machine_cycles: u64,
    pub dma_mode: bool,
    pub dma_cycles: u64,
    /// Tile data stored inside vram. Each tile takes 16 bytes of data.
    /// Thus total fo 384 different tiles. ( 128 of those are shared between sprites and background ).
    /// Each tile contains 8x8 pixels and color depth of 4. Each pixel gives has a color id
    ///
    /// Vram also contains 2 32x32 tilemaps
    /// Any of these tilemaps can be used to display background or the window
    /// Tilemap contains 1 byte index of the tile to display
    /// byte index + offset method to be used gives the tile to use from vram
    pub vram: [u8; VRAM_SIZE],
    /// Sprites
    /// Sprite taken from Vram only. Only 10 can be displayed per line
    /// Only 10 sprites can be displayed per scan line ( due to some hardware limitation )
    pub oam: [OamEntry; 40],
    active_sprites: Vec<OamEntry>,
    lcdc: Lcdc,
    /// LCD Y Coordinate
    /// Indicates current horizontal line to be drawn
    /// Values 0 - 143 are shown in the display,
    /// 144 - 153 indicate VBlank period
    ly: u8,
    /// LY Compare
    /// Used for comparing with ly register
    /// Triggers interrupt
    lyc: u8,
    stat: Stat,
    /// Background positions
    /// Used to scroll the background. Specifices the origin of the 160x144 (width x height) area
    /// Visible area of the background wraps around the background map
    scy: u8,
    scx: u8,
    /// Window positions
    /// Used to change the window positions. It is otherwise non scrollable.
    wy: u8,
    wx: u8,
    pub dma: u8,
    /// Background Palette
    bg_palette: Palette,
    /// Object Palette
    obj_palette_0: Palette,
    obj_palette_1: Palette,
    interrupts: Rc<RefCell<Interrupts>>,
    pub buffer: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT],
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
                if self.dma_mode {
                    return 0xFF;
                }
                let (idx, field_idx) = get_oam_idx(address);
                self.oam[idx].get_field(field_idx)
            }
            VRAM_START..=VRAM_END => {
                if !self.lcdc.is_lcd_enabled() {
                    return 0xFF;
                }

                if self.stat.get_mode() == Mode::LcdTransfer {
                    return 0xFF;
                }

                self.vram[(address - VRAM_START) as usize]
            }
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
        // blarrgs' test -> 0x94
        let ly: u8 = 0x00;

        PPU {
            cycles: 0,
            ticks: 0,
            interrupts,
            oam: [OamEntry::new(); OAM_COUNT],
            vram: [0; VRAM_SIZE],
            lcdc: 0x91.into(),
            ly,
            lyc: 0x00,
            stat: 0x85.into(),
            scy: 0x00,
            scx: 0x00,
            wy: 0x00,
            wx: 0x00,
            dma: 0xFF,
            bg_palette: 0xFC.into(),
            obj_palette_0: 0x00.into(),
            obj_palette_1: 0x00.into(),
            machine_cycles: 0,
            dma_mode: false,
            dma_cycles: 0,
            buffer: [Pixel::default(); SCREEN_WIDTH * SCREEN_HEIGHT],
            active_sprites: vec![],
        }
    }

    fn machine_cycle(&mut self) {
        if self.dma_mode {
            self.dma_cycles += 1;
            self.machine_cycles += 1;

            if self.dma_cycles >= 160 {
                self.dma_mode = false;
            }

            return;
        }

        self.machine_cycles += 1;
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
        self.ticks += 1;

        if self.cycles == 4 {
            self.machine_cycle();
            self.cycles = 0;
        }

        let mode = self.stat.get_mode();

        match mode {
            Mode::HBlank => self.hblank_mode(),
            Mode::VBlank => self.vblank_mode(),
            Mode::OamSearch => self.oam_search_mode(),
            Mode::LcdTransfer => self.lcd_transfer_mode(),
        }
    }

    fn inc_ly(&mut self) {
        self.ly += 1;

        if self.ly == self.lyc {
            self.stat.set_lyc_ly_eq_flag(true);

            if self.stat.lyc_ly_eq_interrupt {
                self.interrupts
                    .borrow_mut()
                    .create_interrupt(InterruptType::LCDSTAT);
            }
        } else {
            self.stat.set_lyc_ly_eq_flag(false);
        }
    }

    fn reset_ly(&mut self) {
        self.ly = 0;
    }

    fn hblank_mode(&mut self) {
        if self.ticks >= HBLANK_TICK_LIMIT {
            // finished one line
            self.inc_ly();

            if self.ly >= VBLANK_LINE_LIMIT {
                // means 1 frame has finished processing
                self.stat.set_mode(Mode::VBlank);

                self.interrupts
                    .borrow_mut()
                    .create_interrupt(InterruptType::VBLANK);

                if self.stat.vblank_interrupt {
                    self.interrupts
                        .borrow_mut()
                        .create_interrupt(InterruptType::LCDSTAT);
                }
            } else {
                self.stat.set_mode(Mode::OamSearch);
            }

            self.ticks -= HBLANK_TICK_LIMIT;
        }
    }

    fn vblank_mode(&mut self) {
        if self.ticks >= HBLANK_TICK_LIMIT {
            self.inc_ly();

            if self.ly >= MAX_LINE_LIMIT {
                // all 153 lines have finished
                // move to next frame
                self.stat.set_mode(Mode::OamSearch);

                if self.stat.oam_interrupt {
                    self.interrupts
                        .borrow_mut()
                        .create_interrupt(InterruptType::LCDSTAT);
                }
                self.reset_ly();
            }

            self.ticks -= HBLANK_TICK_LIMIT;
        }
    }

    fn clear_active_sprites(&mut self) {
        self.active_sprites.clear();
    }

    fn load_active_sprites(&mut self) {
        self.active_sprites = self
            .oam
            .into_iter()
            .filter(|sprite| {
                let height = match self.lcdc.obj_size {
                    true => 16,
                    false => 8,
                };

                let cur_line = self.ly as i16;
                let y_pos = (sprite.y_pos as i16) - 16;

                cur_line >= y_pos && cur_line < y_pos + height
            })
            .take(10)
            .collect();
    }

    fn oam_search_mode(&mut self) {
        if self.ticks >= OAM_TICK_LIMIT as u64 {
            self.clear_active_sprites();
            self.load_active_sprites();
            self.stat.set_mode(Mode::LcdTransfer);
        }
    }

    fn render_line_to_buffer(&mut self) {
        // render line
        self.render_background_line();

        // render sprite line
        self.render_sprite_line();
    }

    fn lcd_transfer_mode(&mut self) {
        if self.ticks >= (LCD_TRANSFER_TICK_LIMIT + OAM_TICK_LIMIT) {
            self.render_line_to_buffer();

            self.stat.set_mode(Mode::HBlank);
            if self.stat.hblank_interrupt {
                self.interrupts
                    .borrow_mut()
                    .create_interrupt(InterruptType::LCDSTAT);
            }
        }
    }

    fn start_dma_transfer(&mut self, byte: u8) {
        self.dma_mode = true;
        self.dma = byte;
        self.dma_cycles = 0;
    }

    fn render_background_line(&self) {
        let bg_map_addr = match self.lcdc.bg_tile_map_area {
            true => 0x9C00 - VRAM_START,
            false => 0x9800 - VRAM_START,
        };

        let window_map_addr = match self.lcdc.window_tile_map_area {
            true => 0x9C00 - VRAM_START,
            false => 0x9800 - VRAM_START,
        };

        for x_pixel in 0..160 {}
    }

    fn render_sprite_line(&self) {}
}
