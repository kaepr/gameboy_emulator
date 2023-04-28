use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::Memory,
    interrupt::Interrupts,
};

pub struct Serial {
    output: String,
    data: u8,
    control: u8,
    interrupts: Rc<RefCell<Interrupts>>,
}

impl Memory for Serial {
    fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => panic!("invalid address for serial"),
        }
    }

    fn write(&mut self, address: u16, byte: u8) {
        match address {
            0xFF01 => self.data = byte,
            0xFF02 => self.control = byte,
            _ => panic!("invalid address for serial"),
        }
    }
}

impl Serial {
    pub fn new(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        Serial {
            output: "".to_string(),
            data: 0x00,
            control: 0x7E,
            interrupts,
        }
    }

    pub fn print_serial_data(&mut self) {
        if self.control == 0x81 {
            self.output.push(self.data as char);
            self.control = 0;
            println!("{}", self.output);
        }
    }
}
