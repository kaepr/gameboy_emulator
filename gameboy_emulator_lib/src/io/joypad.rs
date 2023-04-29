use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::Memory,
    interrupt::{InterruptType, Interrupts},
};

pub enum JoypadInput {
    Start,
    Select,
    A,
    B,
    Up,
    Down,
    Left,
    Right,
}

/// Joypad
///
/// Bits 7 and 6 are not used.
///
/// - 1 -> not pressed
/// - 0 -> pressed
///
pub struct Joypad {
    /// Bit 5
    select_action: bool,
    /// Bit 4
    select_direction: bool,
    /// Down | Start
    bit_3: bool,
    /// Up | Select
    bit_2: bool,
    /// Left | B
    bit_1: bool,
    /// Right | A
    bit_0: bool,
    interrupts: Rc<RefCell<Interrupts>>,
}

impl Joypad {
    pub fn new(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Joypad {
            select_action: false,
            select_direction: false,
            bit_3: true,
            bit_2: true,
            bit_1: true,
            bit_0: true,
            interrupts,
        }
    }

    pub fn key_down(&mut self, key: JoypadInput) {
        self.set_joypad(false, key);
        self.create_interrupt();
    }

    pub fn key_up(&mut self, key: JoypadInput) {
        self.set_joypad(true, key);
    }

    fn set_joypad(&mut self, pressed: bool, key: JoypadInput) {
        if self.is_action_mode() {
            match key {
                JoypadInput::Start => self.bit_3 = pressed,
                JoypadInput::Select => self.bit_2 = pressed,
                JoypadInput::A => self.bit_0 = pressed,
                JoypadInput::B => self.bit_1 = pressed,
                _ => (),
            }
        }

        if self.is_direction_mode() {
            match key {
                JoypadInput::Up => self.bit_2 = pressed,
                JoypadInput::Down => self.bit_3 = pressed,
                JoypadInput::Left => self.bit_1 = pressed,
                JoypadInput::Right => self.bit_0 = pressed,
                _ => (),
            }
        }
    }

    fn enable_action(&mut self) {
        self.select_action = false;
    }

    fn enable_direction(&mut self) {
        self.select_direction = false;
    }

    fn disable_action(&mut self) {
        self.select_action = true;
    }

    fn disable_direction(&mut self) {
        self.select_direction = true;
    }

    /// Basically returns reverse of their actual bit
    fn get_state(&self) -> u8 {
        if self.is_action_mode() {
            return 0b0001_0000;
        }

        if self.is_direction_mode() {
            return 0b0010_0000;
        }

        0b0011_0000
    }

    fn get_input(&self) -> u8 {
        // if button pressed -> false
        // not pressed -> true

        u8::from(self.bit_3) << 3
            | u8::from(self.bit_2) << 2
            | u8::from(self.bit_1) << 1
            | u8::from(self.bit_0) << 0
    }

    fn get_joypad_input(&self) -> u8 {
        self.get_state() | self.get_input()
    }

    fn is_action_mode(&self) -> bool {
        !self.select_action
    }

    fn is_direction_mode(&self) -> bool {
        !self.select_direction
    }

    fn create_interrupt(&mut self) {
        self.interrupts
            .borrow_mut()
            .create_interrupt(InterruptType::JOYPAD);
    }
}

impl Memory for Joypad {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.get_joypad_input(),
            _ => panic!("Accessing invalid address"),
        }
    }

    fn write(&mut self, _address: u16, byte: u8) {
        match byte & 0b0011_0000 {
            0b0001_0000 => {
                self.enable_action();
                self.disable_direction();
            }
            // actions are disabled
            // direction is enabled
            0b0010_0000 => {
                self.enable_direction();
                self.disable_action();
            }
            _ => {
                self.disable_direction();
                self.disable_action();
            }
        }
    }
}
