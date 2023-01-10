use crate::{
    bus::Memory,
    utils::{is_bit_set, is_bit_set_16},
};

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

pub enum ClockFreq {
    C1024,
    C16,
    C64,
    C256,
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
            0xFF05 => todo!(),
            0xFF06 => todo!(),
            0xFF07 => todo!(),
            _ => panic!("invalid address passed"),
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0xAB,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
        }
    }

    pub fn tick(&mut self) {
        self.div = self.div.wrapping_add(4);

        let div_bit = is_bit_set_16(self.div, self.bit_pos().into());
        let timer_enabled_bit = self.get_timer_enabled_bit();

        let res = div_bit && timer_enabled_bit;
    }

    fn get_timer_enabled_bit(&self) -> bool {
        is_bit_set(self.tac, 2)
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
