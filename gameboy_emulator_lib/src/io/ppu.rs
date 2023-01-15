use std::{cell::RefCell, rc::Rc};

use crate::interrupt::Interrupts;

pub struct PPU {
    interrupts: Rc<RefCell<Interrupts>>,
}

impl PPU {
    pub fn new(interrupts: Rc<RefCell<Interrupts>>) -> Self {
        PPU { interrupts }
    }
}
