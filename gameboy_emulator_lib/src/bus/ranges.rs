pub const CART_START: u16 = 0x0000;
pub const CART_END: u16 = 0x7FFF;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

pub const EXTERNAL_START: u16 = 0xA000;
pub const EXTERNAL_END: u16 = 0xBFFF;
pub const EXTERNAL_SIZE: usize = (EXTERNAL_END - EXTERNAL_START + 1) as usize;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;
pub const WRAM_SIZE: usize = (WRAM_END - WRAM_START + 1) as usize;

pub const ECHO_START: u16 = 0xE000;
pub const ECHO_END: u16 = 0xFDFF;
pub const ECHO_SIZE: usize = (ECHO_END - ECHO_START + 1) as usize;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: usize = (OAM_END - OAM_START + 1) as usize;

pub const JOYPAD: u16 = 0xFF00;

pub const SERIAL_START: u16 = 0xFF01;
pub const SERIAL_END : u16 = 0xFF02;

pub const TIMER_START: u16 = 0xFF04;
pub const TIMER_END : u16 = 0xFF07;

pub const LCD_START: u16 = 0xFF40;
pub const LCD_END : u16 = 0xFF4B;
pub const LCD_SIZE: usize = (LCD_END - LCD_START + 1) as usize;

pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFE;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;

pub const INTERRUPT_ENABLE: u16 = 0xFFFF;
pub const INTERRUPT_FLAG: u16 = 0xFF0F;
