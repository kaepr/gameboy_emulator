use crate::{
    bus::Memory,
    cpu::CPU,
    utils::{reset_bit, set_bit, word_to_bytes, BitPosCheck},
};

pub struct Interrupts {
    pub enable: u8,
    pub flag: u8,
}

pub trait Interruptable {
    fn create_interrupt(&mut self);
    fn reset_interrupt(&mut self);
}

impl Memory for Interrupts {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF0F => self.flag,
            0xFFFF => self.enable,
            _ => panic!("invalid interrupt address"),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0xFF0F => self.flag = byte,
            0xFFFF => self.enable = byte,
            _ => panic!("invalid interrupt address"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterruptType {
    VBLANK = 0,
    LCDSTAT = 1,
    TIMER = 2,
    SERIAL = 3,
    JOYPAD = 4,
}

impl Interrupts {
    const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
    const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;

    pub fn new() -> Self {
        Interrupts {
            enable: 0x00,
            flag: 0xE1,
        }
    }

    pub fn create_interrupt(&mut self, it_type: InterruptType) {
        self.flag = set_bit(self.flag, it_type as usize);
    }

    pub fn reset_interrupt(&mut self, it_type: InterruptType) {
        self.flag = reset_bit(self.flag, it_type as usize);
    }

    pub fn pending_interrupt(&self) -> bool {
        self.enable & self.flag > 0
    }

    pub fn interrupt_addr(it_type: InterruptType) -> u16 {
        match it_type {
            InterruptType::VBLANK => 0x0040,
            InterruptType::LCDSTAT => 0x0048,
            InterruptType::TIMER => 0x0050,
            InterruptType::SERIAL => 0x0058,
            InterruptType::JOYPAD => 0x0060,
        }
    }

    pub fn interrupt_type(enable: u8, flag: u8) -> InterruptType {
        if Self::check_flag(enable, flag, 0) {
            return InterruptType::VBLANK;
        }

        if Self::check_flag(enable, flag, 1) {
            return InterruptType::LCDSTAT;
        }

        if Self::check_flag(enable, flag, 2) {
            return InterruptType::TIMER;
        }

        if Self::check_flag(enable, flag, 3) {
            return InterruptType::SERIAL;
        }

        if Self::check_flag(enable, flag, 4) {
            return InterruptType::JOYPAD;
        }

        panic!("invalid interrupt registers");
    }

    fn check_flag(enable: u8, flag: u8, pos: usize) -> bool {
        enable.is_bit_set(pos) && flag.is_bit_set(pos)
    }
}
