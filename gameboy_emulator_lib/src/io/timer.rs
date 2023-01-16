use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::Memory,
    interrupt::{InterruptType, Interruptable, Interrupts},
    utils::BitPosCheck,
};

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    prev_result: bool,
    reload_requested: bool,
    reloaded: bool,
    interrupts: Rc<RefCell<Interrupts>>,
}

pub enum ClockFreq {
    C1024 = 1024,
    C16 = 16,
    C64 = 64,
    C256 = 256,
}

impl Memory for Timer {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("invalid address passed"),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0xFF04 => {
                self.div = 0;
                self.update_timer_flags();
            }
            0xFF05 => {
                if !self.reloaded {
                    self.tima = byte;
                    self.reload_requested = false;
                }
            }
            0xFF06 => {
                self.tma = byte;
                if self.reloaded {
                    self.tima = byte;
                }
            }
            0xFF07 => self.tac = byte,
            _ => panic!("invalid address passed"),
        }
    }
}

impl Timer {
    pub fn new(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Timer {
            div: 0xAB,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
            prev_result: true,
            reload_requested: false,
            reloaded: false,
            interrupts,
        }
    }

    pub fn tick(&mut self) {
        if self.reloaded {
            self.reloaded = false;
        }

        if self.reload_requested {
            self.tima = self.tma;
            self.reload_requested = false;
            self.interrupts
                .borrow_mut()
                .create_interrupt(InterruptType::TIMER);

            self.reloaded = true;
        }

        self.div = self.div.wrapping_add(4);
        self.update_timer_flags();
    }

    fn update_timer_flags(&mut self) {
        let div_bit = self.div.is_bit_set(self.bit_pos().into());
        let tac_bit = self.tac.is_bit_set(2);
        let cur_result = div_bit & tac_bit;

        if self.prev_result && !cur_result {
            let (res, carry) = self.tima.overflowing_add(1);

            if carry {
                self.tima = 0;
                self.reload_requested = true;
            } else {
                self.tima = res;
            }
        }

        self.prev_result = cur_result;
    }

    fn bit_pos(&self) -> u16 {
        let pattern = self.tac & 0b00000011;
        match pattern {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => panic!("invalid bit pattern"),
        }
    }

    fn tima_freq(&self) -> ClockFreq {
        let pattern = self.tac & 0b00000011;
        match pattern {
            0 => ClockFreq::C1024,
            1 => ClockFreq::C16,
            2 => ClockFreq::C64,
            3 => ClockFreq::C256,
            _ => panic!("invalid bit pattern"),
        }
    }
}
