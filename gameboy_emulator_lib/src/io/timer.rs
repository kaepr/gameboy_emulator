use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::Memory,
    interrupt::{InterruptType, Interrupts},
    utils::BitPosCheck,
};

pub struct Timer {
    div: u16,
    tima: u8,
    tima_cycles: u16,
    tima_period: ClockFreq,
    tma: u8,
    tac: u8,
    interrupts: Rc<RefCell<Interrupts>>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
            }
            0xFF05 => {
                self.tima = byte;
            }
            0xFF06 => {
                self.tma = byte;
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
            tima_cycles: 0,
            tima_period: ClockFreq::C1024,
            tma: 0x00,
            tac: 0xF8,
            interrupts,
        }
    }

    pub fn tick(&mut self) {
        self.div = self.div.wrapping_add(1);

        let tac_enabled = self.tac.is_bit_set(2);

        let period = self.tima_freq();

        if period != self.tima_period {
            self.tima_period = period;
            self.tima_cycles = 0;
        } else if tac_enabled {
            self.tima_cycles += 1;
            let tima_period = self.tima_freq();

            if self.tima_cycles > (tima_period as u16) {
                let (res, carry) = self.tima.overflowing_add(1);
                self.tima_cycles = 0;

                if carry {
                    self.tima = self.tma;
                    self.interrupts
                        .borrow_mut()
                        .create_interrupt(InterruptType::TIMER);
                } else {
                    self.tima = res;
                }
            }
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
